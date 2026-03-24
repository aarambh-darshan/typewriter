//! # typewriter-macros
//!
//! Proc macro crate for the typewriter type sync SDK.
//! Provides `#[derive(TypeWriter)]` with `#[sync_to(...)]` and `#[tw(...)]` attributes.

use proc_macro::TokenStream;
use std::path::PathBuf;

/// Derive macro for typewriter type synchronization.
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
/// This will generate corresponding TypeScript and Python type definitions
/// on `cargo build`.
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
