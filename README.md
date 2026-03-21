# mql5-lsp

A native Language Server Protocol (LSP) implementation for MQL5 (MetaQuotes Language 5), built in Rust.

Works with any LSP-capable editor: Zed, VS Code, Neovim, Emacs, etc.

## Features

| Feature | Description |
|---------|-------------|
| **Autocomplete** | 1400+ items: builtins + workspace symbols. Context-aware: `.` for members, `::` for enum values, general scope |
| **Go-to-definition** | Cross-file navigation for functions, classes, enums, variables, macros, `#include` paths |
| **Find all references** | Scope-aware search across all workspace files |
| **Rename** | Scope-aware cross-file rename (prevents renaming builtins) |
| **Hover** | Type signatures, documentation, struct field lists |
| **Signature help** | Parameter hints with active parameter tracking |
| **Diagnostics** | Unresolved includes, duplicate definitions, unknown functions, wrong argument count |
| **Color swatches** | `C'r,g,b'` literals, 140+ named colors (`clrRed`, etc.), `0xRRGGBB` hex |
| **Auto-import** | Suggests `#include` when completing symbols from other files |
| **Code formatting** | Indentation, trailing whitespace, blank line normalization |
| **Inlay hints** | Parameter names at function call sites |
| **Semantic tokens** | Builtins visually distinct from user-defined symbols |
| **Document outline** | All symbols in the active file |
| **Workspace symbols** | Fuzzy search across all indexed files |

## MQL5 API Coverage

| Category | Count |
|----------|-------|
| Built-in functions (with signatures and docs) | 397+ |
| Enum types (with all values) | 72 |
| Struct types (with fields) | 9 |
| Constants | 172 |
| Global variables (`_Symbol`, `_Point`, etc.) | 12 |
| Named colors | 140+ |

## Installation

### Build from source

```bash
git clone https://github.com/MichalBPL/mql5-language-server.git
cd mql5-language-server
cargo build --release
# Binary: target/release/mql5-lsp
```

### Pre-built binaries

Download from [Releases](https://github.com/MichalBPL/mql5-language-server/releases).

### Editor configuration

**Zed** (via [zed-mql5](https://github.com/MichalBPL/zed-mql5) extension):
```json
{
  "lsp": {
    "mql5-lsp": {
      "binary": { "path": "/path/to/mql5-lsp" }
    }
  }
}
```

**Neovim** (via nvim-lspconfig):
```lua
vim.lsp.start({
  name = "mql5-lsp",
  cmd = { "/path/to/mql5-lsp" },
  root_dir = vim.fs.dirname(vim.fs.find({ "MQL5" }, { upward = true })[1]),
})
```

## Architecture

Built with `tower-lsp` and `tree-sitter`. 7000+ lines of Rust.

```
src/
├── main.rs          # LSP server, all request handlers
├── parser.rs        # Tree-sitter AST walking, symbol extraction
├── builtins.rs      # MQL5 built-in API definitions (functions, enums, structs, constants)
├── symbols.rs       # Cross-file symbol index with definition + reference tracking
├── documents.rs     # Incremental document sync (ropey text buffers)
├── formatter.rs     # Code formatting engine
└── includes.rs      # Include resolution (backslash normalization, transitive scanning)
```

The parser uses a tree-sitter grammar derived from `tree-sitter-cpp`, extended with MQL5 keywords (`input`, `sinput`) and types (`datetime`, `color`).

## Known Limitations

- **Tree-sitter grammar**: Based on C++, so some valid MQL5 constructs (`C'r,g,b'`, `type& arr[]`, `ptr.member`) produce parse errors. These are suppressed by the LSP.
- **Type resolution**: AST-based but does not handle complex expression chains.
- **Diagnostics**: Conservative — prefers no false positives over catching everything.

## License

MIT
