# MQL5 Language Server — Approach B Spec & Status

**Project**: Native MQL5 Language Server in Rust
**Binary**: `mql5-lsp` (6.9 MB, arm64 macOS)
**Last updated**: 2026-03-20

---

## Overview

A language server for MetaQuotes Language 5 (MQL5), built from scratch in Rust using the LSP protocol. Provides IDE features (autocomplete, go-to-definition, hover, rename, etc.) for MQL5 development in any LSP-capable editor (Zed, VS Code, Neovim, etc.).

MQL5 is syntactically similar to C++ but has numerous incompatible constructs (dot access on pointers, dynamic array parameters, color literals, string concatenation with `+`, etc.). This LSP uses a tree-sitter grammar derived from tree-sitter-cpp as its parser, with MQL5-specific adjustments handled in Rust.

---

## Tech Stack

| Component | Version / Crate |
|---|---|
| LSP framework | `tower-lsp` 0.20 |
| Parser | `tree-sitter` 0.25 via `tree-sitter-mql5` (C++ based) |
| Async runtime | `tokio` |
| Text buffers | `ropey` (incremental sync) |
| Language | Rust |

---

## Codebase

**5312 lines of Rust across 6 source files.**

| File | Responsibility |
|---|---|
| `main.rs` | LSP server, all request handlers, completion logic |
| `parser.rs` | Tree-sitter AST walking, symbol/include/identifier extraction |
| `builtins.rs` | Static data for all MQL5 built-in APIs (functions, enums, structs, constants) |
| `symbols.rs` | Cross-file symbol index with definition + reference tracking |
| `documents.rs` | Incremental document sync with ropey text buffers |
| `includes.rs` | MQL5 include resolution with backslash normalization + transitive scanning |

---

## Builtins Coverage

| Category | Count |
|---|---|
| Functions (with signatures and docs) | 357+ |
| Enum types (with values) | 72 |
| Struct types (with fields) | 9 |
| Constants | 172 |
| Global variables (`_Symbol`, `_Point`, etc.) | 12 |
| Keywords | Full list |

**Total autocomplete items**: 1398+

---

## Phase Status

### Phase 1 — Core Features (COMPLETE)

All features are implemented and working.

| Feature | Status | Notes |
|---|---|---|
| Autocomplete | Done | 1398+ items, context-aware: after `.`, after `::`, general scope |
| Go-to-definition | Done | Cross-file, resolves definitions from workspace + builtins |
| Hover | Done | Type info, function signatures, doc comments |
| Document symbols / outline | Done | Full outline for the active file |
| Workspace symbols | Done | Search across all indexed files |
| Include resolution | Done | Backslash-to-forward-slash normalization, transitive include scanning |
| File management | Done | Incremental sync via ropey, handles `didOpen`/`didChange`/`didClose` |

### Phase 2 — Extended Features (IMPLEMENTED, needs polish)

Features are functional but have known limitations or rough edges.

| Feature | Status | Limitation |
|---|---|---|
| Find all references | Implemented | Uses tree-sitter identifier extraction across all indexed files; works but may miss some edge cases |
| Rename | Implemented | Cross-file rename; prevents renaming builtins (returns error) |
| Signature help | Implemented | Parameter hints with active parameter tracking |
| Type resolution for dot-completion | Implemented | Uses basic line-scanning heuristic — needs AST-based approach for reliability |
| MQL5 stdlib indexing on startup | **NOT YET DONE** | Need to auto-scan `MQL5/Include/` directory on startup and index all stdlib symbols |

### Phase 3 — Diagnostics (PARTIALLY IMPLEMENTED, most disabled)

Diagnostics are the hardest part due to the C++-based grammar producing false positives on valid MQL5 code.

| Diagnostic | Status | Notes |
|---|---|---|
| Duplicate definition detection | **ENABLED** | Works, reports duplicate symbols within a file |
| Syntax error diagnostics | **DISABLED** | Tree-sitter-cpp flags valid MQL5 constructs as errors (see C++ incompatibilities below) |
| Unresolved include diagnostics | **DISABLED** | False positives from subdirectory resolution edge cases |
| Undeclared identifier diagnostics | Not implemented | Requires complete symbol resolution including stdlib |
| Wrong argument count diagnostics | Not implemented | Requires full function signature resolution for all callees |

---

## Known C++ / MQL5 Incompatibilities

These are valid MQL5 constructs that tree-sitter-cpp parses incorrectly or rejects, making syntax-level diagnostics unreliable without a true MQL5 grammar.

| MQL5 Construct | C++ Equivalent | Problem |
|---|---|---|
| `ptr.member` (dot on pointers) | `ptr->member` | Tree-sitter expects `->` for pointer access |
| `double arr[]` as function parameter | `double* arr` or `std::vector<double>&` | C++ does not allow unsized array parameters this way |
| `double& arr[]` (reference to dynamic array) | No direct equivalent | Combined reference + unsized array is not valid C++ |
| `"hello" + "world"` (string concat with `+`) | `std::string("hello") + "world"` | C++ string literals are `const char*`, no `operator+` |
| `C'255,0,0'` (color literals) | No equivalent | Entirely MQL5-specific syntax |
| Implicit pointer upcasting | Explicit `static_cast` / `dynamic_cast` | MQL5 allows implicit upcast; C++ requires explicit cast |

**Consequence**: Syntax diagnostics (Phase 3) cannot be fully enabled until a native MQL5 tree-sitter grammar is written. The current grammar will always flag some valid MQL5 code as erroneous.

---

## Roadmap / TODO

Ordered by priority and impact.

### High Priority

1. **Auto-scan `MQL5/Include/` on startup for stdlib symbols**
   - Currently the LSP only indexes files that are opened or transitively included. The standard library (`MQL5/Include/`) is not pre-indexed.
   - This means autocomplete and go-to-definition do not cover stdlib classes/functions until the user opens a file that includes them.
   - Implementation: on `initialized`, walk the `MQL5/Include/` directory tree, parse all `.mqh` files, and add their symbols to the index.

2. **AST-based type resolution for dot-completion**
   - The current line-scanning heuristic (regex on the current line to find the variable, then searching for its declaration) is fragile.
   - Need to walk the tree-sitter AST to find the variable's declaration, resolve its type, then look up that type's members.
   - This will also improve hover and go-to-definition for member access expressions.

3. **Clean up compiler warnings**
   - 9 warnings currently. Should be resolved to keep the build clean.

### Medium Priority

4. **Re-enable include diagnostics with improved resolver**
   - Fix false positives from subdirectory resolution. The resolver needs to handle all of MQL5's include search paths correctly.

5. **Undeclared function call diagnostics**
   - Once stdlib indexing (item 1) is complete, flag calls to functions that are not defined anywhere in the workspace or builtins.

6. **Wrong argument count diagnostics**
   - Compare call-site argument count against the function's declared parameter count. Must handle default parameters correctly.

### Low Priority (Long-term)

7. **Comprehensive test suite**
   - Unit tests for parser, symbol index, include resolution.
   - Integration tests with real MQL5 files exercising all LSP features.

8. **Write a true MQL5 `grammar.js`**
   - Fork tree-sitter-cpp and modify it to accept all MQL5-specific constructs (dot on pointers, color literals, dynamic arrays, etc.).
   - This would unlock reliable syntax diagnostics (Phase 3) and eliminate all the C++ incompatibility issues.
   - Major undertaking — tree-sitter-cpp's grammar is complex. Alternatively, write from scratch using the MQL5 language reference as the spec.

---

## Architecture Notes

### Symbol Index

The symbol index (`symbols.rs`) maintains a map of symbol name to a list of definitions. Each definition records:
- File URI
- Byte range in the source
- Symbol kind (function, class, enum, variable, etc.)
- Container (parent class/struct, if any)

References are tracked separately. Find-all-references walks all indexed files and collects identifier nodes matching the target name.

### Include Resolution

`includes.rs` resolves `#include` directives with the following behavior:
- Backslash paths (`\`) are normalized to forward slashes (`/`)
- Angle-bracket includes (`#include <Foo.mqh>`) resolve relative to `MQL5/Include/`
- Quote includes (`#include "Foo.mqh"`) resolve relative to the including file's directory
- Transitive includes are scanned recursively, building a full dependency graph

### Completion Strategy

Completion is context-aware with three modes:
1. **After `.`**: Resolve the type of the expression before the dot, return its members
2. **After `::`**: Resolve the namespace/class before `::`, return its static members and enum values
3. **General**: Return all in-scope symbols — locals, file-level definitions, included symbols, builtins

### Incremental Sync

Documents are tracked with ropey ropes (`documents.rs`). The server advertises `TextDocumentSyncKind::Incremental`, so the editor sends only the changed ranges. Ropey applies these edits efficiently. On each change, the document is re-parsed with tree-sitter and the symbol index is updated.
