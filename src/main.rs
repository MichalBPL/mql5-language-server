mod builtins;
mod documents;
mod formatter;
mod includes;
mod parser;
mod symbols;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use builtins::*;
use documents::DocumentStore;
use includes::IncludeResolver;
use parser::CompletionContext;
use symbols::SymbolIndex;

struct Mql5Lsp {
    client: Client,
    index: Arc<RwLock<SymbolIndex>>,
    documents: Arc<DocumentStore>,
    include_resolver: Arc<RwLock<IncludeResolver>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Mql5Lsp {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Determine workspace root
        let workspace_root = params
            .workspace_folders
            .as_ref()
            .and_then(|f| f.first())
            .and_then(|f| f.uri.to_file_path().ok())
            .or_else(|| {
                params
                    .root_uri
                    .as_ref()
                    .and_then(|u| u.to_file_path().ok())
            });

        if let Some(ref root) = workspace_root {
            // Detect MQL5 include root
            let include_root;
            {
                let mut resolver = self.include_resolver.write().await;
                resolver.detect_include_root(root);
                include_root = resolver.include_root().cloned();
            }

            // Index workspace files
            {
                let mut index = self.index.write().await;
                index.scan_directory(root);
                log::info!(
                    "Workspace: {} files, {} symbols",
                    index.file_count(),
                    index.symbol_count()
                );

                // Also index MQL5 stdlib (Include/) for go-to-definition
                // and autocomplete of standard library classes
                if let Some(ref inc_root) = include_root {
                    index.scan_directory(inc_root);
                    log::info!(
                        "After stdlib: {} files, {} symbols",
                        index.file_count(),
                        index.symbol_count()
                    );
                }
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                definition_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                })),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                    retrigger_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                inlay_hint_provider: Some(OneOf::Left(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                color_provider: Some(ColorProviderCapability::Simple(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::VARIABLE,      // 0: local variable
                                SemanticTokenType::PARAMETER,     // 1: function parameter
                                SemanticTokenType::FUNCTION,      // 2: function
                                SemanticTokenType::METHOD,        // 3: method
                                SemanticTokenType::PROPERTY,      // 4: class field
                                SemanticTokenType::CLASS,         // 5: class/struct
                                SemanticTokenType::ENUM,          // 6: enum
                                SemanticTokenType::ENUM_MEMBER,   // 7: enum value
                                SemanticTokenType::MACRO,         // 8: #define
                                SemanticTokenType::TYPE,          // 9: type alias
                            ],
                            token_modifiers: vec![
                                SemanticTokenModifier::DECLARATION,    // 0
                                SemanticTokenModifier::DEFINITION,     // 1
                                SemanticTokenModifier::READONLY,       // 2
                                SemanticTokenModifier::STATIC,         // 3
                                SemanticTokenModifier::DEFAULT_LIBRARY,// 4: builtin
                            ],
                        },
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        range: None,
                        ..Default::default()
                    }),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "mql5-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // ── Document Sync ──

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text.clone();
        let version = params.text_document.version;

        self.documents.open(uri.clone(), text.clone(), version);

        // Re-index this file
        if let Ok(path) = uri.to_file_path() {
            let mut index = self.index.write().await;
            index.rescan_file(&path, Some(&text));

            // Also index included files
            self.index_includes(&path, &text, &mut index).await;
        }

        // Publish diagnostics
        self.publish_diagnostics_for(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let version = params.text_document.version;

        self.documents
            .apply_changes(&uri, params.content_changes, version);

        // Re-index from updated content
        if let Ok(path) = uri.to_file_path() {
            if let Some(text) = self.documents.get_text(&uri) {
                let mut index = self.index.write().await;
                index.rescan_file(&path, Some(&text));
            }
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Ok(path) = uri.to_file_path() {
            let text = self.documents.get_text(&uri);
            let mut index = self.index.write().await;
            index.rescan_file(&path, text.as_deref());

            // Re-index includes on save
            let source = text
                .or_else(|| std::fs::read_to_string(&path).ok())
                .unwrap_or_default();
            self.index_includes(&path, &source, &mut index).await;
        }

        // Publish diagnostics on save
        self.publish_diagnostics_for(&uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Clear diagnostics when closing
        self.client
            .publish_diagnostics(params.text_document.uri.clone(), vec![], None)
            .await;
        self.documents.close(&params.text_document.uri);
    }

    // ── Completion ──

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let line = pos.line as usize;
        let col = pos.character as usize;

        // Get source content
        let source = self.get_source(uri);
        let source = match source {
            Some(s) => s,
            None => return Ok(None),
        };

        // Don't complete inside strings or comments
        if is_inside_string_or_comment(&source, line, col) {
            return Ok(None);
        }

        let context = parser::get_completion_context(&source, line, col);
        let mut items: Vec<CompletionItem> = Vec::new();

        match context {
            CompletionContext::DotAccess { object_text } => {
                // Find the type of the object and show its members
                self.complete_dot_access(&object_text, uri, &source, &mut items)
                    .await;
            }
            CompletionContext::ScopeResolution { scope_text } => {
                // Show enum values or class static members
                self.complete_scope_resolution(&scope_text, &mut items)
                    .await;
            }
            CompletionContext::General => {
                // Show everything: builtins + workspace symbols
                self.complete_general(uri, &mut items).await;
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    // ── Hover ──

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let line = pos.line as usize;
        let col = pos.character as usize;

        let source = self.get_source(uri);
        let word = source.and_then(|s| parser::extract_word_at(&s, line, col));

        let word = match word {
            Some(w) => w,
            None => return Ok(None),
        };

        // Check builtins first
        if let Some(func) = find_function(&word) {
            let mut value = format!("```mql5\n{}\n```", func.signature);
            if let Some(doc) = func.doc {
                value.push_str(&format!("\n\n{}", doc));
            }
            return Ok(Some(make_hover(value)));
        }

        if let Some(e) = find_enum(&word) {
            let values_preview: Vec<&str> = e.values.iter().take(10).copied().collect();
            let mut value = format!("```mql5\nenum {}\n```", e.name);
            value.push_str(&format!("\n\nValues: `{}`", values_preview.join("`, `")));
            if e.values.len() > 10 {
                value.push_str(&format!(" ... ({} total)", e.values.len()));
            }
            return Ok(Some(make_hover(value)));
        }

        if let Some(s) = find_struct(&word) {
            let mut value = format!("```mql5\nstruct {}\n```\n\nFields:", s.name);
            for (fname, ftype) in s.fields {
                value.push_str(&format!("\n- `{} {}`", ftype, fname));
            }
            return Ok(Some(make_hover(value)));
        }

        if let Some(c) = find_constant(&word) {
            let mut value = format!("```mql5\n#define {} {}\n```", c.name, c.value);
            if let Some(doc) = c.doc {
                value.push_str(&format!("\n\n{}", doc));
            }
            return Ok(Some(make_hover(value)));
        }

        // Check if it's a builtin enum value
        for e in BUILTIN_ENUMS {
            if e.values.contains(&word.as_str()) {
                return Ok(Some(make_hover(format!(
                    "```mql5\n{}::{}\n```\n\nMember of `enum {}`",
                    e.name, word, e.name
                ))));
            }
        }

        // Check workspace symbols
        let index = self.index.read().await;
        if let Some(sym) = index.find_symbol(&word) {
            let file_info = sym
                .uri
                .to_file_path()
                .ok()
                .map(|p| {
                    // Show relative path if possible
                    p.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                })
                .unwrap_or_default();

            let value = format!(
                "```mql5\n{}\n```\n\nDefined in `{}`",
                sym.detail, file_info
            );
            return Ok(Some(make_hover(value)));
        }

        Ok(None)
    }

    // ── Go to Definition ──

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let line = pos.line as usize;
        let col = pos.character as usize;

        let source = match self.get_source(uri) {
            Some(s) => s,
            None => return Ok(None),
        };

        // Check if cursor is on an #include line — navigate to included file
        if let Some(target_line) = source.lines().nth(line) {
            let trimmed = target_line.trim();
            if trimmed.starts_with("#include") {
                if let Some(inc) = parser::parse_include_from_line(trimmed, line as u32) {
                    if let Ok(path) = uri.to_file_path() {
                        let mut resolver = self.include_resolver.write().await;
                        if let Some(resolved) = resolver.resolve(&inc, &path) {
                            if let Ok(target_uri) = Url::from_file_path(&resolved) {
                                return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                                    uri: target_uri,
                                    range: Range {
                                        start: Position { line: 0, character: 0 },
                                        end: Position { line: 0, character: 0 },
                                    },
                                })));
                            }
                        }
                    }
                }
            }
        }

        let word = match parser::extract_word_at(&source, line, col) {
            Some(w) => w,
            None => return Ok(None),
        };

        let index = self.index.read().await;

        // Find all definitions
        let matches = index.find_symbols(&word);
        if matches.is_empty() {
            return Ok(None);
        }

        if matches.len() == 1 {
            return Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: matches[0].uri.clone(),
                range: matches[0].range,
            })));
        }

        // Multiple definitions — return all
        let locations: Vec<Location> = matches
            .into_iter()
            .map(|s| Location {
                uri: s.uri.clone(),
                range: s.range,
            })
            .collect();

        Ok(Some(GotoDefinitionResponse::Array(locations)))
    }

    // ── Find All References ──

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let line = pos.line as usize;
        let col = pos.character as usize;

        let source = self.get_source(uri);
        let word = source.and_then(|s| parser::extract_word_at(&s, line, col));

        let word = match word {
            Some(w) => w,
            None => return Ok(None),
        };

        let index = self.index.read().await;
        let mut locations: Vec<Location> = Vec::new();

        // Determine the scope of the identifier at cursor
        let scope = index.get_scope_at(uri, pos.line, pos.character)
            .unwrap_or_else(|| "global".to_string());

        // If the symbol has a definition at file scope, use global references
        // If it's a local variable (scope != "global"), use scoped references
        let is_global = scope == "global" || !index.find_symbols(&word).is_empty();

        // Include definitions if requested
        if params.context.include_declaration {
            for sym in index.find_symbols(&word) {
                locations.push(Location {
                    uri: sym.uri.clone(),
                    range: sym.range,
                });
            }
        }

        // Include references — scope-aware for locals
        let refs = if is_global {
            index.find_references(&word)
        } else {
            index.find_references_in_scope(&word, &scope)
        };
        for reference in refs {
            let loc = Location {
                uri: reference.uri.clone(),
                range: reference.range,
            };
            if !locations.iter().any(|l| l.uri == loc.uri && l.range == loc.range) {
                locations.push(loc);
            }
        }

        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(locations))
        }
    }

    // ── Rename ──

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        let uri = &params.text_document.uri;
        let pos = params.position;
        let line = pos.line as usize;
        let col = pos.character as usize;

        let source = self.get_source(uri);
        let source = match source {
            Some(s) => s,
            None => return Ok(None),
        };

        // Find the word at cursor and its range
        let target_line = match source.lines().nth(line) {
            Some(l) => l,
            None => return Ok(None),
        };
        let bytes = target_line.as_bytes();
        if col >= bytes.len() {
            return Ok(None);
        }

        let mut start = col;
        while start > 0 && is_ident_byte(bytes[start - 1]) {
            start -= 1;
        }
        let mut end = col;
        while end < bytes.len() && is_ident_byte(bytes[end]) {
            end += 1;
        }

        if start == end {
            return Ok(None);
        }

        let word = &target_line[start..end];

        // Check that the symbol exists (either as a definition or a reference)
        let index = self.index.read().await;
        let has_definition = !index.find_symbols(word).is_empty();
        let has_references = !index.find_references(word).is_empty();

        if !has_definition && !has_references {
            // Also check builtins — we cannot rename builtins
            if find_function(word).is_some()
                || find_enum(word).is_some()
                || find_struct(word).is_some()
                || find_constant(word).is_some()
            {
                return Ok(None); // Cannot rename builtins
            }
            return Ok(None);
        }

        Ok(Some(PrepareRenameResponse::Range(Range {
            start: Position {
                line: pos.line,
                character: start as u32,
            },
            end: Position {
                line: pos.line,
                character: end as u32,
            },
        })))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = &params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let line = pos.line as usize;
        let col = pos.character as usize;
        let new_name = &params.new_name;

        let source = self.get_source(uri);
        let word = source.and_then(|s| parser::extract_word_at(&s, line, col));

        let word = match word {
            Some(w) => w,
            None => return Ok(None),
        };

        let index = self.index.read().await;

        // Determine scope for rename — locals stay in scope, globals rename everywhere
        let scope = index.get_scope_at(uri, pos.line, pos.character)
            .unwrap_or_else(|| "global".to_string());
        let is_global = scope == "global" || !index.find_symbols(&word).is_empty();

        let mut file_edits: HashMap<Url, Vec<TextEdit>> = HashMap::new();

        // Definitions
        for sym in index.find_symbols(&word) {
            file_edits
                .entry(sym.uri.clone())
                .or_default()
                .push(TextEdit {
                    range: sym.range,
                    new_text: new_name.clone(),
                });
        }

        // References — scope-aware
        let refs = if is_global {
            index.find_references(&word)
        } else {
            index.find_references_in_scope(&word, &scope)
        };
        for reference in refs {
            file_edits
                .entry(reference.uri.clone())
                .or_default()
                .push(TextEdit {
                    range: reference.range,
                    new_text: new_name.clone(),
                });
        }

        if file_edits.is_empty() {
            return Ok(None);
        }

        let changes: HashMap<Url, Vec<TextEdit>> = file_edits;
        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }))
    }

    // ── Signature Help ──

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let line = pos.line as usize;
        let col = pos.character as usize;

        let source = match self.get_source(uri) {
            Some(s) => s,
            None => return Ok(None),
        };

        // Find function name and active parameter by scanning backwards from cursor
        let (func_name, active_param) = match find_function_call_context(&source, line, col) {
            Some(ctx) => ctx,
            None => return Ok(None),
        };

        // Look up the function signature
        let signature_str;
        let doc_str;

        if let Some(func) = find_function(&func_name) {
            signature_str = func.signature.to_string();
            doc_str = func.doc.map(|d| d.to_string());
        } else {
            // Check workspace symbols
            let index = self.index.read().await;
            let sym = index.find_symbol(&func_name);
            match sym {
                Some(s)
                    if matches!(
                        s.kind,
                        parser::ParsedSymbolKind::Function | parser::ParsedSymbolKind::Method
                    ) =>
                {
                    signature_str = s.detail.clone();
                    doc_str = None;
                }
                _ => return Ok(None),
            }
        }

        // Parse parameters from signature
        let params_list = parse_signature_params(&signature_str);

        let param_infos: Vec<ParameterInformation> = params_list
            .iter()
            .map(|p| ParameterInformation {
                label: ParameterLabel::Simple(p.clone()),
                documentation: None,
            })
            .collect();

        let sig = SignatureInformation {
            label: signature_str,
            documentation: doc_str.map(|d| {
                Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: d,
                })
            }),
            parameters: Some(param_infos),
            active_parameter: Some(active_param as u32),
        };

        Ok(Some(SignatureHelp {
            signatures: vec![sig],
            active_signature: Some(0),
            active_parameter: Some(active_param as u32),
        }))
    }

    // ── Inlay Hints ──

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let uri = &params.text_document.uri;
        let source = match self.get_source(uri) {
            Some(s) => s,
            None => return Ok(None),
        };

        let tree = match parser::parse(&source) {
            Some(t) => t,
            None => return Ok(None),
        };

        let range = params.range;
        let mut hints: Vec<InlayHint> = Vec::new();

        // Show return types for function definitions
        let symbols = parser::extract_symbols(&source, &tree);
        for sym in &symbols {
            // Only within the requested range
            if sym.start_line < range.start.line || sym.start_line > range.end.line {
                continue;
            }

            // For function calls that return a value assigned to a variable,
            // show the return type as an inlay hint
            match sym.kind {
                parser::ParsedSymbolKind::InputVar | parser::ParsedSymbolKind::Field => {
                    // Show type for variable declarations where type might not be obvious
                    // Skip — type is already visible in declaration
                }
                _ => {}
            }
        }

        // Show parameter names at call sites for builtin functions
        let calls = parser::extract_function_calls(&source, &tree);
        for call in &calls {
            if call.line < range.start.line || call.line > range.end.line {
                continue;
            }
            if call.is_method {
                continue;
            }

            if let Some(func) = find_function(&call.name) {
                let params_list = parse_signature_params(func.signature);
                // Show parameter name hints for calls with 2+ args
                if params_list.len() >= 2 && call.arg_count >= 2 {
                    // Extract argument positions from source
                    let call_args = parser::extract_call_arg_positions(&source, &tree, call.line, call.col);
                    for (i, (arg_line, arg_col)) in call_args.iter().enumerate() {
                        if i >= params_list.len() {
                            break;
                        }
                        // Extract just the parameter name from "type name" or "type name = default"
                        let param = &params_list[i];
                        let param_name = param
                            .split('=').next().unwrap_or(param)
                            .trim()
                            .split_whitespace().last()
                            .unwrap_or("")
                            .trim_start_matches('&')
                            .trim_start_matches('*');
                        if param_name.is_empty() || param_name == "..." {
                            continue;
                        }
                        hints.push(InlayHint {
                            position: Position {
                                line: *arg_line,
                                character: *arg_col,
                            },
                            label: InlayHintLabel::String(format!("{}:", param_name)),
                            kind: Some(InlayHintKind::PARAMETER),
                            text_edits: None,
                            tooltip: None,
                            padding_left: Some(false),
                            padding_right: Some(true),
                            data: None,
                        });
                    }
                }
            }
        }

        if hints.is_empty() {
            Ok(None)
        } else {
            Ok(Some(hints))
        }
    }

    // ── Code Actions ──

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        let mut actions: Vec<CodeActionOrCommand> = Vec::new();

        for diag in &params.context.diagnostics {
            if diag.source.as_deref() != Some("mql5-lsp") {
                continue;
            }

            // Unresolved include → suggest creating file
            if diag.message.starts_with("Unresolved include:") {
                let include_path = diag.message.strip_prefix("Unresolved include: ").unwrap_or("");
                if !include_path.is_empty() {
                    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                        title: format!("Create {}", include_path),
                        kind: Some(CodeActionKind::QUICKFIX),
                        diagnostics: Some(vec![diag.clone()]),
                        ..Default::default()
                    }));
                }
            }

            // Unknown function → suggest adding #include if found in another file
            if diag.message.starts_with("Unknown function") {
                let func_name = diag.message
                    .strip_prefix("Unknown function `")
                    .and_then(|s| s.split('`').next());

                if let Some(func_name) = func_name {
                    let index = self.index.read().await;
                    if let Some(sym) = index.find_symbol(func_name) {
                        // Compute relative include path
                        if let (Ok(current_path), Ok(sym_path)) =
                            (uri.to_file_path(), sym.uri.to_file_path())
                        {
                            if let Some(include_text) =
                                compute_include_directive(&current_path, &sym_path)
                            {
                                // Find the line to insert (after last #include, or line 0)
                                let insert_line = self
                                    .get_source(uri)
                                    .map(|s| find_include_insert_line(&s))
                                    .unwrap_or(0);

                                let mut changes = HashMap::new();
                                changes.insert(
                                    uri.clone(),
                                    vec![TextEdit {
                                        range: Range {
                                            start: Position {
                                                line: insert_line,
                                                character: 0,
                                            },
                                            end: Position {
                                                line: insert_line,
                                                character: 0,
                                            },
                                        },
                                        new_text: format!("{}\n", include_text),
                                    }],
                                );

                                actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                                    title: format!(
                                        "Add #include for `{}`",
                                        sym_path
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy()
                                    ),
                                    kind: Some(CodeActionKind::QUICKFIX),
                                    diagnostics: Some(vec![diag.clone()]),
                                    edit: Some(WorkspaceEdit {
                                        changes: Some(changes),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                }));
                            }
                        }
                    }
                }
            }
        }

        if actions.is_empty() {
            Ok(None)
        } else {
            Ok(Some(actions))
        }
    }

    // ── Document Formatting ──

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;
        let source = match self.get_source(uri) {
            Some(s) => s,
            None => return Ok(None),
        };

        let formatted = formatter::format_mql5(&source);

        if formatted == source {
            return Ok(None); // No changes needed
        }

        // Replace entire document
        let line_count = source.lines().count() as u32;
        let last_line_len = source.lines().last().map(|l| l.len()).unwrap_or(0) as u32;

        Ok(Some(vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: line_count,
                    character: last_line_len,
                },
            },
            new_text: formatted,
        }]))
    }

    // ── Semantic Tokens ──

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = &params.text_document.uri;
        let source = match self.get_source(uri) {
            Some(s) => s,
            None => return Ok(None),
        };

        let tree = match parser::parse(&source) {
            Some(t) => t,
            None => return Ok(None),
        };

        let idents = parser::extract_identifiers_scoped(&source, &tree);
        let index = self.index.read().await;

        let mut tokens: Vec<(u32, u32, u32, u32, u32)> = Vec::new(); // (line, col, len, type, modifiers)

        for id in &idents {
            let len = id.end_col - id.start_col;
            if len == 0 { continue; }

            // Classify the identifier
            let (token_type, modifiers) = classify_identifier(&id.name, &id.scope, &index);

            // Skip unclassified identifiers (they get tree-sitter highlighting)
            if token_type == u32::MAX { continue; }

            tokens.push((id.line, id.start_col, len, token_type, modifiers));
        }

        // Sort by position
        tokens.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        // Encode as delta-encoded semantic tokens
        let mut data = Vec::with_capacity(tokens.len() * 5);
        let mut prev_line = 0u32;
        let mut prev_col = 0u32;

        for (line, col, len, token_type, modifiers) in &tokens {
            let delta_line = line - prev_line;
            let delta_col = if delta_line == 0 { col - prev_col } else { *col };

            data.push(SemanticToken {
                delta_line,
                delta_start: delta_col,
                length: *len,
                token_type: *token_type,
                token_modifiers_bitset: *modifiers,
            });

            prev_line = *line;
            prev_col = *col;
        }

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        })))
    }

    // ── Document Colors ──

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        let uri = &params.text_document.uri;
        let source = match self.get_source(uri) {
            Some(s) => s,
            None => return Ok(vec![]),
        };

        let mut colors = Vec::new();

        // Find all C'r,g,b' color literals in the source
        // MQL5 color literals are RGB
        let bytes = source.as_bytes();
        let mut i = 0;
        while i < bytes.len().saturating_sub(6) {
            if bytes[i] == b'C' && bytes.get(i + 1) == Some(&b'\'') {
                // Try to parse C'r,g,b'
                if let Some((r, g, b, end)) = parse_color_literal_at(bytes, i + 2) {
                    let (start_line, start_col) = byte_to_position(&source, i);
                    let (end_line, end_col) = byte_to_position(&source, end);

                    colors.push(ColorInformation {
                        range: Range {
                            start: Position { line: start_line, character: start_col },
                            end: Position { line: end_line, character: end_col },
                        },
                        color: Color {
                            red: r as f32 / 255.0,
                            green: g as f32 / 255.0,
                            blue: b as f32 / 255.0,
                            alpha: 1.0,
                        },
                    });
                    i = end;
                    continue;
                }
            }

            // Also detect MQL5 named color constants (clrRed, clrBlue, etc.)
            if bytes[i..].starts_with(b"clr") {
                if let Some((r, g, b, name_end)) = parse_named_color(&source, i) {
                    let (start_line, start_col) = byte_to_position(&source, i);
                    let (end_line, end_col) = byte_to_position(&source, name_end);

                    colors.push(ColorInformation {
                        range: Range {
                            start: Position { line: start_line, character: start_col },
                            end: Position { line: end_line, character: end_col },
                        },
                        color: Color {
                            red: r as f32 / 255.0,
                            green: g as f32 / 255.0,
                            blue: b as f32 / 255.0,
                            alpha: 1.0,
                        },
                    });
                    i = name_end;
                    continue;
                }
            }

            // Hex color literals: 0xRRGGBB or 0xAARRGGBB
            if bytes[i..].starts_with(b"0x") || bytes[i..].starts_with(b"0X") {
                if let Some((r, g, b, a, hex_end)) = parse_hex_color(&source, i) {
                    let (start_line, start_col) = byte_to_position(&source, i);
                    let (end_line, end_col) = byte_to_position(&source, hex_end);

                    colors.push(ColorInformation {
                        range: Range {
                            start: Position { line: start_line, character: start_col },
                            end: Position { line: end_line, character: end_col },
                        },
                        color: Color {
                            red: r as f32 / 255.0,
                            green: g as f32 / 255.0,
                            blue: b as f32 / 255.0,
                            alpha: a as f32 / 255.0,
                        },
                    });
                    i = hex_end;
                    continue;
                }
            }

            i += 1;
        }

        Ok(colors)
    }

    async fn color_presentation(
        &self,
        params: ColorPresentationParams,
    ) -> Result<Vec<ColorPresentation>> {
        let color = params.color;
        let r = (color.red * 255.0).round() as u8;
        let g = (color.green * 255.0).round() as u8;
        let b = (color.blue * 255.0).round() as u8;

        Ok(vec![ColorPresentation {
            label: format!("C'{},{},{}'", r, g, b),
            text_edit: Some(TextEdit {
                range: params.range,
                new_text: format!("C'{},{},{}'", r, g, b),
            }),
            additional_text_edits: None,
        }])
    }

    // ── Document Symbols ──

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;

        let source = self.get_source(uri);
        let source = match source {
            Some(s) => s,
            None => return Ok(None),
        };

        let tree = match parser::parse(&source) {
            Some(t) => t,
            None => return Ok(None),
        };

        let parsed = parser::extract_symbols(&source, &tree);
        let items: Vec<SymbolInformation> = parsed
            .into_iter()
            .map(|s| {
                let kind = match s.kind {
                    parser::ParsedSymbolKind::Function => SymbolKind::FUNCTION,
                    parser::ParsedSymbolKind::Class => SymbolKind::CLASS,
                    parser::ParsedSymbolKind::Struct => SymbolKind::STRUCT,
                    parser::ParsedSymbolKind::Enum => SymbolKind::ENUM,
                    parser::ParsedSymbolKind::EnumValue => SymbolKind::ENUM_MEMBER,
                    parser::ParsedSymbolKind::Define => SymbolKind::CONSTANT,
                    parser::ParsedSymbolKind::InputVar => SymbolKind::PROPERTY,
                    parser::ParsedSymbolKind::GlobalVar => SymbolKind::VARIABLE,
                    parser::ParsedSymbolKind::Method => SymbolKind::METHOD,
                    parser::ParsedSymbolKind::Field => SymbolKind::FIELD,
                    parser::ParsedSymbolKind::TypeAlias => SymbolKind::TYPE_PARAMETER,
                };
                #[allow(deprecated)]
                SymbolInformation {
                    name: s.name,
                    kind,
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position {
                                line: s.start_line,
                                character: 0,
                            },
                            end: Position {
                                line: s.end_line,
                                character: 1000,
                            },
                        },
                    },
                    container_name: s.parent_name,
                }
            })
            .collect();

        Ok(Some(DocumentSymbolResponse::Flat(items)))
    }

    // ── Workspace Symbols ──

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        let query = params.query.to_lowercase();
        let index = self.index.read().await;

        let results: Vec<SymbolInformation> = index
            .all_symbols()
            .filter(|s| {
                if query.is_empty() {
                    true
                } else {
                    s.name.to_lowercase().contains(&query)
                }
            })
            .take(100)
            .map(|s| {
                #[allow(deprecated)]
                SymbolInformation {
                    name: s.name.clone(),
                    kind: s.symbol_kind(),
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: s.uri.clone(),
                        range: s.range,
                    },
                    container_name: s.parent_name.clone(),
                }
            })
            .collect();

        Ok(Some(results))
    }
}

// ── Helper implementations ──

impl Mql5Lsp {
    /// Get source content for a URI — prefers in-memory document, falls back to disk.
    fn get_source(&self, uri: &Url) -> Option<String> {
        self.documents
            .get_text(uri)
            .or_else(|| uri.to_file_path().ok().and_then(|p| std::fs::read_to_string(p).ok()))
    }

    /// Index files that a source file includes.
    async fn index_includes(
        &self,
        path: &PathBuf,
        source: &str,
        index: &mut SymbolIndex,
    ) {
        let mut resolver = self.include_resolver.write().await;
        let included = resolver.get_transitive_includes(
            path,
            source,
            &|p| std::fs::read_to_string(p).ok(),
        );

        for inc_path in included {
            index.index_file_from_disk(&inc_path);
        }
    }

    /// Publish diagnostics for a document.
    async fn publish_diagnostics_for(&self, uri: &Url) {
        let source = match self.get_source(uri) {
            Some(s) => s,
            None => return,
        };

        let tree = match parser::parse(&source) {
            Some(t) => t,
            None => return,
        };

        let mut diagnostics: Vec<Diagnostic> = Vec::new();

        // (a) Syntax errors from tree-sitter ERROR/MISSING nodes
        {
            let source_lines: Vec<&str> = source.lines().collect();
            let errors = parser::extract_errors(&source, &tree);
            for err in errors {
                // Suppress known MQL5 constructs that C++ grammar can't parse
                let line_text = source_lines.get(err.start_line as usize).unwrap_or(&"");
                let trimmed = line_text.trim();

                // input group "..." — MQL5 parameter grouping
                if trimmed.starts_with("input group") { continue; }
                // sinput group "..."
                if trimmed.starts_with("sinput group") { continue; }
                // #property — MQL5 preprocessor directive
                if trimmed.starts_with("#property") { continue; }
                // #import "..." — MQL5 DLL import
                if trimmed.starts_with("#import") { continue; }
                // #resource "..." — embedded resource
                if trimmed.starts_with("#resource") { continue; }
                // operator overloads with MQL5 syntax that C++ doesn't like
                if trimmed.contains("operator") && err.message.contains("Syntax error") { continue; }

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: err.start_line, character: err.start_col },
                        end: Position { line: err.end_line, character: err.end_col },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("mql5-lsp".to_string()),
                    message: err.message,
                    ..Default::default()
                });
            }
        }

        // (b) Unresolved includes
        {
            let includes = parser::extract_includes(&source, &tree);
            if let Ok(path) = uri.to_file_path() {
                let mut resolver = self.include_resolver.write().await;
                for inc in &includes {
                    if resolver.resolve(inc, &path).is_none() {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: inc.line,
                                    character: 0,
                                },
                                end: Position {
                                    line: inc.line,
                                    character: 1000,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("mql5-lsp".to_string()),
                            message: format!(
                                "Unresolved include: {}",
                                inc.path
                            ),
                            ..Default::default()
                        });
                    }
                }
            }
        }

        // (c) Duplicate definitions within the same file
        {
            let parsed = parser::extract_symbols(&source, &tree);
            let mut seen: HashMap<String, u32> = HashMap::new();
            for sym in &parsed {
                // Only check top-level symbols (no parent) and non-methods
                if sym.parent_name.is_none()
                    && !matches!(sym.kind, parser::ParsedSymbolKind::EnumValue)
                {
                    if let Some(prev_line) = seen.get(&sym.name) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: sym.start_line,
                                    character: 0,
                                },
                                end: Position {
                                    line: sym.start_line,
                                    character: 1000,
                                },
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            source: Some("mql5-lsp".to_string()),
                            message: format!(
                                "Duplicate definition of `{}` (first defined on line {})",
                                sym.name,
                                prev_line + 1
                            ),
                            ..Default::default()
                        });
                    } else {
                        seen.insert(sym.name.clone(), sym.start_line);
                    }
                }
            }
        }

        // (d) Function call diagnostics: undeclared + wrong argument count
        {
            let calls = parser::extract_function_calls(&source, &tree);
            let index = self.index.read().await;
            for call in &calls {
                // Skip C/C++ keywords that look like calls
                if matches!(call.name.as_str(),
                    "if" | "for" | "while" | "switch" | "return" | "else" | "do"
                    | "sizeof" | "delete" | "new" | "typename" | "template"
                ) {
                    continue;
                }

                // Check if it's a builtin function — verify argument count
                // Skip method calls since the same name may have different signatures
                if let Some(func) = find_function(&call.name).filter(|_| !call.is_method) {
                    let expected = count_signature_params(func.signature);
                    // Only check if we have a definite expected count and call has args
                    if expected.min > 0 && call.arg_count < expected.min {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: call.line, character: call.col },
                                end: Position { line: call.line, character: call.col + call.name.len() as u32 },
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            source: Some("mql5-lsp".to_string()),
                            message: format!(
                                "`{}` expects at least {} argument(s), got {}",
                                call.name, expected.min, call.arg_count
                            ),
                            ..Default::default()
                        });
                    } else if expected.max > 0 && call.arg_count > expected.max {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: call.line, character: call.col },
                                end: Position { line: call.line, character: call.col + call.name.len() as u32 },
                            },
                            severity: Some(DiagnosticSeverity::WARNING),
                            source: Some("mql5-lsp".to_string()),
                            message: format!(
                                "`{}` expects at most {} argument(s), got {}",
                                call.name, expected.max, call.arg_count
                            ),
                            ..Default::default()
                        });
                    }
                    continue;
                }

                // For method calls, try to resolve the receiver type and check members
                if call.is_method {
                    if let Some(ref receiver) = call.receiver {
                        // Resolve receiver type
                        if let Some(type_name) = parser::resolve_type_at(
                            &source, &tree, receiver, call.line as usize,
                        ) {
                            // Check if method exists on that type
                            let members = index.find_members(&type_name);
                            let has_method = members.iter().any(|m| m.name == call.name);
                            if !has_method {
                                // Also check builtin struct fields
                                let has_builtin = find_struct(&type_name)
                                    .map(|s| s.fields.iter().any(|(f, _)| *f == call.name))
                                    .unwrap_or(false);
                                if !has_builtin {
                                    diagnostics.push(Diagnostic {
                                        range: Range {
                                            start: Position { line: call.line, character: call.col },
                                            end: Position { line: call.line, character: call.col + call.name.len() as u32 },
                                        },
                                        severity: Some(DiagnosticSeverity::HINT),
                                        source: Some("mql5-lsp".to_string()),
                                        message: format!(
                                            "Method `{}` not found on type `{}`",
                                            call.name, type_name
                                        ),
                                        ..Default::default()
                                    });
                                }
                            }
                        }
                    }
                    continue;
                }
                // Skip known types/constants/enums
                if find_enum(&call.name).is_some()
                    || find_struct(&call.name).is_some()
                    || find_constant(&call.name).is_some()
                    || is_builtin_type(&call.name)
                {
                    continue;
                }
                // Skip if found in workspace index (as definition or member)
                if !index.find_symbols(&call.name).is_empty() {
                    continue;
                }
                // Skip MQL5 event handlers
                if call.name.starts_with("On") && MQL5_KEYWORDS.contains(&call.name.as_str()) {
                    continue;
                }

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { line: call.line, character: call.col },
                        end: Position { line: call.line, character: call.col + call.name.len() as u32 },
                    },
                    severity: Some(DiagnosticSeverity::HINT),
                    source: Some("mql5-lsp".to_string()),
                    message: format!(
                        "Unknown function `{}` — not in builtins or workspace index",
                        call.name
                    ),
                    ..Default::default()
                });
            }
        }

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }

    /// Completion after `.` — show members of the type.
    async fn complete_dot_access(
        &self,
        object_text: &str,
        uri: &Url,
        source: &str,
        items: &mut Vec<CompletionItem>,
    ) {
        // Try to find what type the object is
        let type_name = self.resolve_variable_type(object_text, uri, source).await;

        if let Some(ref type_name) = type_name {
            // Show members from workspace symbols
            let index = self.index.read().await;
            let members = index.find_members(type_name);
            for member in members {
                items.push(CompletionItem {
                    label: member.name.clone(),
                    kind: Some(member.completion_kind()),
                    detail: Some(member.detail.clone()),
                    ..Default::default()
                });
            }

            // Show members from builtin structs
            if let Some(s) = find_struct(type_name) {
                for (fname, ftype) in s.fields {
                    items.push(CompletionItem {
                        label: fname.to_string(),
                        kind: Some(CompletionItemKind::FIELD),
                        detail: Some(format!("{} {}", ftype, fname)),
                        ..Default::default()
                    });
                }
            }
        }
    }

    /// Completion after `::` — show enum values or class statics.
    async fn complete_scope_resolution(
        &self,
        scope_text: &str,
        items: &mut Vec<CompletionItem>,
    ) {
        // Check builtin enums
        if let Some(e) = find_enum(scope_text) {
            for val in e.values {
                items.push(CompletionItem {
                    label: val.to_string(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some(format!("{}::{}", e.name, val)),
                    ..Default::default()
                });
            }
        }

        // Check workspace symbols for class members
        let index = self.index.read().await;
        let members = index.find_members(scope_text);
        for member in members {
            items.push(CompletionItem {
                label: member.name.clone(),
                kind: Some(member.completion_kind()),
                detail: Some(member.detail.clone()),
                ..Default::default()
            });
        }
    }

    /// General completion — show all available symbols.
    async fn complete_general(&self, uri: &Url, items: &mut Vec<CompletionItem>) {
        // Builtin functions
        for func in BUILTIN_FUNCTIONS {
            items.push(CompletionItem {
                label: func.name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(func.signature.to_string()),
                documentation: func.doc.map(|d| {
                    Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: d.to_string(),
                    })
                }),
                ..Default::default()
            });
        }

        // Builtin enums (as types)
        for e in BUILTIN_ENUMS {
            items.push(CompletionItem {
                label: e.name.to_string(),
                kind: Some(CompletionItemKind::ENUM),
                detail: Some(format!("enum {} ({} values)", e.name, e.values.len())),
                ..Default::default()
            });
            // Also add enum values directly
            for val in e.values {
                items.push(CompletionItem {
                    label: val.to_string(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    detail: Some(format!("{}::{}", e.name, val)),
                    ..Default::default()
                });
            }
        }

        // Builtin structs
        for s in BUILTIN_STRUCTS {
            items.push(CompletionItem {
                label: s.name.to_string(),
                kind: Some(CompletionItemKind::STRUCT),
                detail: Some(format!("struct {}", s.name)),
                ..Default::default()
            });
        }

        // Builtin constants
        for c in BUILTIN_CONSTANTS {
            items.push(CompletionItem {
                label: c.name.to_string(),
                kind: Some(CompletionItemKind::CONSTANT),
                detail: Some(c.value.to_string()),
                ..Default::default()
            });
        }

        // Builtin globals
        for (name, type_name) in BUILTIN_GLOBALS {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some(format!("{} {}", type_name, name)),
                ..Default::default()
            });
        }

        // MQL5 keywords
        for kw in MQL5_KEYWORDS {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }

        // Workspace symbols — with auto-import for symbols from other files
        let index = self.index.read().await;
        let current_source = self.get_source(uri).unwrap_or_default();
        let current_includes: Vec<String> = current_source
            .lines()
            .filter(|l| l.trim().starts_with("#include"))
            .map(|l| l.to_string())
            .collect();

        for sym in index.all_symbols() {
            // Skip members — they'll appear via dot completion
            if sym.parent_name.is_some() {
                continue;
            }

            let mut item = CompletionItem {
                label: sym.name.clone(),
                kind: Some(sym.completion_kind()),
                detail: Some(sym.detail.clone()),
                ..Default::default()
            };

            // If symbol is from a different file, check if we need to auto-import
            if sym.uri != *uri {
                if let (Ok(current_path), Ok(sym_path)) =
                    (uri.to_file_path(), sym.uri.to_file_path())
                {
                    if let Some(include_text) =
                        compute_include_directive(&current_path, &sym_path)
                    {
                        // Only add if not already included
                        let already_included = current_includes.iter().any(|inc| {
                            inc.contains(
                                &sym_path
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string(),
                            )
                        });

                        if !already_included {
                            let insert_line = find_include_insert_line(&current_source);
                            item.additional_text_edits = Some(vec![TextEdit {
                                range: Range {
                                    start: Position {
                                        line: insert_line,
                                        character: 0,
                                    },
                                    end: Position {
                                        line: insert_line,
                                        character: 0,
                                    },
                                },
                                new_text: format!("{}\n", include_text),
                            }]);
                            item.detail = Some(format!(
                                "{} (auto-import)",
                                sym.detail
                            ));
                        }
                    }
                }
            }

            items.push(item);
        }
    }

    /// Try to resolve the type of a variable by looking at declarations.
    /// Uses AST-based resolution first, then falls back to workspace index.
    async fn resolve_variable_type(
        &self,
        var_name: &str,
        _uri: &Url,
        source: &str,
    ) -> Option<String> {
        // AST-based: parse the source and walk declarations
        if let Some(tree) = parser::parse(source) {
            // Use line count as "use_line" to search the whole file
            let use_line = source.lines().count();
            if let Some(t) = parser::resolve_type_at(source, &tree, var_name, use_line) {
                return Some(t);
            }
        }

        // Fallback: check workspace symbols for the variable's type
        let index = self.index.read().await;
        if let Some(sym) = index.find_symbol(var_name) {
            // Parse type from detail string
            let detail = &sym.detail;
            let type_name = detail.split_whitespace().next();
            if let Some(t) = type_name {
                let t = t.trim_end_matches('*').trim_end_matches('&');
                if t != var_name {
                    return Some(t.to_string());
                }
            }
        }

        None
    }
}

// ── Free functions ──

fn make_hover(value: String) -> Hover {
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }),
        range: None,
    }
}

/// Classify an identifier for semantic token highlighting.
/// Returns (token_type_index, modifier_bitset). u32::MAX means skip.
fn classify_identifier(name: &str, scope: &str, index: &SymbolIndex) -> (u32, u32) {
    const TY_VARIABLE: u32 = 0;
    const _TY_PARAMETER: u32 = 1;
    const TY_FUNCTION: u32 = 2;
    const TY_METHOD: u32 = 3;
    const TY_PROPERTY: u32 = 4;
    const TY_CLASS: u32 = 5;
    const TY_ENUM: u32 = 6;
    const TY_ENUM_MEMBER: u32 = 7;
    const TY_MACRO: u32 = 8;
    const _TY_TYPE: u32 = 9;
    const MOD_BUILTIN: u32 = 1 << 4; // DEFAULT_LIBRARY

    // Skip keywords and types — tree-sitter handles these
    if matches!(name,
        "if" | "else" | "for" | "while" | "do" | "switch" | "case" | "default"
        | "return" | "break" | "continue" | "void" | "int" | "double" | "float"
        | "bool" | "char" | "string" | "long" | "short" | "uchar" | "ushort"
        | "uint" | "ulong" | "datetime" | "color" | "true" | "false" | "NULL"
        | "class" | "struct" | "enum" | "virtual" | "override" | "const" | "static"
        | "public" | "private" | "protected" | "new" | "delete" | "this" | "sizeof"
        | "input" | "sinput" | "template" | "typename" | "extern" | "typedef"
    ) {
        return (u32::MAX, 0);
    }

    // Check builtins
    if find_function(name).is_some() {
        return (TY_FUNCTION, MOD_BUILTIN);
    }
    if find_enum(name).is_some() {
        return (TY_ENUM, MOD_BUILTIN);
    }
    if find_struct(name).is_some() {
        return (TY_CLASS, MOD_BUILTIN);
    }
    // Check enum values across all enums
    for e in BUILTIN_ENUMS {
        if e.values.contains(&name) {
            return (TY_ENUM_MEMBER, MOD_BUILTIN);
        }
    }
    if find_constant(name).is_some() {
        return (TY_MACRO, MOD_BUILTIN);
    }

    // Check workspace symbols
    if let Some(sym) = index.find_symbol(name) {
        return match sym.kind {
            parser::ParsedSymbolKind::Function => (TY_FUNCTION, 0),
            parser::ParsedSymbolKind::Method => (TY_METHOD, 0),
            parser::ParsedSymbolKind::Class => (TY_CLASS, 0),
            parser::ParsedSymbolKind::Struct => (TY_CLASS, 0),
            parser::ParsedSymbolKind::Enum => (TY_ENUM, 0),
            parser::ParsedSymbolKind::EnumValue => (TY_ENUM_MEMBER, 0),
            parser::ParsedSymbolKind::Define => (TY_MACRO, 0),
            parser::ParsedSymbolKind::InputVar => (TY_PROPERTY, 0),
            parser::ParsedSymbolKind::Field => (TY_PROPERTY, 0),
            parser::ParsedSymbolKind::GlobalVar => (TY_VARIABLE, 0),
            parser::ParsedSymbolKind::TypeAlias => (TY_CLASS, 0),
        };
    }

    // Local variable (not found globally, has a function scope)
    if scope != "global" && !name.is_empty() {
        if name.chars().next().is_some_and(|c| c.is_lowercase()) {
            return (TY_VARIABLE, 0);
        }
    }

    // Skip — let tree-sitter handle it
    (u32::MAX, 0)
}

/// Check if cursor position is inside a string literal or comment.
fn is_inside_string_or_comment(source: &str, line: usize, col: usize) -> bool {
    let target_line = match source.lines().nth(line) {
        Some(l) => l,
        None => return false,
    };
    let before = if col <= target_line.len() { &target_line[..col] } else { target_line };

    // Check for line comment: // before cursor
    if let Some(slash_pos) = before.find("//") {
        // Make sure it's not inside a string
        let before_slash = &before[..slash_pos];
        let quote_count = before_slash.chars().filter(|&c| c == '"').count();
        if quote_count % 2 == 0 {
            return true; // Inside line comment
        }
    }

    // Check for string: count unescaped quotes before cursor
    let mut in_string = false;
    let mut prev_backslash = false;
    for ch in before.chars() {
        if ch == '"' && !prev_backslash {
            in_string = !in_string;
        }
        prev_backslash = ch == '\\';
    }
    if in_string {
        return true;
    }

    // Check for single-quoted char literal
    let mut in_char = false;
    for ch in before.chars() {
        if ch == '\'' && !prev_backslash {
            in_char = !in_char;
        }
        prev_backslash = ch == '\\';
    }
    in_char
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Convert a byte offset to (line, character) position.
fn byte_to_position(source: &str, byte_offset: usize) -> (u32, u32) {
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

/// Parse r,g,b values from bytes starting after C'
/// Returns (r, g, b, end_byte_offset_after_closing_quote)
fn parse_color_literal_at(bytes: &[u8], start: usize) -> Option<(u8, u8, u8, usize)> {
    let mut i = start;
    let mut values = [0u16; 3];
    let mut val_idx = 0;
    let mut has_digit = false;

    while i < bytes.len() {
        match bytes[i] {
            b'0'..=b'9' => {
                values[val_idx] = values[val_idx] * 10 + (bytes[i] - b'0') as u16;
                if values[val_idx] > 255 { return None; }
                has_digit = true;
            }
            b',' => {
                if !has_digit || val_idx >= 2 { return None; }
                val_idx += 1;
                has_digit = false;
            }
            b'\'' => {
                if !has_digit || val_idx != 2 { return None; }
                return Some((values[0] as u8, values[1] as u8, values[2] as u8, i + 1));
            }
            _ => return None,
        }
        i += 1;
    }
    None
}

/// Parse hex color: 0xRRGGBB (6 digits) or 0xAARRGGBB (8 digits).
/// Returns (r, g, b, a, end_byte_offset). Only matches exactly 6 or 8 hex digits.
fn parse_hex_color(source: &str, start: usize) -> Option<(u8, u8, u8, u8, usize)> {
    let rest = &source[start..];
    if !rest.starts_with("0x") && !rest.starts_with("0X") {
        return None;
    }

    let hex_start = start + 2;
    let hex_bytes = source[hex_start..].as_bytes();
    let mut hex_len = 0;
    while hex_len < hex_bytes.len() && hex_bytes[hex_len].is_ascii_hexdigit() {
        hex_len += 1;
    }

    // Only match 6 (RRGGBB) or 8 (AARRGGBB) hex digits
    // Don't match if followed by more alphanumeric chars (it's a different constant)
    if hex_len != 6 && hex_len != 8 {
        return None;
    }
    if hex_start + hex_len < source.len() {
        let next = source.as_bytes()[hex_start + hex_len];
        if next.is_ascii_alphanumeric() || next == b'_' {
            return None; // Part of a larger identifier/number
        }
    }

    let hex_str = &source[hex_start..hex_start + hex_len];
    let value = u32::from_str_radix(hex_str, 16).ok()?;

    let (r, g, b, a) = if hex_len == 8 {
        // 0xAARRGGBB
        let a = ((value >> 24) & 0xFF) as u8;
        let r = ((value >> 16) & 0xFF) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = (value & 0xFF) as u8;
        (r, g, b, a)
    } else {
        // 0xRRGGBB
        let r = ((value >> 16) & 0xFF) as u8;
        let g = ((value >> 8) & 0xFF) as u8;
        let b = (value & 0xFF) as u8;
        (r, g, b, 255)
    };

    Some((r, g, b, a, hex_start + hex_len))
}

/// Expected argument count range for a function.
struct ExpectedArgs {
    min: usize,
    max: usize, // 0 = unlimited (variadic)
}

/// Count expected parameters from a signature like "void Foo(int a, string b = \"x\")".
/// Parameters with `= default` are optional, reducing min count.
/// Variadic `...` makes max unlimited (0).
fn count_signature_params(signature: &str) -> ExpectedArgs {
    let start = match signature.find('(') {
        Some(p) => p + 1,
        None => return ExpectedArgs { min: 0, max: 0 },
    };
    let end = match signature.rfind(')') {
        Some(p) => p,
        None => return ExpectedArgs { min: 0, max: 0 },
    };
    if start >= end {
        return ExpectedArgs { min: 0, max: 0 };
    }

    let params_str = &signature[start..end].trim();
    if params_str.is_empty() {
        return ExpectedArgs { min: 0, max: 0 };
    }

    // Variadic functions — unlimited args
    if params_str.contains("...") {
        return ExpectedArgs { min: 0, max: 0 };
    }

    // Split by commas (respecting nested parens)
    let mut total = 0usize;
    let mut optional = 0usize;
    let mut depth = 0i32;
    let mut current = String::new();

    for ch in params_str.chars() {
        match ch {
            '(' | '<' | '[' => { depth += 1; current.push(ch); }
            ')' | '>' | ']' => { depth -= 1; current.push(ch); }
            ',' if depth == 0 => {
                if current.contains('=') { optional += 1; }
                total += 1;
                current.clear();
            }
            _ => { current.push(ch); }
        }
    }
    // Last parameter
    if !current.trim().is_empty() {
        if current.contains('=') { optional += 1; }
        total += 1;
    }

    ExpectedArgs {
        min: total.saturating_sub(optional),
        max: total,
    }
}

/// Parse a named MQL5 color constant (full web/X11 color set).
/// Returns (r, g, b, end_byte_offset)
fn parse_named_color(source: &str, start: usize) -> Option<(u8, u8, u8, usize)> {
    let rest = &source[start..];
    let end = rest.find(|c: char| !c.is_ascii_alphanumeric() && c != '_').unwrap_or(rest.len());
    let name = &rest[..end];

    let (r, g, b) = match name {
        // Reds
        "clrRed" => (255, 0, 0),
        "clrDarkRed" => (139, 0, 0),
        "clrIndianRed" => (205, 92, 92),
        "clrLightCoral" => (240, 128, 128),
        "clrSalmon" => (250, 128, 114),
        "clrDarkSalmon" => (233, 150, 122),
        "clrLightSalmon" => (255, 160, 122),
        "clrCrimson" => (220, 20, 60),
        "clrFireBrick" => (178, 34, 34),
        // Pinks
        "clrPink" => (255, 192, 203),
        "clrLightPink" => (255, 182, 193),
        "clrHotPink" => (255, 105, 180),
        "clrDeepPink" => (255, 20, 147),
        "clrMediumVioletRed" => (199, 21, 133),
        "clrPaleVioletRed" => (219, 112, 147),
        // Oranges
        "clrOrange" => (255, 165, 0),
        "clrDarkOrange" => (255, 140, 0),
        "clrOrangeRed" => (255, 69, 0),
        "clrTomato" => (255, 99, 71),
        "clrCoral" => (255, 127, 80),
        // Yellows
        "clrYellow" => (255, 255, 0),
        "clrLightYellow" => (255, 255, 224),
        "clrLemonChiffon" => (255, 250, 205),
        "clrLightGoldenrodYellow" => (250, 250, 210),
        "clrPapayaWhip" => (255, 239, 213),
        "clrMoccasin" => (255, 228, 181),
        "clrPeachPuff" => (255, 218, 185),
        "clrPaleGoldenrod" => (238, 232, 170),
        "clrKhaki" => (240, 230, 140),
        "clrDarkKhaki" => (189, 183, 107),
        "clrGold" => (255, 215, 0),
        // Purples
        "clrLavender" => (230, 230, 250),
        "clrThistle" => (216, 191, 216),
        "clrPlum" => (221, 160, 221),
        "clrViolet" => (238, 130, 238),
        "clrOrchid" => (218, 112, 214),
        "clrFuchsia" | "clrMagenta" => (255, 0, 255),
        "clrMediumOrchid" => (186, 85, 211),
        "clrMediumPurple" => (147, 112, 219),
        "clrBlueViolet" => (138, 43, 226),
        "clrDarkViolet" => (148, 0, 211),
        "clrDarkOrchid" => (153, 50, 204),
        "clrDarkMagenta" => (139, 0, 139),
        "clrPurple" => (128, 0, 128),
        "clrRebeccaPurple" => (102, 51, 153),
        "clrIndigo" => (75, 0, 130),
        "clrMediumSlateBlue" => (123, 104, 238),
        "clrSlateBlue" => (106, 90, 205),
        "clrDarkSlateBlue" => (72, 61, 139),
        // Greens
        "clrGreen" => (0, 128, 0),
        "clrLime" => (0, 255, 0),
        "clrLimeGreen" => (50, 205, 50),
        "clrLawnGreen" => (124, 252, 0),
        "clrChartreuse" => (127, 255, 0),
        "clrGreenYellow" => (173, 255, 47),
        "clrSpringGreen" => (0, 255, 127),
        "clrMediumSpringGreen" => (0, 250, 154),
        "clrLightGreen" => (144, 238, 144),
        "clrPaleGreen" => (152, 251, 152),
        "clrDarkSeaGreen" => (143, 188, 143),
        "clrMediumSeaGreen" => (60, 179, 113),
        "clrSeaGreen" => (46, 139, 87),
        "clrForestGreen" => (34, 139, 34),
        "clrDarkGreen" => (0, 100, 0),
        "clrYellowGreen" => (154, 205, 50),
        "clrOliveDrab" => (107, 142, 35),
        "clrOlive" => (128, 128, 0),
        "clrDarkOliveGreen" => (85, 107, 47),
        "clrMediumAquamarine" => (102, 205, 170),
        "clrDarkCyan" => (0, 139, 139),
        "clrTeal" => (0, 128, 128),
        // Blues
        "clrBlue" => (0, 0, 255),
        "clrAqua" | "clrCyan" => (0, 255, 255),
        "clrLightCyan" => (224, 255, 255),
        "clrPaleTurquoise" => (175, 238, 238),
        "clrAquamarine" => (127, 255, 212),
        "clrTurquoise" => (64, 224, 208),
        "clrMediumTurquoise" => (72, 209, 204),
        "clrDarkTurquoise" => (0, 206, 209),
        "clrCadetBlue" => (95, 158, 160),
        "clrSteelBlue" => (70, 130, 180),
        "clrLightSteelBlue" => (176, 196, 222),
        "clrPowderBlue" => (176, 224, 230),
        "clrLightBlue" => (173, 216, 230),
        "clrSkyBlue" => (135, 206, 235),
        "clrLightSkyBlue" => (135, 206, 250),
        "clrDeepSkyBlue" => (0, 191, 255),
        "clrDodgerBlue" => (30, 144, 255),
        "clrCornflowerBlue" => (100, 149, 237),
        "clrRoyalBlue" => (65, 105, 225),
        "clrMediumBlue" => (0, 0, 205),
        "clrDarkBlue" => (0, 0, 139),
        "clrNavy" => (0, 0, 128),
        "clrMidnightBlue" => (25, 25, 112),
        // Browns
        "clrCornsilk" => (255, 248, 220),
        "clrBlanchedAlmond" => (255, 235, 205),
        "clrBisque" => (255, 228, 196),
        "clrNavajoWhite" => (255, 222, 173),
        "clrWheat" => (245, 222, 179),
        "clrBurlyWood" => (222, 184, 135),
        "clrTan" => (210, 180, 140),
        "clrRosyBrown" => (188, 143, 143),
        "clrSandyBrown" => (244, 164, 96),
        "clrGoldenrod" => (218, 165, 32),
        "clrDarkGoldenrod" => (184, 134, 11),
        "clrPeru" => (205, 133, 63),
        "clrChocolate" => (210, 105, 30),
        "clrSaddleBrown" => (139, 69, 19),
        "clrSienna" => (160, 82, 45),
        "clrBrown" => (165, 42, 42),
        "clrMaroon" => (128, 0, 0),
        // Whites
        "clrWhite" => (255, 255, 255),
        "clrSnow" => (255, 250, 250),
        "clrHoneydew" => (240, 255, 240),
        "clrMintCream" => (245, 255, 250),
        "clrAzure" => (240, 255, 255),
        "clrAliceBlue" => (240, 248, 255),
        "clrGhostWhite" => (248, 248, 255),
        "clrWhiteSmoke" => (245, 245, 245),
        "clrSeashell" => (255, 245, 238),
        "clrBeige" => (245, 245, 220),
        "clrOldLace" => (253, 245, 230),
        "clrFloralWhite" => (255, 250, 240),
        "clrIvory" => (255, 255, 240),
        "clrAntiqueWhite" => (250, 235, 215),
        "clrLinen" => (250, 240, 230),
        "clrLavenderBlush" => (255, 240, 245),
        "clrMistyRose" => (255, 228, 225),
        // Grays
        "clrBlack" => (0, 0, 0),
        "clrGainsboro" => (220, 220, 220),
        "clrLightGray" | "clrLightGrey" => (211, 211, 211),
        "clrSilver" => (192, 192, 192),
        "clrDarkGray" | "clrDarkGrey" => (169, 169, 169),
        "clrGray" | "clrGrey" => (128, 128, 128),
        "clrDimGray" | "clrDimGrey" => (105, 105, 105),
        "clrLightSlateGray" | "clrLightSlateGrey" => (119, 136, 153),
        "clrSlateGray" | "clrSlateGrey" => (112, 128, 144),
        "clrDarkSlateGray" | "clrDarkSlateGrey" => (47, 79, 79),
        "clrNONE" => return None,
        _ => return None,
    };
    Some((r, g, b, start + end))
}

/// Scan backwards from (line, col) to find the enclosing function call name
/// and count commas to determine the active parameter index.
fn find_function_call_context(source: &str, line: usize, col: usize) -> Option<(String, usize)> {
    let lines: Vec<&str> = source.lines().collect();
    if line >= lines.len() {
        return None;
    }

    // Build the text from the start of the file to the cursor position
    let mut text = String::new();
    for (i, l) in lines.iter().enumerate() {
        if i < line {
            text.push_str(l);
            text.push('\n');
        } else if i == line {
            let end = col.min(l.len());
            text.push_str(&l[..end]);
        }
    }

    // Scan backwards from the end of `text` to find the matching open paren
    let bytes = text.as_bytes();
    let mut depth = 0i32;
    let mut comma_count = 0usize;
    let mut paren_pos = None;

    let mut i = bytes.len();
    while i > 0 {
        i -= 1;
        match bytes[i] {
            b')' => depth += 1,
            b'(' => {
                if depth == 0 {
                    paren_pos = Some(i);
                    break;
                }
                depth -= 1;
            }
            b',' if depth == 0 => {
                comma_count += 1;
            }
            _ => {}
        }
    }

    let paren_pos = paren_pos?;

    // Extract the function name immediately before the open paren
    let before_paren = &text[..paren_pos];
    let trimmed = before_paren.trim_end();
    if trimmed.is_empty() {
        return None;
    }

    // Walk backwards from end of trimmed to find identifier
    let tb = trimmed.as_bytes();
    let end = tb.len();
    let mut start = end;
    while start > 0 && is_ident_byte(tb[start - 1]) {
        start -= 1;
    }

    if start == end {
        return None;
    }

    let func_name = &trimmed[start..end];
    Some((func_name.to_string(), comma_count))
}

/// Parse parameter strings from a function signature like "void Foo(int a, string b)".
fn parse_signature_params(signature: &str) -> Vec<String> {
    // Find the content between first ( and last )
    let start = match signature.find('(') {
        Some(p) => p + 1,
        None => return Vec::new(),
    };
    let end = match signature.rfind(')') {
        Some(p) => p,
        None => return Vec::new(),
    };

    if start >= end {
        return Vec::new();
    }

    let params_str = &signature[start..end];

    // Split by commas (respecting nested parens/brackets)
    let mut params = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();

    for ch in params_str.chars() {
        match ch {
            '(' | '<' | '[' => {
                depth += 1;
                current.push(ch);
            }
            ')' | '>' | ']' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    params.push(trimmed);
                }
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        params.push(trimmed);
    }

    params
}

/// Compute the #include directive to add for a target file, relative to the current file.
fn compute_include_directive(current_file: &Path, target_file: &Path) -> Option<String> {
    // Check if target is in the MQL5/Include/ tree → use angle brackets
    let target_str = target_file.to_string_lossy();
    if let Some(idx) = target_str.find("MQL5/Include/") {
        let relative = &target_str[idx + "MQL5/Include/".len()..];
        return Some(format!("#include <{}>", relative.replace('/', "\\")));
    }

    // Otherwise, compute relative path from current file's directory
    let current_dir = current_file.parent()?;
    let relative = pathdiff_relative(current_dir, target_file)?;
    Some(format!("#include \"{}\"", relative))
}

/// Simple relative path computation (current_dir → target).
fn pathdiff_relative(base: &Path, target: &Path) -> Option<String> {
    // Try to make target relative to base
    if let Ok(stripped) = target.strip_prefix(base) {
        return Some(stripped.to_string_lossy().to_string());
    }

    // Walk up from base to find common ancestor
    let base_components: Vec<_> = base.components().collect();
    let target_components: Vec<_> = target.components().collect();

    // Find common prefix length
    let common_len = base_components
        .iter()
        .zip(target_components.iter())
        .take_while(|(a, b)| a == b)
        .count();

    if common_len == 0 {
        return None;
    }

    let ups = base_components.len() - common_len;
    let mut parts: Vec<String> = (0..ups).map(|_| "..".to_string()).collect();
    for comp in &target_components[common_len..] {
        parts.push(comp.as_os_str().to_string_lossy().to_string());
    }

    Some(parts.join("/"))
}

/// Find the line number after the last #include directive (or 0 if none).
fn find_include_insert_line(source: &str) -> u32 {
    let mut last_include_line = None;
    for (i, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("#include") {
            last_include_line = Some(i as u32);
        }
    }
    last_include_line.map(|l| l + 1).unwrap_or(0)
}

use std::path::Path;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Mql5Lsp {
        client,
        index: Arc::new(RwLock::new(SymbolIndex::new())),
        documents: Arc::new(DocumentStore::new()),
        include_resolver: Arc::new(RwLock::new(IncludeResolver::new())),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
