//! Local filesystem content source provider.
//!
//! Wraps existing directory walking and file reading logic behind the
//! `ContentSourceProvider` trait. The Watchtower still uses `notify` for
//! real-time watching — this provider handles scan and read operations.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use sha2::{Digest, Sha256};

use super::{ContentSourceProvider, SourceError, SourceFile};
use crate::automation::watchtower::matches_patterns;

/// Local filesystem content source.
pub struct LocalFsProvider {
    base_path: PathBuf,
}

impl LocalFsProvider {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl ContentSourceProvider for LocalFsProvider {
    fn source_type(&self) -> &str {
        "local_fs"
    }

    async fn scan_for_changes(
        &self,
        _since_cursor: Option<&str>,
        patterns: &[String],
    ) -> Result<Vec<SourceFile>, SourceError> {
        let base = self.base_path.clone();
        let patterns = patterns.to_vec();

        // Walk directory synchronously (matches existing WatchtowerLoop behaviour).
        let files = tokio::task::spawn_blocking(move || walk_and_hash(&base, &base, &patterns))
            .await
            .map_err(|e| SourceError::Io(std::io::Error::other(e)))??;

        Ok(files)
    }

    async fn read_content(&self, file_id: &str) -> Result<String, SourceError> {
        let full_path = self.base_path.join(file_id);
        tokio::fs::read_to_string(&full_path)
            .await
            .map_err(SourceError::Io)
    }
}

/// Recursively walk a directory, returning `SourceFile` entries for matching files.
fn walk_and_hash(
    base: &Path,
    current: &Path,
    patterns: &[String],
) -> Result<Vec<SourceFile>, SourceError> {
    let mut out = Vec::new();
    let entries = std::fs::read_dir(current)?;

    for entry in entries {
        let entry = entry?;
        let ft = entry.file_type()?;
        let path = entry.path();

        if ft.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') {
                    continue;
                }
            }
            out.extend(walk_and_hash(base, &path, patterns)?);
        } else if ft.is_file() && matches_patterns(&path, patterns) {
            if let Ok(rel) = path.strip_prefix(base) {
                let content = std::fs::read(&path)?;
                let hash = format!("{:x}", Sha256::digest(&content));
                let modified = std::fs::metadata(&path)?
                    .modified()
                    .ok()
                    .map(|t| {
                        let dt: chrono::DateTime<chrono::Utc> = t.into();
                        dt.to_rfc3339()
                    })
                    .unwrap_or_default();

                out.push(SourceFile {
                    provider_id: rel.to_string_lossy().to_string(),
                    display_name: path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    content_hash: hash,
                    modified_at: modified,
                });
            }
        }
    }
    Ok(out)
}
