//! # typewriter-engine
//!
//! Shared parser/emitter engine used by both proc-macro and CLI flows.
//!
//! This crate provides the core functionality for the typewriter type synchronization SDK:
//! - **Scanning**: Discovers `#[derive(TypeWriter)]` items in Rust source files
//! - **Parsing**: Converts Rust AST (syn) into language-agnostic IR (Internal Representation)
//! - **Rendering**: Generates type definitions for target languages (TypeScript, Python, Go, etc.)
//! - **Drift Detection**: Compares expected generated output with actual on-disk files
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────┐     ┌─────────────┐     ┌─────────┐
//! │  Rust Source     │────▶│  Scanner    │────▶│  Parser │
//! │  (.rs files)     │     │  (scan.rs)  │     │(parser) │
//! └─────────────────┘     └─────────────┘     └────┬────┘
//!                                                   │
//!                                                   ▼
//! ┌─────────────────┐     ┌─────────────┐     ┌─────────┐
//! │  Generated Files │◀────│  Writer     │◀────│   IR    │
//! │  (.ts, .py, ...) │     │  (emit.rs)  │     │  Types  │
//! └─────────────────┘     └─────────────┘     └─────────┘
//! ```
//!
//! ## Modules
//!
//! - [`scan`] - Source file discovery and TypeWriter item detection
//! - [`parser`] - syn DeriveInput → IR type conversion
//! - [`emit`] - IR → target language type rendering and file writing
//! - [`drift`] - Drift detection between expected and actual generated files
//! - [`project`] - Project root and configuration discovery
//!
//! ## Usage
//!
//! ### CLI Usage
//!
//! ```rust,ignore
//! use typewriter_engine::{scan, emit, project};
//!
//! // Discover project root and load config
//! let project_root = project::discover_project_root(&std::env::current_dir()?)?;
//! let config = project::load_config(&project_root)?;
//!
//! // Scan for TypeWriter definitions
//! let specs = scan::scan_project(&project_root)?;
//!
//! // Render to all target languages
//! let files = emit::render_specs_deduped(&specs, &project_root, &config, &[], false)?;
//!
//! // Write generated files
//! emit::write_generated_files(&files)?;
//! ```
//!
//! ### Library Usage
//!
//! ```rust,ignore
//! use typewriter_engine::{TypeSpec, Language, TypeDef};
//! use typewriter_core::ir::{StructDef, TypeDef};
//!
//! // Create a type spec manually
//! let spec = TypeSpec {
//!     type_def: TypeDef::Struct(StructDef { ... }),
//!     targets: vec![Language::TypeScript, Language::Python],
//!     source_path: PathBuf::from("src/models.rs"),
//!     zod_schema: None,
//! };
//! ```

pub mod drift;
pub mod emit;
pub mod parser;
pub mod project;
pub mod scan;

use std::path::PathBuf;

pub use typewriter_core::{config::TypewriterConfig, ir::Language, ir::TypeDef};

/// Parsed TypeWriter definition discovered in a Rust source file.
///
/// This struct contains all information needed to generate type definitions
/// for a single Rust struct or enum marked with `#[derive(TypeWriter)]`.
#[derive(Debug, Clone)]
pub struct TypeSpec {
    /// The parsed IR type definition (struct or enum)
    pub type_def: TypeDef,
    /// Target languages to generate types for (from `#[sync_to(...)]`)
    pub targets: Vec<Language>,
    /// Path to the source file containing this type
    pub source_path: PathBuf,
    /// Optional Zod schema generation override (None = use config default)
    pub zod_schema: Option<bool>,
}

/// Parse a comma-separated or list of language names (case-insensitive) into Language values.
///
/// # Supported Languages
///
/// | String | Language |
/// |--------|----------|
/// | `"typescript"`, `"ts"` | TypeScript |
/// | `"python"`, `"py"` | Python |
/// | `"go"`, `"golang"` | Go |
/// | `"swift"` | Swift |
/// | `"kotlin"`, `"kt"` | Kotlin |
/// | `"graphql"`, `"gql"` | GraphQL SDL |
/// | `"json_schema"`, `"jsonschema"` | JSON Schema |
///
/// # Errors
///
/// Returns an error if any language name is not recognized.
///
/// # Examples
///
/// ```
/// use typewriter_engine::parse_languages;
///
/// let langs = parse_languages(&["typescript,python".to_string()]).unwrap();
/// assert!(langs.contains(&typewriter_engine::Language::TypeScript));
/// assert!(langs.contains(&typewriter_engine::Language::Python));
/// ```
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
                    "unknown language '{}'. Supported: typescript, python, go, swift, kotlin, graphql, json_schema",
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

/// Returns a vector of all supported languages.
///
/// This is useful when you want to generate types for all supported languages
/// without having to list them manually.
///
/// # Examples
///
/// ```
/// use typewriter_engine::all_languages;
///
/// let all = all_languages();
/// assert!(all.contains(&typewriter_engine::Language::TypeScript));
/// assert!(all.contains(&typewriter_engine::Language::Python));
/// assert_eq!(all.len(), 7);
/// ```
pub fn all_languages() -> Vec<Language> {
    vec![
        Language::TypeScript,
        Language::Python,
        Language::Go,
        Language::Swift,
        Language::Kotlin,
        Language::GraphQL,
        Language::JsonSchema,
    ]
}
