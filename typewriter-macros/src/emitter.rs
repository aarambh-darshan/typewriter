//! Emitter dispatcher — calls language-specific emitters and handles file I/O.

use std::fs;
use std::path::PathBuf;

use typewriter_core::config::TypewriterConfig;
use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;

/// Emit type definitions to all requested target languages.
pub fn emit_all(type_def: &TypeDef, targets: &[Language], config: &TypewriterConfig) {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let base_path = PathBuf::from(&manifest_dir);

    for target in targets {
        match target {
            #[cfg(feature = "typescript")]
            Language::TypeScript => {
                let mut mapper = typewriter_typescript::TypeScriptMapper::new()
                    .with_readonly(config.ts_readonly());
                if let Some(style) = config.ts_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = base_path.join(config.ts_output_dir());
                emit_single(&mapper, type_def, &output_dir);
            }
            #[cfg(feature = "python")]
            Language::Python => {
                let mut mapper = typewriter_python::PythonMapper::new();
                if let Some(style) = config.py_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = base_path.join(config.py_output_dir());
                emit_single(&mapper, type_def, &output_dir);
            }
            #[cfg(feature = "go")]
            Language::Go => {
                let mut mapper = typewriter_go::GoMapper::new();
                if let Some(style) = config.go_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = base_path.join(config.go_output_dir());
                emit_single(&mapper, type_def, &output_dir);
            }
            #[cfg(feature = "swift")]
            Language::Swift => {
                let mut mapper = typewriter_swift::SwiftMapper::new();
                if let Some(style) = config.swift_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = base_path.join(config.swift_output_dir());
                emit_single(&mapper, type_def, &output_dir);
            }
            #[cfg(feature = "kotlin")]
            Language::Kotlin => {
                let mut mapper = typewriter_kotlin::KotlinMapper::new()
                    .with_package_name(config.kotlin_package_name());
                if let Some(style) = config.kotlin_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = base_path.join(config.kotlin_output_dir());
                emit_single(&mapper, type_def, &output_dir);
            }
            #[allow(unreachable_patterns)]
            _ => {
                // Language not enabled via feature flags — silently skip
                // In a future version, this could emit a compile warning
            }
        }
    }
}

/// Emit a single type definition using a specific mapper.
fn emit_single<M: TypeMapper>(mapper: &M, type_def: &TypeDef, output_dir: &PathBuf) {
    // Generate the content
    let content = mapper.emit_type_def(type_def);
    let filename = mapper.output_filename(type_def.name());

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!(
            "typewriter: failed to create output directory {}: {}",
            output_dir.display(),
            e
        );
        return;
    }

    // Write the file
    let output_path = output_dir.join(&filename);
    if let Err(e) = fs::write(&output_path, &content) {
        eprintln!(
            "typewriter: failed to write {}: {}",
            output_path.display(),
            e
        );
        return;
    }

    // Print status (visible during cargo build)
    eprintln!(
        "  typewriter: {} → {}",
        type_def.name(),
        output_path.display()
    );
}
