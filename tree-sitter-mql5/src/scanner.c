#include "tree_sitter/alloc.h"
#include "tree_sitter/parser.h"

#include <assert.h>
#include <string.h>
#include <wctype.h>

enum TokenType { RAW_STRING_DELIMITER, RAW_STRING_CONTENT, COLOR_LITERAL };

/// The spec limits delimiters to 16 chars
#define MAX_DELIMITER_LENGTH 16

typedef struct {
    uint8_t delimiter_length;
    wchar_t delimiter[MAX_DELIMITER_LENGTH];
} Scanner;

static inline void advance(TSLexer *lexer) { lexer->advance(lexer, false); }

static inline void reset(Scanner *scanner) {
    scanner->delimiter_length = 0;
    memset(scanner->delimiter, 0, sizeof scanner->delimiter);
}

/// Scan the raw string delimiter in R"delimiter(content)delimiter"
static bool scan_raw_string_delimiter(Scanner *scanner, TSLexer *lexer) {
    if (scanner->delimiter_length > 0) {
        // Closing delimiter: must exactly match the opening delimiter.
        // We already checked this when scanning content, but this is how we
        // know when to stop. We can't stop at ", because R"""hello""" is valid.
        for (int i = 0; i < scanner->delimiter_length; ++i) {
            if (lexer->lookahead != scanner->delimiter[i]) {
                return false;
            }
            advance(lexer);
        }
        reset(scanner);
        return true;
    }

    // Opening delimiter: record the d-char-sequence up to (.
    // d-char is any basic character except parens, backslashes, and spaces.
    for (;;) {
        if (scanner->delimiter_length >= MAX_DELIMITER_LENGTH || lexer->eof(lexer) || lexer->lookahead == '\\' ||
            iswspace(lexer->lookahead)) {
            return false;
        }
        if (lexer->lookahead == '(') {
            // Rather than create a token for an empty delimiter, we fail and
            // let the grammar fall back to a delimiter-less rule.
            return scanner->delimiter_length > 0;
        }
        scanner->delimiter[scanner->delimiter_length++] = lexer->lookahead;
        advance(lexer);
    }
}

/// Scan the raw string content in R"delimiter(content)delimiter"
static bool scan_raw_string_content(Scanner *scanner, TSLexer *lexer) {
    // The progress made through the delimiter since the last ')'.
    // The delimiter may not contain ')' so a single counter suffices.
    for (int delimiter_index = -1;;) {
        // If we hit EOF, consider the content to terminate there.
        // This forms an incomplete raw_string_literal, and models the code
        // well.
        if (lexer->eof(lexer)) {
            lexer->mark_end(lexer);
            return true;
        }

        if (delimiter_index >= 0) {
            if (delimiter_index == scanner->delimiter_length) {
                if (lexer->lookahead == '"') {
                    return true;
                }
                delimiter_index = -1;
            } else {
                if (lexer->lookahead == scanner->delimiter[delimiter_index]) {
                    delimiter_index += 1;
                } else {
                    delimiter_index = -1;
                }
            }
        }

        if (delimiter_index == -1 && lexer->lookahead == ')') {
            // The content doesn't include the )delimiter" part.
            // We must still scan through it, but exclude it from the token.
            lexer->mark_end(lexer);
            delimiter_index = 0;
        }

        advance(lexer);
    }
}

/// Scan MQL5 color literal: C'r,g,b' where r,g,b are 0-255
/// Matches: C'255,0,128' or C'30,140,50'
static bool scan_color_literal(TSLexer *lexer) {
    // Lexer should be at 'C' (whitespace already skipped by caller)
    if (lexer->lookahead != 'C') return false;
    advance(lexer);

    // Must be immediately followed by single quote (no space)
    if (lexer->lookahead != '\'') return false;
    advance(lexer);

    // Scan digits, commas (expect r,g,b pattern)
    int comma_count = 0;
    bool has_digit = false;
    for (;;) {
        if (lexer->eof(lexer)) return false;
        if (lexer->lookahead >= '0' && lexer->lookahead <= '9') {
            has_digit = true;
            advance(lexer);
        } else if (lexer->lookahead == ',') {
            if (!has_digit) return false;
            comma_count++;
            if (comma_count > 2) return false; // max 3 components
            has_digit = false;
            advance(lexer);
        } else if (lexer->lookahead == '\'') {
            if (!has_digit || comma_count != 2) return false; // must have exactly r,g,b
            advance(lexer);
            return true;
        } else {
            return false;
        }
    }
}

void *tree_sitter_mql5_external_scanner_create() {
    Scanner *scanner = (Scanner *)ts_calloc(1, sizeof(Scanner));
    memset(scanner, 0, sizeof(Scanner));
    return scanner;
}

bool tree_sitter_mql5_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) {
    Scanner *scanner = (Scanner *)payload;

    if (valid_symbols[RAW_STRING_DELIMITER] && valid_symbols[RAW_STRING_CONTENT]) {
        // we're in error recovery
        return false;
    }

    // MQL5 color literals: C'r,g,b' — must check before identifiers consume 'C'
    if (valid_symbols[COLOR_LITERAL]) {
        // Skip whitespace
        while (lexer->lookahead == ' ' || lexer->lookahead == '\t' ||
               lexer->lookahead == '\n' || lexer->lookahead == '\r') {
            lexer->advance(lexer, true);
        }
        if (lexer->lookahead == 'C') {
            lexer->result_symbol = COLOR_LITERAL;
            bool result = scan_color_literal(lexer);
            if (result) return true;
        }
    }

    // No skipping leading whitespace: raw-string grammar is space-sensitive.
    if (valid_symbols[RAW_STRING_DELIMITER]) {
        lexer->result_symbol = RAW_STRING_DELIMITER;
        return scan_raw_string_delimiter(scanner, lexer);
    }

    if (valid_symbols[RAW_STRING_CONTENT]) {
        lexer->result_symbol = RAW_STRING_CONTENT;
        return scan_raw_string_content(scanner, lexer);
    }

    return false;
}

unsigned tree_sitter_mql5_external_scanner_serialize(void *payload, char *buffer) {
    static_assert(MAX_DELIMITER_LENGTH * sizeof(wchar_t) < TREE_SITTER_SERIALIZATION_BUFFER_SIZE,
                  "Serialized delimiter is too long!");

    Scanner *scanner = (Scanner *)payload;
    size_t size = scanner->delimiter_length * sizeof(wchar_t);
    memcpy(buffer, scanner->delimiter, size);
    return (unsigned)size;
}

void tree_sitter_mql5_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) {
    assert(length % sizeof(wchar_t) == 0 && "Can't decode serialized delimiter!");

    Scanner *scanner = (Scanner *)payload;
    scanner->delimiter_length = length / sizeof(wchar_t);
    if (length > 0) {
        memcpy(&scanner->delimiter[0], buffer, length);
    }
}

void tree_sitter_mql5_external_scanner_destroy(void *payload) {
    Scanner *scanner = (Scanner *)payload;
    ts_free(scanner);
}
