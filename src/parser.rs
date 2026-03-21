//! Tree-sitter based MQL5 parser for symbol extraction.

use tree_sitter::{Language, Node, Parser, Tree};

unsafe extern "C" {
    fn tree_sitter_mql5() -> *const std::ffi::c_void;
}

/// Get the tree-sitter MQL5 language.
pub fn mql5_language() -> Language {
    let ptr = unsafe { tree_sitter_mql5() };
    unsafe { Language::from_raw(ptr as *const _) }
}

/// Create a new parser configured for MQL5.
pub fn new_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&mql5_language())
        .expect("failed to set MQL5 language");
    parser
}

/// Parse source code into a tree-sitter tree.
pub fn parse(source: &str) -> Option<Tree> {
    let mut parser = new_parser();
    parser.parse(source, None)
}

/// Incremental parse with an old tree.
#[allow(dead_code)]
pub fn parse_incremental(source: &str, old_tree: &Tree) -> Option<Tree> {
    let mut parser = new_parser();
    parser.parse(source, Some(old_tree))
}

// ── Symbol extraction from tree-sitter AST ──

#[derive(Clone, Debug)]
pub struct ParsedSymbol {
    pub name: String,
    pub kind: ParsedSymbolKind,
    pub detail: String,
    pub start_line: u32,
    pub end_line: u32,
    /// For class/struct members
    pub parent_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum ParsedSymbolKind {
    Function,
    Class,
    Struct,
    Enum,
    EnumValue,
    Define,
    InputVar,
    GlobalVar,
    Method,
    Field,
    TypeAlias,
}

/// Extract all symbols from parsed source code.
pub fn extract_symbols(source: &str, tree: &Tree) -> Vec<ParsedSymbol> {
    let mut symbols = Vec::new();
    let root = tree.root_node();
    extract_from_node(root, source, &mut symbols, None);
    symbols
}

fn extract_from_node(
    node: Node,
    source: &str,
    symbols: &mut Vec<ParsedSymbol>,
    parent_class: Option<&str>,
) {
    match node.kind() {
        "function_definition" | "function_declarator" => {
            if parent_class.is_none() {
                if let Some(sym) = extract_function(node, source, parent_class) {
                    symbols.push(sym);
                }
            }
        }
        "declaration" => {
            extract_declaration(node, source, symbols, parent_class);
        }
        "class_specifier" | "struct_specifier" => {
            extract_class_or_struct(node, source, symbols);
        }
        "enum_specifier" => {
            extract_enum(node, source, symbols);
        }
        "preproc_def" | "preproc_function_def" => {
            extract_define(node, source, symbols);
        }
        _ => {}
    }

    // Recurse into children (but not into class/struct bodies — handled separately)
    if !matches!(node.kind(), "class_specifier" | "struct_specifier") {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            extract_from_node(child, source, symbols, parent_class);
        }
    }
}

fn node_text<'a>(node: Node, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

fn extract_function(
    node: Node,
    source: &str,
    parent_class: Option<&str>,
) -> Option<ParsedSymbol> {
    // Find the declarator node which contains the function name
    let declarator = find_child_by_kind(node, "function_declarator")
        .or_else(|| find_child_by_kind(node, "pointer_declarator"))?;

    let name_node = find_child_by_kind(declarator, "identifier")
        .or_else(|| find_child_by_kind(declarator, "field_identifier"))?;

    let name = node_text(name_node, source).to_string();

    // Skip common false positives
    if matches!(
        name.as_str(),
        "if" | "for" | "while" | "switch" | "return" | "else" | "do"
    ) {
        return None;
    }

    // Build signature from the full function node text (first line)
    let full_text = node_text(node, source);
    let first_line = full_text.lines().next().unwrap_or(&full_text);
    let detail = first_line
        .trim()
        .trim_end_matches('{')
        .trim()
        .to_string();

    let kind = if parent_class.is_some() {
        ParsedSymbolKind::Method
    } else {
        ParsedSymbolKind::Function
    };

    Some(ParsedSymbol {
        name,
        kind,
        detail,
        start_line: node.start_position().row as u32,
        end_line: node.end_position().row as u32,
        parent_name: parent_class.map(|s| s.to_string()),
    })
}

fn extract_declaration(
    node: Node,
    source: &str,
    symbols: &mut Vec<ParsedSymbol>,
    parent_class: Option<&str>,
) {
    // Check for input/sinput variables
    let text = node_text(node, source);
    if text.starts_with("input ") || text.starts_with("sinput ") {
        // Parse: input TYPE NAME = ...;
        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.len() >= 3 {
            let var_name = parts[2].trim_end_matches(';').trim_end_matches('=');
            if !var_name.is_empty() {
                symbols.push(ParsedSymbol {
                    name: var_name.to_string(),
                    kind: ParsedSymbolKind::InputVar,
                    detail: text.lines().next().unwrap_or(text).trim().to_string(),
                    start_line: node.start_position().row as u32,
                    end_line: node.end_position().row as u32,
                    parent_name: parent_class.map(|s| s.to_string()),
                });
            }
        }
        return;
    }

    // Check for typedef
    if text.starts_with("typedef ") {
        let parts: Vec<&str> = text.split_whitespace().collect();
        if let Some(name) = parts.last() {
            let name = name.trim_end_matches(';');
            if !name.is_empty() {
                symbols.push(ParsedSymbol {
                    name: name.to_string(),
                    kind: ParsedSymbolKind::TypeAlias,
                    detail: text.lines().next().unwrap_or(text).trim().to_string(),
                    start_line: node.start_position().row as u32,
                    end_line: node.end_position().row as u32,
                    parent_name: None,
                });
            }
        }
        return;
    }

    // Check for function declarations (not definitions — no body)
    if let Some(_declarator) = find_child_by_kind(node, "function_declarator") {
        if let Some(sym) = extract_function(node, source, parent_class) {
            symbols.push(sym);
        }
        return;
    }

    // Regular variable declarations at file scope or as class members
    if parent_class.is_some() {
        // Class member variable
        if let Some(declarator) = find_child_by_kind(node, "identifier")
            .or_else(|| find_descendant_by_kind(node, "field_identifier"))
        {
            let name = node_text(declarator, source).to_string();
            let detail = text.lines().next().unwrap_or(text).trim().to_string();
            symbols.push(ParsedSymbol {
                name,
                kind: ParsedSymbolKind::Field,
                detail,
                start_line: node.start_position().row as u32,
                end_line: node.end_position().row as u32,
                parent_name: parent_class.map(|s| s.to_string()),
            });
        }
    }
}

fn extract_class_or_struct(node: Node, source: &str, symbols: &mut Vec<ParsedSymbol>) {
    let is_class = node.kind() == "class_specifier";
    let kind = if is_class {
        ParsedSymbolKind::Class
    } else {
        ParsedSymbolKind::Struct
    };

    // Find name
    let name_node = find_child_by_kind(node, "type_identifier");
    let name = match name_node {
        Some(n) => node_text(n, source).to_string(),
        None => return, // anonymous struct/class
    };

    // Find base class
    let base = find_child_by_kind(node, "base_class_clause").map(|bc| {
        let bc_text = node_text(bc, source);
        bc_text.trim_start_matches(':').trim().to_string()
    });

    let detail = if let Some(ref base) = base {
        format!(
            "{} {} : {}",
            if is_class { "class" } else { "struct" },
            name,
            base
        )
    } else {
        format!("{} {}", if is_class { "class" } else { "struct" }, name)
    };

    symbols.push(ParsedSymbol {
        name: name.clone(),
        kind,
        detail,
        start_line: node.start_position().row as u32,
        end_line: node.end_position().row as u32,
        parent_name: None,
    });

    // Extract members from the body
    if let Some(body) = find_child_by_kind(node, "field_declaration_list") {
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            match child.kind() {
                "field_declaration" => {
                    extract_class_member(child, source, symbols, &name);
                }
                "function_definition" => {
                    if let Some(sym) = extract_function(child, source, Some(&name)) {
                        symbols.push(sym);
                    }
                }
                "declaration" => {
                    extract_declaration(child, source, symbols, Some(&name));
                }
                "access_specifier" => {} // skip public:/private:/protected:
                _ => {}
            }
        }
    }
}

fn extract_class_member(
    node: Node,
    source: &str,
    symbols: &mut Vec<ParsedSymbol>,
    class_name: &str,
) {
    let text = node_text(node, source);

    // Check if this is a method declaration (has parameter_list)
    if find_descendant_by_kind(node, "function_declarator").is_some() {
        if let Some(sym) = extract_function(node, source, Some(class_name)) {
            symbols.push(sym);
        }
        return;
    }

    // It's a field
    let name_node = find_child_by_kind(node, "field_identifier")
        .or_else(|| find_descendant_by_kind(node, "field_identifier"));
    if let Some(n) = name_node {
        let name = node_text(n, source).to_string();
        symbols.push(ParsedSymbol {
            name,
            kind: ParsedSymbolKind::Field,
            detail: text.trim().trim_end_matches(';').trim().to_string(),
            start_line: node.start_position().row as u32,
            end_line: node.end_position().row as u32,
            parent_name: Some(class_name.to_string()),
        });
    }
}

fn extract_enum(node: Node, source: &str, symbols: &mut Vec<ParsedSymbol>) {
    let name_node = find_child_by_kind(node, "type_identifier");
    let name = match name_node {
        Some(n) => node_text(n, source).to_string(),
        None => return,
    };

    symbols.push(ParsedSymbol {
        name: name.clone(),
        kind: ParsedSymbolKind::Enum,
        detail: format!("enum {}", name),
        start_line: node.start_position().row as u32,
        end_line: node.end_position().row as u32,
        parent_name: None,
    });

    // Extract enum values
    if let Some(body) = find_child_by_kind(node, "enumerator_list") {
        let mut cursor = body.walk();
        for child in body.children(&mut cursor) {
            if child.kind() == "enumerator" {
                if let Some(id) = find_child_by_kind(child, "identifier") {
                    let val_name = node_text(id, source).to_string();
                    symbols.push(ParsedSymbol {
                        name: val_name.clone(),
                        kind: ParsedSymbolKind::EnumValue,
                        detail: format!("{}::{}", name, val_name),
                        start_line: child.start_position().row as u32,
                        end_line: child.end_position().row as u32,
                        parent_name: Some(name.clone()),
                    });
                }
            }
        }
    }
}

fn extract_define(node: Node, source: &str, symbols: &mut Vec<ParsedSymbol>) {
    if let Some(name_node) = find_child_by_kind(node, "identifier") {
        let name = node_text(name_node, source).to_string();
        let text = node_text(node, source);
        symbols.push(ParsedSymbol {
            name,
            kind: ParsedSymbolKind::Define,
            detail: text.lines().next().unwrap_or(text).trim().to_string(),
            start_line: node.start_position().row as u32,
            end_line: node.end_position().row as u32,
            parent_name: None,
        });
    }
}

// ── Tree-sitter node helpers ──

fn find_child_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    let result = node.children(&mut cursor).find(|c| c.kind() == kind);
    result
}

fn find_descendant_by_kind<'a>(node: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let child_count = node.child_count();
    for i in 0..child_count {
        let child = node.child(i).unwrap();
        if child.kind() == kind {
            return Some(child);
        }
        if let Some(found) = find_descendant_by_kind(child, kind) {
            return Some(found);
        }
    }
    None
}

// ── Include extraction ──

/// Extract #include directives from source code.
pub fn extract_includes(source: &str, tree: &Tree) -> Vec<IncludeDirective> {
    let mut includes = Vec::new();
    let root = tree.root_node();
    let mut cursor = root.walk();

    for child in root.children(&mut cursor) {
        if child.kind() == "preproc_include" {
            if let Some(inc) = parse_include_directive(child, source) {
                includes.push(inc);
            }
        }
    }

    // Also scan with regex as fallback for tree-sitter edge cases
    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("#include") {
            if let Some(inc) = parse_include_line(trimmed, i as u32) {
                // Avoid duplicates
                if !includes.iter().any(|existing: &IncludeDirective| existing.path == inc.path) {
                    includes.push(inc);
                }
            }
        }
    }

    includes
}

#[derive(Clone, Debug)]
pub struct IncludeDirective {
    pub path: String,
    pub is_system: bool, // true for <>, false for ""
    pub line: u32,
}

fn parse_include_directive(node: Node, source: &str) -> Option<IncludeDirective> {
    let text = node_text(node, source).trim();
    parse_include_line(text, node.start_position().row as u32)
}

/// Public wrapper for use from go-to-definition.
pub fn parse_include_from_line(line: &str, line_num: u32) -> Option<IncludeDirective> {
    parse_include_line(line, line_num)
}

fn parse_include_line(line: &str, line_num: u32) -> Option<IncludeDirective> {
    let after_include = line.strip_prefix("#include")?.trim();

    if after_include.starts_with('<') {
        // Extract between < and first >
        let start = 1; // skip '<'
        let end = after_include.find('>')?;
        let path = &after_include[start..end];
        Some(IncludeDirective {
            path: path.replace('\\', "/"),
            is_system: true,
            line: line_num,
        })
    } else if after_include.starts_with('"') {
        // Extract between first " and second "
        let start = 1; // skip opening '"'
        let end = after_include[start..].find('"')? + start;
        let path = &after_include[start..end];
        Some(IncludeDirective {
            path: path.replace('\\', "/"),
            is_system: false,
            line: line_num,
        })
    } else {
        None
    }
}

// ── Completion context detection ──

#[derive(Debug, PartialEq)]
pub enum CompletionContext {
    /// After a dot: `obj.` — show members of the type
    DotAccess { object_text: String },
    /// After `::` — show static members or scope resolution
    ScopeResolution { scope_text: String },
    /// Inside a function body — show locals + params + class members + globals
    General,
}

/// Determine what kind of completion to provide based on cursor position.
pub fn get_completion_context(source: &str, line: usize, col: usize) -> CompletionContext {
    let target_line = match source.lines().nth(line) {
        Some(l) => l,
        None => return CompletionContext::General,
    };

    // Get text before cursor on this line
    let before_cursor = if col <= target_line.len() {
        &target_line[..col]
    } else {
        target_line
    };
    let trimmed = before_cursor.trim_end();

    // Check for dot access: `expr.`
    if trimmed.ends_with('.') {
        let before_dot = trimmed[..trimmed.len() - 1].trim_end();

        // Handle array indexing: `arr[i].` or `arr[expr].` — extract `arr`
        let object_source = if before_dot.ends_with(']') {
            // Walk backwards to find matching '['
            let bytes = before_dot.as_bytes();
            let mut depth = 0i32;
            let mut bracket_start = None;
            let mut i = bytes.len();
            while i > 0 {
                i -= 1;
                match bytes[i] {
                    b']' => depth += 1,
                    b'[' => {
                        depth -= 1;
                        if depth == 0 {
                            bracket_start = Some(i);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            match bracket_start {
                Some(pos) => &before_dot[..pos],
                None => before_dot,
            }
        } else {
            before_dot
        };

        // Handle chained dot access: `a.b.` — extract `b` but resolve through `a`
        // For now extract the last identifier (handles simple `obj.` and `arr[i].`)
        let object = extract_last_identifier(object_source);
        if !object.is_empty() {
            return CompletionContext::DotAccess {
                object_text: object.to_string(),
            };
        }
    }

    // Check for scope resolution: `Type::`
    if trimmed.ends_with("::") {
        let before_colons = trimmed[..trimmed.len() - 2].trim_end();
        let scope = extract_last_identifier(before_colons);
        if !scope.is_empty() {
            return CompletionContext::ScopeResolution {
                scope_text: scope.to_string(),
            };
        }
    }

    CompletionContext::General
}

fn extract_last_identifier(text: &str) -> &str {
    let bytes = text.as_bytes();
    let mut end = bytes.len();
    while end > 0 && !bytes[end - 1].is_ascii_alphanumeric() && bytes[end - 1] != b'_' {
        end -= 1;
    }
    let mut start = end;
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }
    &text[start..end]
}

/// Extract the word (identifier) at a given line/col position.
pub fn extract_word_at(source: &str, line: usize, col: usize) -> Option<String> {
    let target_line = source.lines().nth(line)?;
    let bytes = target_line.as_bytes();
    if col >= bytes.len() {
        return None;
    }

    let mut start = col;
    while start > 0 && is_ident_char(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = col;
    while end < bytes.len() && is_ident_char(bytes[end]) {
        end += 1;
    }

    if start == end {
        return None;
    }
    Some(target_line[start..end].to_string())
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

// ── Identifier extraction (for references) ──

/// An identifier usage with its scope context.
#[derive(Clone, Debug)]
pub struct IdentifierUsage {
    pub name: String,
    pub line: u32,
    pub start_col: u32,
    pub end_col: u32,
    /// The enclosing scope: "global", "ClassName", "ClassName::MethodName", or "FunctionName"
    pub scope: String,
}

/// Extract all identifier usages from a parsed tree with scope tracking.
pub fn extract_identifiers_scoped(source: &str, tree: &Tree) -> Vec<IdentifierUsage> {
    let mut identifiers = Vec::new();
    collect_identifiers_scoped(tree.root_node(), source, &mut identifiers, "global");
    identifiers
}

/// Legacy non-scoped version for backward compatibility.
#[allow(dead_code)]
pub fn extract_identifiers(source: &str, tree: &Tree) -> Vec<(String, u32, u32, u32)> {
    extract_identifiers_scoped(source, tree)
        .into_iter()
        .map(|id| (id.name, id.line, id.start_col, id.end_col))
        .collect()
}

fn collect_identifiers_scoped(
    node: Node,
    source: &str,
    out: &mut Vec<IdentifierUsage>,
    current_scope: &str,
) {
    // Determine new scope for children
    let child_scope;
    match node.kind() {
        "function_definition" => {
            // Extract function name for scope
            let func_name = find_child_by_kind(node, "function_declarator")
                .and_then(|d| find_child_by_kind(d, "identifier")
                    .or_else(|| find_child_by_kind(d, "field_identifier")))
                .map(|n| node_text(n, source))
                .unwrap_or("?");
            if current_scope == "global" {
                child_scope = func_name.to_string();
            } else {
                child_scope = format!("{}::{}", current_scope, func_name);
            }
        }
        "class_specifier" | "struct_specifier" => {
            let type_name = find_child_by_kind(node, "type_identifier")
                .map(|n| node_text(n, source))
                .unwrap_or("?");
            child_scope = type_name.to_string();
        }
        _ => {
            child_scope = current_scope.to_string();
        }
    }

    match node.kind() {
        "identifier" | "field_identifier" | "type_identifier" => {
            let text = &source[node.byte_range()];
            if !text.is_empty() && text.chars().next().is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
                let start = node.start_position();
                let end = node.end_position();
                out.push(IdentifierUsage {
                    name: text.to_string(),
                    line: start.row as u32,
                    start_col: start.column as u32,
                    end_col: end.column as u32,
                    scope: current_scope.to_string(),
                });
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_identifiers_scoped(child, source, out, &child_scope);
    }
}

// ── Variable type resolution from AST ──

/// Resolve the type of a variable at a given line by searching the AST for declarations.
/// Returns the type name if found.
pub fn resolve_type_at(source: &str, tree: &Tree, var_name: &str, use_line: usize) -> Option<String> {
    let root = tree.root_node();
    let mut best: Option<(usize, String)> = None; // (line, type_name) — closest declaration above use site

    resolve_type_recursive(root, source, var_name, use_line, &mut best);
    best.map(|(_, t)| t)
}

fn resolve_type_recursive(
    node: Node,
    source: &str,
    var_name: &str,
    use_line: usize,
    best: &mut Option<(usize, String)>,
) {
    match node.kind() {
        "declaration" | "field_declaration" => {
            // Look for: Type varName ...
            let decl_line = node.start_position().row;
            if decl_line <= use_line {
                try_extract_var_type(node, source, var_name, decl_line, best);
            }
        }
        "parameter_declaration" => {
            // Function parameters: void Foo(Type varName)
            let decl_line = node.start_position().row;
            if decl_line <= use_line {
                try_extract_param_type(node, source, var_name, decl_line, best);
            }
        }
        "init_declarator" => {
            // Type var = new ClassName(...)
            let decl_line = node.start_position().row;
            if decl_line <= use_line {
                try_extract_init_type(node, source, var_name, decl_line, best);
            }
        }
        _ => {}
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        resolve_type_recursive(child, source, var_name, use_line, best);
    }
}

fn try_extract_var_type(
    node: Node,
    source: &str,
    var_name: &str,
    decl_line: usize,
    best: &mut Option<(usize, String)>,
) {
    // Find the type specifier (first child that's a type)
    let mut type_name = None;
    let mut found_var = false;
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "primitive_type" | "type_identifier" | "sized_type_specifier"
            | "template_type" | "qualified_identifier" => {
                type_name = Some(node_text(child, source).to_string());
            }
            "identifier" | "field_identifier" => {
                let name = node_text(child, source);
                if name == var_name {
                    found_var = true;
                }
            }
            "init_declarator" => {
                // Check inside init_declarator for the variable name
                if let Some(id) = find_child_by_kind(child, "identifier") {
                    if node_text(id, source) == var_name {
                        found_var = true;
                        // Check for `= new ClassName(...)`
                        if let Some(new_expr) = find_descendant_by_kind(child, "new_expression") {
                            if let Some(type_id) = find_child_by_kind(new_expr, "type_identifier") {
                                type_name = Some(node_text(type_id, source).to_string());
                            }
                        }
                    }
                }
                // Also check for array init: Type arr[] = ...
                if let Some(arr_decl) = find_descendant_by_kind(child, "array_declarator") {
                    if let Some(id) = find_child_by_kind(arr_decl, "identifier") {
                        if node_text(id, source) == var_name {
                            found_var = true;
                        }
                    }
                }
            }
            "array_declarator" => {
                // Type arr[] — the identifier is inside the array_declarator
                if let Some(id) = find_child_by_kind(child, "identifier") {
                    if node_text(id, source) == var_name {
                        found_var = true;
                    }
                }
            }
            "pointer_declarator" => {
                if let Some(id) = find_descendant_by_kind(child, "identifier") {
                    if node_text(id, source) == var_name {
                        found_var = true;
                    }
                }
            }
            _ => {}
        }
    }

    if found_var {
        if let Some(t) = type_name {
            let t = t.trim_end_matches('*').trim_end_matches('&').to_string();
            match best {
                Some((prev_line, _)) if decl_line > *prev_line => {
                    *best = Some((decl_line, t));
                }
                None => {
                    *best = Some((decl_line, t));
                }
                _ => {}
            }
        }
    }
}

fn try_extract_param_type(
    node: Node,
    source: &str,
    var_name: &str,
    decl_line: usize,
    best: &mut Option<(usize, String)>,
) {
    let mut type_name = None;
    let mut found_var = false;
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        match child.kind() {
            "primitive_type" | "type_identifier" | "sized_type_specifier"
            | "template_type" | "qualified_identifier" => {
                type_name = Some(node_text(child, source).to_string());
            }
            "identifier" => {
                if node_text(child, source) == var_name {
                    found_var = true;
                }
            }
            "pointer_declarator" | "reference_declarator" => {
                if let Some(id) = find_descendant_by_kind(child, "identifier") {
                    if node_text(id, source) == var_name {
                        found_var = true;
                    }
                }
            }
            "array_declarator" => {
                // Type arr[] parameter
                if let Some(id) = find_child_by_kind(child, "identifier") {
                    if node_text(id, source) == var_name {
                        found_var = true;
                    }
                }
                // Also check reference_declarator inside: Type& arr[]
                if let Some(ref_decl) = find_child_by_kind(child, "reference_declarator") {
                    if let Some(id) = find_descendant_by_kind(ref_decl, "identifier") {
                        if node_text(id, source) == var_name {
                            found_var = true;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    if found_var {
        if let Some(t) = type_name {
            let t = t.trim_end_matches('*').trim_end_matches('&').to_string();
            if best.is_none() {
                *best = Some((decl_line, t));
            }
        }
    }
}

fn try_extract_init_type(
    node: Node,
    source: &str,
    var_name: &str,
    decl_line: usize,
    best: &mut Option<(usize, String)>,
) {
    if let Some(id) = find_child_by_kind(node, "identifier") {
        if node_text(id, source) == var_name {
            // Check for `= new ClassName(...)`
            if let Some(new_expr) = find_descendant_by_kind(node, "new_expression") {
                if let Some(type_id) = find_child_by_kind(new_expr, "type_identifier") {
                    let t = node_text(type_id, source).to_string();
                    match best {
                        Some((prev_line, _)) if decl_line > *prev_line => {
                            *best = Some((decl_line, t));
                        }
                        None => {
                            *best = Some((decl_line, t));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

// ── Function call extraction (for diagnostics) ──

/// A function call site found in source code.
#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub name: String,
    pub line: u32,
    pub col: u32,
    pub arg_count: usize,
    /// true if this is a method call (obj.Method())
    pub is_method: bool,
    /// For method calls, the receiver expression (e.g. "trade" in "trade.Buy()")
    pub receiver: Option<String>,
}

/// Extract all function call sites from parsed source code.
pub fn extract_function_calls(source: &str, tree: &Tree) -> Vec<FunctionCall> {
    let mut calls = Vec::new();
    collect_function_calls(tree.root_node(), source, &mut calls);
    calls
}

fn collect_function_calls(node: Node, source: &str, out: &mut Vec<FunctionCall>) {
    if node.kind() == "call_expression" {
        // Count arguments from the argument_list child
        let arg_count = node.child_by_field_name("arguments")
            .or_else(|| find_child_by_kind(node, "argument_list"))
            .map(|args| count_arguments(args, source))
            .unwrap_or(0);

        // The function name is the first child (usually an identifier or field_expression)
        if let Some(func_node) = node.child(0) {
            match func_node.kind() {
                "identifier" => {
                    let name = &source[func_node.byte_range()];
                    let start = func_node.start_position();
                    out.push(FunctionCall {
                        name: name.to_string(),
                        line: start.row as u32,
                        col: start.column as u32,
                        arg_count,
                        is_method: false,
                        receiver: None,
                    });
                }
                "field_expression" => {
                    // obj.Method() — extract method name and receiver
                    let receiver = func_node.child(0)
                        .map(|n| source[n.byte_range()].to_string());
                    if let Some(field) = find_child_by_kind(func_node, "field_identifier") {
                        let name = &source[field.byte_range()];
                        let start = field.start_position();
                        out.push(FunctionCall {
                            name: name.to_string(),
                            line: start.row as u32,
                            col: start.column as u32,
                            arg_count,
                            is_method: true,
                            receiver,
                        });
                    }
                }
                "qualified_identifier" => {
                    let text = &source[func_node.byte_range()];
                    if let Some(last_part) = text.rsplit("::").next() {
                        let start = func_node.start_position();
                        out.push(FunctionCall {
                            name: last_part.to_string(),
                            line: start.row as u32,
                            col: start.column as u32,
                            arg_count,
                            is_method: false,
                            receiver: None,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_function_calls(child, source, out);
    }
}

/// Count arguments in an argument_list node.
/// Uses source text to handle MQL5 color literals C'r,g,b' correctly
/// (commas inside color literals are not argument separators).
fn count_arguments(args_node: Node, source: &str) -> usize {
    let text = &source[args_node.byte_range()];
    // Strip outer parens
    let inner = text.strip_prefix('(').unwrap_or(text);
    let inner = inner.strip_suffix(')').unwrap_or(inner);
    let inner = inner.trim();
    if inner.is_empty() {
        return 0;
    }

    let mut count = 1usize; // at least 1 arg if non-empty
    let mut depth = 0i32;
    let mut in_color_literal = false;
    let bytes = inner.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'<' | b'[' => depth += 1,
            b')' | b'>' | b']' => depth -= 1,
            b'\'' if in_color_literal => {
                in_color_literal = false; // closing '
            }
            b'\'' => {
                // Check for C' color literal: look back for 'C'
                if i > 0 && bytes[i - 1] == b'C' {
                    in_color_literal = true;
                }
            }
            b',' if depth == 0 && !in_color_literal => {
                count += 1;
            }
            _ => {}
        }
        i += 1;
    }
    count
}

// ── Argument position extraction (for inlay hints) ──

/// Extract the (line, col) positions of each argument in a function call at the given location.
pub fn extract_call_arg_positions(
    source: &str,
    tree: &Tree,
    call_line: u32,
    call_col: u32,
) -> Vec<(u32, u32)> {
    let root = tree.root_node();
    let mut positions = Vec::new();
    find_call_args(root, source, call_line, call_col, &mut positions);
    positions
}

fn find_call_args(
    node: Node,
    source: &str,
    target_line: u32,
    target_col: u32,
    out: &mut Vec<(u32, u32)>,
) {
    if node.kind() == "call_expression" {
        // Check if this call is at the right location
        let func_node = node.child(0);
        let matches = func_node.map(|f| {
            let start = f.start_position();
            start.row as u32 == target_line && start.column as u32 == target_col
        }).unwrap_or(false);

        // Also check field_identifier for method calls
        let matches = matches || func_node.and_then(|f| {
            if f.kind() == "field_expression" {
                find_child_by_kind(f, "field_identifier")
            } else {
                None
            }
        }).map(|f| {
            let start = f.start_position();
            start.row as u32 == target_line && start.column as u32 == target_col
        }).unwrap_or(false);

        if matches {
            if let Some(args) = node.child_by_field_name("arguments")
                .or_else(|| find_child_by_kind(node, "argument_list"))
            {
                // Use source text to find arg start positions (handles color literals)
                let text = &source[args.byte_range()];
                let base_byte = args.start_byte();

                // Find positions of each argument start
                let inner_start = 1; // skip '('
                let inner_end = text.len().saturating_sub(1); // skip ')'
                if inner_start < inner_end {
                    let inner = &text[inner_start..inner_end];
                    let mut depth = 0i32;
                    let mut in_color = false;
                    // First arg starts right after '('
                    // Find its position, skipping whitespace
                    let first_non_ws = inner.find(|c: char| !c.is_whitespace())
                        .unwrap_or(0);
                    let abs_byte = base_byte + inner_start + first_non_ws;
                    let (line, col) = byte_to_line_col(source, abs_byte);
                    out.push((line, col));

                    for (i, b) in inner.bytes().enumerate() {
                        match b {
                            b'(' | b'<' | b'[' => depth += 1,
                            b')' | b'>' | b']' => depth -= 1,
                            b'\'' if in_color => in_color = false,
                            b'\'' if i > 0 && inner.as_bytes()[i-1] == b'C' => in_color = true,
                            b',' if depth == 0 && !in_color => {
                                // Next arg starts after this comma
                                let after_comma = &inner[i+1..];
                                let ws_skip = after_comma.find(|c: char| !c.is_whitespace())
                                    .unwrap_or(0);
                                let abs_byte = base_byte + inner_start + i + 1 + ws_skip;
                                let (line, col) = byte_to_line_col(source, abs_byte);
                                out.push((line, col));
                            }
                            _ => {}
                        }
                    }
                }
            }
            return; // Found our call, done
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        find_call_args(child, source, target_line, target_col, out);
        if !out.is_empty() {
            return;
        }
    }
}

/// Convert a byte offset in source to (line, col).
fn byte_to_line_col(source: &str, byte_offset: usize) -> (u32, u32) {
    let mut line = 0u32;
    let mut col = 0u32;
    for (i, ch) in source.char_indices() {
        if i >= byte_offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

// ── Error node extraction (for diagnostics) ──

/// A syntax error found by tree-sitter.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub message: String,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

/// Extract all ERROR and MISSING nodes from the parse tree.
#[allow(dead_code)]
pub fn extract_errors(source: &str, tree: &Tree) -> Vec<SyntaxError> {
    let mut errors = Vec::new();
    collect_errors(tree.root_node(), source, &mut errors);
    errors
}

#[allow(dead_code)]
fn collect_errors(node: Node, source: &str, out: &mut Vec<SyntaxError>) {
    if node.is_error() {
        let text = &source[node.byte_range()];
        let preview: String = text.chars().take(40).collect();
        let start = node.start_position();
        let end = node.end_position();
        out.push(SyntaxError {
            message: format!("Syntax error near `{}`", preview.trim()),
            start_line: start.row as u32,
            start_col: start.column as u32,
            end_line: end.row as u32,
            end_col: end.column as u32,
        });
    } else if node.is_missing() {
        let start = node.start_position();
        let end = node.end_position();
        out.push(SyntaxError {
            message: format!("Missing `{}`", node.kind()),
            start_line: start.row as u32,
            start_col: start.column as u32,
            end_line: end.row as u32,
            end_col: end.column as u32,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_errors(child, source, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let source = r#"
void OnTick() {
    Print("Hello");
}
"#;
        let tree = parse(source).unwrap();
        let symbols = extract_symbols(source, &tree);
        assert!(symbols.iter().any(|s| s.name == "OnTick"));
    }

    #[test]
    fn test_include_extraction() {
        let source = r#"
#include <Trade\Trade.mqh>
#include "MyHelper.mqh"
"#;
        let tree = parse(source).unwrap();
        let includes = extract_includes(source, &tree);
        assert!(includes.iter().any(|i| i.path == "Trade/Trade.mqh" && i.is_system));
        assert!(includes.iter().any(|i| i.path == "MyHelper.mqh" && !i.is_system));
    }

    #[test]
    fn test_completion_context() {
        assert_eq!(
            get_completion_context("   trade.", 0, 9),
            CompletionContext::DotAccess {
                object_text: "trade".to_string()
            }
        );
        assert_eq!(
            get_completion_context("   CTrade::", 0, 11),
            CompletionContext::ScopeResolution {
                scope_text: "CTrade".to_string()
            }
        );
        assert_eq!(
            get_completion_context("   int x = ", 0, 11),
            CompletionContext::General
        );
    }

    #[test]
    fn test_extract_class_with_members() {
        let source = "class CMyClass { public: int m_value; void SetValue(int val) { } int GetValue() { return 0; } };";
        let tree = parse(source).unwrap();
        let symbols = extract_symbols(source, &tree);

        assert!(symbols.iter().any(|s| s.name == "CMyClass" && s.kind == ParsedSymbolKind::Class));
        assert!(symbols.iter().any(|s| s.name == "m_value" && s.kind == ParsedSymbolKind::Field));
        assert!(symbols.iter().any(|s| s.name == "SetValue" && s.kind == ParsedSymbolKind::Method));
        assert!(symbols.iter().any(|s| s.name == "GetValue" && s.kind == ParsedSymbolKind::Method));

        // Check parent names
        let method = symbols.iter().find(|s| s.name == "SetValue").unwrap();
        assert_eq!(method.parent_name.as_deref(), Some("CMyClass"));
    }

    #[test]
    fn test_extract_enum() {
        let source = r#"
enum ENUM_MY_TYPE {
    MY_TYPE_A,
    MY_TYPE_B,
    MY_TYPE_C
};
"#;
        let tree = parse(source).unwrap();
        let symbols = extract_symbols(source, &tree);

        assert!(symbols.iter().any(|s| s.name == "ENUM_MY_TYPE" && s.kind == ParsedSymbolKind::Enum));
        assert!(symbols.iter().any(|s| s.name == "MY_TYPE_A" && s.kind == ParsedSymbolKind::EnumValue));
        assert!(symbols.iter().any(|s| s.name == "MY_TYPE_B" && s.kind == ParsedSymbolKind::EnumValue));
        assert_eq!(symbols.iter().filter(|s| s.kind == ParsedSymbolKind::EnumValue).count(), 3);
    }

    #[test]
    fn test_extract_define() {
        let source = r#"
#define MY_CONSTANT 42
#define MY_MACRO(x) ((x) * 2)
"#;
        let tree = parse(source).unwrap();
        let symbols = extract_symbols(source, &tree);

        assert!(symbols.iter().any(|s| s.name == "MY_CONSTANT" && s.kind == ParsedSymbolKind::Define));
        assert!(symbols.iter().any(|s| s.name == "MY_MACRO" && s.kind == ParsedSymbolKind::Define));
    }

    #[test]
    fn test_extract_input_vars() {
        let source = r#"
input int    InpMagicNumber = 12345;
input double InpLotSize     = 0.01;
sinput bool  InpDebug       = false;
"#;
        let tree = parse(source).unwrap();
        let symbols = extract_symbols(source, &tree);

        assert!(symbols.iter().any(|s| s.name == "InpMagicNumber" && s.kind == ParsedSymbolKind::InputVar));
        assert!(symbols.iter().any(|s| s.name == "InpLotSize" && s.kind == ParsedSymbolKind::InputVar));
        assert!(symbols.iter().any(|s| s.name == "InpDebug" && s.kind == ParsedSymbolKind::InputVar));
    }

    #[test]
    fn test_extract_function_calls() {
        let source = r#"
void OnTick() {
    Print("Hello");
    int size = ArraySize(rates);
    chart.Redraw();
}
"#;
        let tree = parse(source).unwrap();
        let calls = extract_function_calls(source, &tree);

        assert!(calls.iter().any(|c| c.name == "Print"));
        assert!(calls.iter().any(|c| c.name == "ArraySize"));
        assert!(calls.iter().any(|c| c.name == "Redraw"));
    }

    #[test]
    fn test_extract_identifiers() {
        let source = r#"
int x = 42;
void Foo() { Print(x); }
"#;
        let tree = parse(source).unwrap();
        let idents = extract_identifiers(source, &tree);

        assert!(idents.iter().any(|(name, _, _, _)| name == "x"));
        assert!(idents.iter().any(|(name, _, _, _)| name == "Foo"));
        assert!(idents.iter().any(|(name, _, _, _)| name == "Print"));
    }

    #[test]
    fn test_resolve_type_at() {
        let source = r#"
void OnTick() {
    MqlTick tick;
    CTrade *trade;
    int x = 42;
}
"#;
        let tree = parse(source).unwrap();
        assert_eq!(resolve_type_at(source, &tree, "tick", 10), Some("MqlTick".to_string()));
        assert_eq!(resolve_type_at(source, &tree, "trade", 10), Some("CTrade".to_string()));
        assert_eq!(resolve_type_at(source, &tree, "x", 10), Some("int".to_string()));
        assert_eq!(resolve_type_at(source, &tree, "nonexistent", 10), None);
    }

    #[test]
    fn test_resolve_array_element_type() {
        let source = r#"
void OnTick() {
    MqlRates rates[];
    CMyClass *objects[];
}
"#;
        let tree = parse(source).unwrap();
        assert_eq!(resolve_type_at(source, &tree, "rates", 10), Some("MqlRates".to_string()));
        assert_eq!(resolve_type_at(source, &tree, "objects", 10), Some("CMyClass".to_string()));
    }

    #[test]
    fn test_completion_context_array_dot() {
        // arr[i]. should extract "arr" as the object
        assert_eq!(
            get_completion_context("    rates[i].", 0, 13),
            CompletionContext::DotAccess { object_text: "rates".to_string() }
        );
        // Nested: arr[foo(x)]. should extract "arr" — cursor at col 21 (after dot)
        assert_eq!(
            get_completion_context("    data[GetIndex()].", 0, 21),
            CompletionContext::DotAccess { object_text: "data".to_string() }
        );
    }

    #[test]
    fn test_color_literal_arg_count() {
        // ColorToARGB(C'30,140,50', 220) should be 2 args, not 4
        let source = "void f() { ColorToARGB(C'30,140,50', 220); }";
        let tree = parse(source).unwrap();
        let calls = extract_function_calls(source, &tree);
        let call = calls.iter().find(|c| c.name == "ColorToARGB").unwrap();
        assert_eq!(call.arg_count, 2, "C'r,g,b' should count as 1 arg, got {}", call.arg_count);
    }

    #[test]
    fn test_extract_word_at() {
        let source = "int myVariable = 42;";
        assert_eq!(extract_word_at(source, 0, 5), Some("myVariable".to_string()));
        assert_eq!(extract_word_at(source, 0, 0), Some("int".to_string()));
        assert_eq!(extract_word_at(source, 0, 17), Some("42".to_string()));
    }
}
