//! Shared parser/emitter engine used by both proc-macro and CLI flows.

pub mod drift;
pub mod emit;
pub mod parser;
pub mod project;
pub mod scan;

use std::path::PathBuf;

pub use typewriter_core::{config::TypewriterConfig, ir::Language, ir::TypeDef};

/// Parsed TypeWriter definition discovered in a Rust source file.
#[derive(Debug, Clone)]
pub struct TypeSpec {
    pub type_def: TypeDef,
    pub targets: Vec<Language>,
    pub source_path: PathBuf,
    pub zod_schema: Option<bool>,
}

/// Parse a CSV/list of language names (case-insensitive) into Language values.
pub fn parse_languages(values: &[String]) -> anyhow::Result<Vec<Language>> {
    let mut langs = Vec::new();
    for value in values {
        for raw in value.split(',') {
            let name = raw.trim();
            if name.is_empty() {
                continue;
            }
            let lang = Language::from_str(name).ok_or_else(|| {
                anyhow::anyhow!(
                    "unknown language '{}'. Supported: typescript, python, go, swift, kotlin",
                    name
                )
            })?;
            if !langs.contains(&lang) {
                langs.push(lang);
            }
        }
    }
    Ok(langs)
}

pub fn all_languages() -> Vec<Language> {
    vec![
        Language::TypeScript,
        Language::Python,
        Language::Go,
        Language::Swift,
        Language::Kotlin,
    ]
}
