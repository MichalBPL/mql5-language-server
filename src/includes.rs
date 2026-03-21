//! Include resolution for MQL5 files.
//!
//! MQL5 include paths:
//! - `#include <Path\File.mqh>` — relative to MQL5/Include/
//! - `#include "File.mqh"` — relative to current file's directory

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::parser::{self, IncludeDirective};

/// Resolves MQL5 include paths to absolute file paths.
pub struct IncludeResolver {
    /// MQL5/Include/ directory (for system includes with <>)
    include_root: Option<PathBuf>,
    /// Cache of resolved paths: (source_file, include_path) -> resolved_path
    cache: HashMap<(PathBuf, String), Option<PathBuf>>,
}

impl IncludeResolver {
    pub fn new() -> Self {
        Self {
            include_root: None,
            cache: HashMap::new(),
        }
    }

    /// Get the MQL5 Include root directory, if detected.
    pub fn include_root(&self) -> Option<&PathBuf> {
        self.include_root.as_ref()
    }

    /// Set the MQL5 Include root directory.
    #[allow(dead_code)]
    pub fn set_include_root(&mut self, root: PathBuf) {
        self.include_root = Some(root);
    }

    /// Try to detect the MQL5/Include/ root from a workspace path.
    /// Looks for the MQL5 directory structure.
    pub fn detect_include_root(&mut self, workspace: &Path) {
        // Walk up from the workspace to find MQL5/Include/
        let mut current = workspace;
        loop {
            // Check if current/Include/ exists
            let include_dir = current.join("Include");
            if include_dir.is_dir() {
                self.include_root = Some(include_dir);
                log::info!("Detected MQL5 Include root: {:?}", self.include_root);
                return;
            }

            // Check if we're inside an MQL5 directory
            if current.file_name().is_some_and(|n| n == "MQL5") {
                let include_dir = current.join("Include");
                if include_dir.is_dir() {
                    self.include_root = Some(include_dir);
                    log::info!("Detected MQL5 Include root: {:?}", self.include_root);
                    return;
                }
            }

            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }
        log::warn!("Could not detect MQL5 Include root from {:?}", workspace);
    }

    /// Resolve an include directive to an absolute file path.
    pub fn resolve(
        &mut self,
        include: &IncludeDirective,
        source_file: &Path,
    ) -> Option<PathBuf> {
        let key = (
            source_file.to_path_buf(),
            include.path.clone(),
        );

        if let Some(cached) = self.cache.get(&key) {
            return cached.clone();
        }

        let result = self.resolve_uncached(include, source_file);
        self.cache.insert(key, result.clone());
        result
    }

    fn resolve_uncached(
        &self,
        include: &IncludeDirective,
        source_file: &Path,
    ) -> Option<PathBuf> {
        let normalized_path = include.path.replace('\\', "/");

        if include.is_system {
            // System include: look in MQL5/Include/
            if let Some(ref root) = self.include_root {
                let resolved = root.join(&normalized_path);
                log::debug!("System include {:?} -> {:?} exists={}", normalized_path, resolved.display(), resolved.is_file());
                if resolved.is_file() {
                    return Some(resolved);
                }
            }
        } else {
            // Local include: look relative to source file's directory
            if let Some(source_dir) = source_file.parent() {
                let resolved = source_dir.join(&normalized_path);
                log::debug!("Local include {:?} dir={:?} -> {:?} exists={}", normalized_path, source_dir.display(), resolved.display(), resolved.is_file());
                if resolved.is_file() {
                    return Some(resolved);
                }
            }

            // Fallback: also try system include path
            if let Some(ref root) = self.include_root {
                let resolved = root.join(&normalized_path);
                if resolved.is_file() {
                    return Some(resolved);
                }
            }
        }

        // Last resort for both system and local: walk up from source file
        // looking for the file. Handles cases where include root detection
        // fails (e.g., paths with spaces on macOS).
        if let Some(mut dir) = source_file.parent().map(|p| p.to_path_buf()) {
            for _ in 0..10 {
                let candidate = dir.join(&normalized_path);
                if candidate.is_file() {
                    return Some(candidate);
                }
                // Also check Include/ subdirectory at each level
                let inc_candidate = dir.join("Include").join(&normalized_path);
                if inc_candidate.is_file() {
                    return Some(inc_candidate);
                }
                match dir.parent() {
                    Some(p) => dir = p.to_path_buf(),
                    None => break,
                }
            }
        }

        None
    }

    /// Get all files that a source file includes (transitively).
    /// Returns the set of resolved file paths.
    pub fn get_transitive_includes(
        &mut self,
        source_file: &Path,
        source_content: &str,
        file_reader: &dyn Fn(&Path) -> Option<String>,
    ) -> Vec<PathBuf> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.collect_includes(source_file, source_content, file_reader, &mut visited, &mut result);
        result
    }

    fn collect_includes(
        &mut self,
        source_file: &Path,
        source_content: &str,
        file_reader: &dyn Fn(&Path) -> Option<String>,
        visited: &mut HashSet<PathBuf>,
        result: &mut Vec<PathBuf>,
    ) {
        if !visited.insert(source_file.to_path_buf()) {
            return; // Already visited (circular include)
        }

        let tree = match parser::parse(source_content) {
            Some(t) => t,
            None => return,
        };

        let includes = parser::extract_includes(source_content, &tree);

        for inc in &includes {
            if let Some(resolved) = self.resolve(inc, source_file) {
                if !visited.contains(&resolved) {
                    result.push(resolved.clone());
                    if let Some(content) = file_reader(&resolved) {
                        self.collect_includes(
                            &resolved,
                            &content,
                            file_reader,
                            visited,
                            result,
                        );
                    }
                }
            }
        }
    }

    /// Clear the resolution cache.
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
