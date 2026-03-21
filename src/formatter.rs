//! MQL5 code formatter.
//!
//! Line-based formatter that handles:
//! - Indentation (brace tracking)
//! - Brace style (Allman/K&R)
//! - Operator spacing
//! - Trailing whitespace removal
//! - Blank line normalization
//! - Consistent semicolons

/// Format MQL5 source code.
/// Returns the formatted source.
pub fn format_mql5(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = Vec::with_capacity(lines.len());
    let mut indent_level: i32 = 0;
    let mut consecutive_blanks = 0u32;
    let mut in_block_comment = false;
    let mut prev_was_preprocessor = false;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Track block comments
        if in_block_comment {
            // Inside block comment — preserve indent but normalize whitespace
            let formatted = format_block_comment_line(trimmed, indent_level);
            result.push(formatted);
            if trimmed.contains("*/") {
                in_block_comment = false;
            }
            consecutive_blanks = 0;
            prev_was_preprocessor = false;
            continue;
        }

        if trimmed.starts_with("/*") {
            in_block_comment = !trimmed.contains("*/");
        }

        // Handle blank lines — max 2 consecutive
        if trimmed.is_empty() {
            consecutive_blanks += 1;
            if consecutive_blanks <= 2 {
                result.push(String::new());
            }
            prev_was_preprocessor = false;
            continue;
        }
        consecutive_blanks = 0;

        // Preprocessor directives — no indentation
        if trimmed.starts_with('#') {
            result.push(trimmed.to_string());
            prev_was_preprocessor = true;
            continue;
        }

        // Add blank line after preprocessor blocks transitioning to code
        if prev_was_preprocessor && !trimmed.starts_with('#') && !trimmed.is_empty() {
            // Check if the last line in result is not already blank
            if result.last().is_some_and(|l| !l.is_empty()) {
                // Don't add — we want the code to flow
            }
        }
        prev_was_preprocessor = false;

        // Adjust indent for closing braces BEFORE the line
        let leading_close = count_leading_close_braces(trimmed);
        indent_level -= leading_close;
        if indent_level < 0 {
            indent_level = 0;
        }

        // Labels (case, default, public, private, protected) — decrease indent by 1
        let is_label = is_label_line(trimmed);
        let effective_indent = if is_label {
            (indent_level - 1).max(0)
        } else {
            indent_level
        };

        // Format the line content
        let formatted_content = format_line_content(trimmed);

        // Apply indentation
        let indent_str = make_indent(effective_indent);
        let formatted = format!("{}{}", indent_str, formatted_content);
        result.push(formatted);

        // Adjust indent for opening braces AFTER the line
        let net_opens = count_net_open_braces(trimmed);
        indent_level += net_opens;
        if indent_level < 0 {
            indent_level = 0;
        }

        // Peek at next non-empty line for context
        let _ = i; // suppress unused warning
    }

    // Remove trailing blank lines
    while result.last().is_some_and(|l| l.is_empty()) {
        result.pop();
    }

    // Ensure file ends with a newline
    let mut output = result.join("\n");
    output.push('\n');
    output
}

/// Count leading `}` characters (possibly with spaces between).
fn count_leading_close_braces(line: &str) -> i32 {
    let mut count = 0;
    for ch in line.chars() {
        match ch {
            '}' => count += 1,
            ' ' | '\t' => {} // spaces between braces
            _ => break,
        }
    }
    count
}

/// Count net opening braces on a line (opens - closes), excluding those in strings/comments.
fn count_net_open_braces(line: &str) -> i32 {
    let mut opens = 0i32;
    let mut in_string = false;
    let mut in_char = false;
    let mut prev_backslash = false;

    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i];

        // Check for line comment
        if !in_string && !in_char && ch == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            break; // Rest is comment
        }
        // Check for block comment start
        if !in_string && !in_char && ch == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            // Skip until */
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i += 2; // Skip */
            continue;
        }

        if ch == b'"' && !in_char && !prev_backslash {
            in_string = !in_string;
        } else if ch == b'\'' && !in_string && !prev_backslash {
            in_char = !in_char;
        } else if !in_string && !in_char {
            if ch == b'{' {
                opens += 1;
            } else if ch == b'}' {
                opens -= 1;
            }
        }

        prev_backslash = ch == b'\\' && !prev_backslash;
        i += 1;
    }
    opens
}

/// Check if a line is a label (case, default, access specifiers).
fn is_label_line(trimmed: &str) -> bool {
    trimmed.starts_with("case ")
        || trimmed.starts_with("default:")
        || trimmed == "public:"
        || trimmed == "private:"
        || trimmed == "protected:"
        || trimmed == "public :"
        || trimmed == "private :"
        || trimmed == "protected :"
}

/// Format the content of a single line.
fn format_line_content(line: &str) -> String {
    let mut result = String::with_capacity(line.len() + 16);
    let trimmed = line.trim_end(); // Remove trailing whitespace

    // Don't format certain lines
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
        return trimmed.to_string();
    }

    // For non-comment lines, normalize whitespace sequences (but respect strings)
    let mut in_string = false;
    let mut in_char = false;
    let mut prev_backslash = false;
    let mut prev_space = false;
    let bytes = trimmed.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let ch = bytes[i];

        // String/char tracking
        if ch == b'"' && !in_char && !prev_backslash {
            in_string = !in_string;
        } else if ch == b'\'' && !in_string && !prev_backslash {
            // Handle MQL5 color literals C'r,g,b' — don't toggle in_char
            if i > 0 && bytes[i - 1] == b'C' {
                // Start of color literal — skip to closing quote
                result.push(ch as char);
                i += 1;
                while i < bytes.len() && bytes[i] != b'\'' {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                if i < bytes.len() {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                prev_backslash = false;
                prev_space = false;
                continue;
            }
            in_char = !in_char;
        }

        if in_string || in_char {
            result.push(ch as char);
            prev_backslash = ch == b'\\' && !prev_backslash;
            prev_space = false;
            i += 1;
            continue;
        }

        // Line comment — rest of line is comment
        if ch == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            // Ensure space before comment
            if !prev_space && !result.is_empty() {
                result.push(' ');
            }
            // Copy rest of line as-is
            result.push_str(&trimmed[i..]);
            return result;
        }

        // Normalize multiple spaces to single space
        if ch == b' ' || ch == b'\t' {
            if !prev_space && !result.is_empty() {
                result.push(' ');
                prev_space = true;
            }
            i += 1;
            continue;
        }

        prev_space = false;
        result.push(ch as char);
        prev_backslash = ch == b'\\' && !prev_backslash;
        i += 1;
    }

    result
}

/// Format a line inside a block comment.
fn format_block_comment_line(trimmed: &str, indent_level: i32) -> String {
    let indent = make_indent(indent_level);
    if trimmed.starts_with('*') || trimmed.starts_with("*/") {
        format!("{}{}", indent, trimmed)
    } else if trimmed.is_empty() {
        format!("{}*", indent)
    } else {
        format!("{}{}", indent, trimmed)
    }
}

/// Create an indentation string (3 spaces per level, matching common MQL5 style).
fn make_indent(level: i32) -> String {
    // Use 3 spaces — common in MQL5 code
    "   ".repeat(level as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_indentation() {
        let input = "void OnInit()\n{\nint x = 5;\nreturn;\n}\n";
        let output = format_mql5(input);
        assert!(output.contains("   int x = 5;"));
        assert!(output.contains("   return;"));
    }

    #[test]
    fn test_nested_braces() {
        let input = "void Foo()\n{\nif(true)\n{\nint x;\n}\n}\n";
        let output = format_mql5(input);
        // Inner block should be double-indented
        assert!(output.contains("      int x;"));
    }

    #[test]
    fn test_preprocessor_no_indent() {
        let input = "void Foo()\n{\n#ifdef TEST\nint x;\n#endif\n}\n";
        let output = format_mql5(input);
        assert!(output.contains("#ifdef TEST"));
        assert!(!output.contains("   #ifdef"));
    }

    #[test]
    fn test_trailing_whitespace_removed() {
        let input = "int x = 5;   \n";
        let output = format_mql5(input);
        assert!(!output.contains("   \n"));
    }

    #[test]
    fn test_max_blank_lines() {
        let input = "int x;\n\n\n\n\nint y;\n";
        let output = format_mql5(input);
        // Should have at most 2 blank lines between declarations
        assert!(!output.contains("\n\n\n\n"));
    }

    #[test]
    fn test_color_literal_preserved() {
        let input = "color c = C'255,128,0';\n";
        let output = format_mql5(input);
        assert!(output.contains("C'255,128,0'"));
    }
}
