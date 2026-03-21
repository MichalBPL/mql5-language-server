//! Open document management — tracks file contents in memory.

use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::lsp_types::*;

/// Manages open document contents in memory.
pub struct DocumentStore {
    docs: DashMap<Url, DocumentState>,
}

pub struct DocumentState {
    pub rope: Rope,
    pub version: i32,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self {
            docs: DashMap::new(),
        }
    }

    /// Called when a document is opened.
    pub fn open(&self, uri: Url, text: String, version: i32) {
        self.docs.insert(
            uri,
            DocumentState {
                rope: Rope::from_str(&text),
                version,
            },
        );
    }

    /// Called when a document is changed (incremental sync).
    pub fn apply_changes(&self, uri: &Url, changes: Vec<TextDocumentContentChangeEvent>, version: i32) {
        if let Some(mut doc) = self.docs.get_mut(uri) {
            for change in changes {
                if let Some(range) = change.range {
                    // Incremental change
                    let start_line = range.start.line as usize;
                    let start_char = range.start.character as usize;
                    let end_line = range.end.line as usize;
                    let end_char = range.end.character as usize;

                    let start_idx = line_char_to_idx(&doc.rope, start_line, start_char);
                    let end_idx = line_char_to_idx(&doc.rope, end_line, end_char);

                    if start_idx <= end_idx && end_idx <= doc.rope.len_chars() {
                        doc.rope.remove(start_idx..end_idx);
                        doc.rope.insert(start_idx, &change.text);
                    }
                } else {
                    // Full replacement
                    doc.rope = Rope::from_str(&change.text);
                }
            }
            doc.version = version;
        }
    }

    /// Called when a document is closed.
    pub fn close(&self, uri: &Url) {
        self.docs.remove(uri);
    }

    /// Get the full text of a document.
    pub fn get_text(&self, uri: &Url) -> Option<String> {
        self.docs.get(uri).map(|doc| doc.rope.to_string())
    }

    /// Get a specific line from a document.
    #[allow(dead_code)]
    pub fn get_line(&self, uri: &Url, line: usize) -> Option<String> {
        self.docs.get(uri).and_then(|doc| {
            if line < doc.rope.len_lines() {
                Some(doc.rope.line(line).to_string())
            } else {
                None
            }
        })
    }

    /// Check if a document is tracked (open in editor).
    #[allow(dead_code)]
    pub fn is_open(&self, uri: &Url) -> bool {
        self.docs.contains_key(uri)
    }
}

fn line_char_to_idx(rope: &Rope, line: usize, character: usize) -> usize {
    if line >= rope.len_lines() {
        return rope.len_chars();
    }
    let line_start = rope.line_to_char(line);
    let line_len = rope.line(line).len_chars();
    line_start + character.min(line_len)
}
