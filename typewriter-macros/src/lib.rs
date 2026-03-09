//! # typewriter-macros
//!
//! Proc macro crate for the typewriter type sync SDK.
//! Provides `#[derive(TypeWriter)]` with `#[sync_to(...)]` and `#[tw(...)]` attributes.

mod emitter;
mod parser;

use proc_macro::TokenStream;

/// Derive macro for typewriter type synchronization.
///
/// # Usage
///
/// ```rust,ignore
/// use typewriter::TypeWriter;
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
    // 1. Parse the derive input into our IR
    let type_def = parser::parse_type_def(input)?;

    // 2. Extract target languages from #[sync_to(...)]
    let targets = parser::parse_sync_to_attr(input)?;

    if targets.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "typewriter: #[sync_to(...)] attribute is required. \
             Example: #[sync_to(typescript, python)]",
        ));
    }

    // 3. Load config (typewriter.toml) - look upwards from CARGO_MANIFEST_DIR
    let config = load_config();

    // 4. Emit to each target language
    emitter::emit_all(&type_def, &targets, &config);

    Ok(())
}

/// Try to load typewriter.toml from the project root.
fn load_config() -> typewriter_core::config::TypewriterConfig {
    // In a proc macro context, CARGO_MANIFEST_DIR points to the crate being compiled
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let path = std::path::Path::new(&manifest_dir);
        // Try the manifest dir itself, then parent dirs
        for ancestor in path.ancestors() {
            if let Ok(config) = typewriter_core::config::TypewriterConfig::load(ancestor) {
                if config.typescript.is_some() || config.python.is_some() {
                    return config;
                }
            }
        }
    }
    typewriter_core::config::TypewriterConfig::default()
}
