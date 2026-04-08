//! Project root and configuration discovery utilities.

use anyhow::{Result, bail};
use std::path::{Path, PathBuf};
use typewriter_core::config::TypewriterConfig;

/// Discover project root from a starting path by searching ancestors for `Cargo.toml`.
pub fn discover_project_root(start: &Path) -> Result<PathBuf> {
    for ancestor in start.ancestors() {
        if ancestor.join("Cargo.toml").exists() {
            return Ok(ancestor.to_path_buf());
        }
    }

    bail!(
        "failed to locate project root from '{}': no Cargo.toml found in ancestors",
        start.display()
    )
}

/// Discover best root to resolve `typewriter.toml`/generated paths in proc-macro context.
///
/// Priority:
/// 1) nearest ancestor containing `typewriter.toml`
/// 2) nearest ancestor containing `Cargo.toml`
/// 3) original manifest directory
pub fn discover_macro_root(manifest_dir: &Path) -> PathBuf {
    let mut cargo_root = None;

    for ancestor in manifest_dir.ancestors() {
        if ancestor.join("typewriter.toml").exists() {
            return ancestor.to_path_buf();
        }
        if cargo_root.is_none() && ancestor.join("Cargo.toml").exists() {
            cargo_root = Some(ancestor.to_path_buf());
        }
    }

    cargo_root.unwrap_or_else(|| manifest_dir.to_path_buf())
}

pub fn load_config(project_root: &Path) -> Result<TypewriterConfig> {
    TypewriterConfig::load(project_root)
}

pub fn load_config_or_default(project_root: &Path) -> TypewriterConfig {
    load_config(project_root).unwrap_or_default()
}
