//! # typewriter-macros
//!
//! Proc macro crate for the typewriter type sync SDK.
//! Provides `#[derive(TypeWriter)]` with `#[sync_to(...)]` and `#[tw(...)]` attributes.
//!
//! ## Overview
//!
//! This crate implements the `#[derive(TypeWriter)]` proc macro that generates
//! type definitions in multiple target languages from a single Rust struct or enum definition.
//!
//! ## Supported Attributes
//!
//! ### `#[sync_to(...)]`
//!
//! Specifies which target languages to generate types for:
//!
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(typescript, python, go)]           // Generate TS, Python, and Go types
//! #[sync_to(typescript)]                         // TypeScript only
//! #[sync_to(typescript, python, go, swift, kotlin, graphql, json_schema)]  // All languages
//! pub struct MyType { ... }
//! ```
//!
//! **Supported languages:**
//! - `typescript` / `ts` - TypeScript interfaces and Zod schemas
//! - `python` / `py` - Python Pydantic models
//! - `go` / `golang` - Go structs with JSON tags
//! - `swift` - Swift Codable structs
//! - `kotlin` / `kt` - Kotlin data classes
//! - `graphql` / `gql` - GraphQL SDL types
//! - `json_schema` / `jsonschema` - JSON Schema definitions
//!
//! ### `#[tw(...)]`
//!
//! Fine-tune the generated output per-type or per-field:
//!
//! | Attribute | Description |
//! |-----------|-------------|
//! | `#[tw(skip)]` | Exclude field from generated output |
//! | `#[tw(rename = "name")]` | Override field/variant name in output |
//! | `#[tw(optional)]` | Force field to be optional |
//! | `#[tw(type = "custom")]` | Override the generated type string |
//! | `#[tw(zod)]` | Enable Zod schema generation (TypeScript only) |
//! | `#[tw(zod = false)]` | Disable Zod schema generation (TypeScript only) |
//!
//! ## Example
//!
//! ```rust,ignore
//! use typebridge::TypeWriter;
//! use serde::{Serialize, Deserialize};
//!
//! /// A user profile with all supported features.
//! #[derive(Serialize, Deserialize, TypeWriter)]
//! #[sync_to(typescript, python)]
//! #[tw(zod)]  // Enable Zod schema generation
//! pub struct UserProfile {
//!     pub id: Uuid,
//!     
//!     /// User's email address
//!     pub email: String,
//!     
//!     #[tw(skip)]  // Not included in generated types
//!     pub password_hash: String,
//!     
//!     #[tw(rename = "displayName")]  // Renamed in output
//!     pub username: String,
//!     
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! This generates:
//! - `./generated/typescript/user-profile.ts` - TypeScript interface
//! - `./generated/typescript/user-profile.schema.ts` - Zod schema
//! - `./generated/python/user_profile.py` - Python Pydantic model
//!
//! ## Build-Time Behavior
//!
//! Type files are generated during `cargo build`. The macro:
//! 1. Parses the annotated struct/enum
//! 2. Reads `typewriter.toml` for configuration (if present)
//! 3. Generates type definitions for each target language
//! 4. Writes files to the configured output directories

use proc_macro::TokenStream;
use std::path::PathBuf;

/// Derive macro for typewriter type synchronization.
///
/// This macro generates type definitions in target languages from Rust structs and enums.
///
/// # Usage
///
/// ```rust,ignore
/// use typebridge::TypeWriter;
///
/// #[derive(TypeWriter)]
/// #[sync_to(typescript, python)]
/// pub struct UserProfile {
///     pub id: Uuid,
///     pub email: String,
///     pub age: Option<u32>,
/// }
/// ```
///
/// # Errors
///
/// The macro will produce a compile error if:
/// - `#[sync_to(...)]` is missing (required)
/// - An unsupported language is specified
/// - The type is a union (not supported)
///
/// # Output
///
/// On successful compilation, type files are generated:
/// - TypeScript: `generated/typescript/<type-name>.ts` (+ `.schema.ts` for Zod)
/// - Python: `generated/python/<type_name>.py`
/// - Go: `generated/go/<type_name>.go`
/// - And more for other target languages
#[proc_macro_derive(TypeWriter, attributes(sync_to, tw))]
pub fn derive_typewriter(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match typewriter_impl(&input) {
        Ok(_) => TokenStream::new(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn typewriter_impl(input: &syn::DeriveInput) -> syn::Result<()> {
    let type_def = typewriter_engine::parser::parse_type_def(input)?;
    let targets = typewriter_engine::parser::parse_sync_to_attr(input)?;
    let zod_schema = typewriter_engine::parser::parse_tw_zod_attr(input)?;

    if targets.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "typewriter: #[sync_to(...)] attribute is required. \
             Example: #[sync_to(typescript, python)]",
        ));
    }

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let manifest_dir = PathBuf::from(manifest_dir);
    let project_root = typewriter_engine::project::discover_macro_root(&manifest_dir);
    let config = typewriter_engine::project::load_config_or_default(&project_root);

    let spec = typewriter_engine::TypeSpec {
        type_def,
        targets,
        source_path: manifest_dir.join("<proc-macro>"),
        zod_schema,
    };

    let files =
        match typewriter_engine::emit::render_specs(&[spec], &project_root, &config, &[], true) {
            Ok(files) => files,
            Err(err) => {
                eprintln!("typewriter: generation failed for {}: {}", input.ident, err);
                return Ok(());
            }
        };

    if let Err(err) = typewriter_engine::emit::write_generated_files(&files) {
        eprintln!("typewriter: failed to write generated files: {}", err);
        return Ok(());
    }

    for file in files {
        eprintln!(
            "  typewriter: {} → {}",
            file.type_name,
            file.output_path.display()
        );
    }

    Ok(())
}
