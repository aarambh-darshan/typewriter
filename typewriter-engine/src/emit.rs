//! Render and write generated output files for target languages.

use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use typewriter_core::config::TypewriterConfig;
use typewriter_core::ir::{Language, TypeDef};
use typewriter_core::mapper::TypeMapper;

use crate::TypeSpec;

#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub type_name: String,
    pub language: Language,
    pub output_path: PathBuf,
    pub content: String,
    pub source_path: PathBuf,
}

/// Render all expected generated files for a set of type specs.
pub fn render_specs(
    specs: &[TypeSpec],
    project_root: &Path,
    config: &TypewriterConfig,
    lang_filter: &[Language],
    skip_unavailable: bool,
) -> Result<Vec<GeneratedFile>> {
    let mut files = Vec::new();

    for spec in specs {
        for target in effective_targets(&spec.targets, lang_filter) {
            let mut rendered = render_single(
                &spec.type_def,
                spec.source_path.clone(),
                spec.zod_schema,
                target,
                project_root,
                config,
                skip_unavailable,
            )?;
            files.append(&mut rendered);
        }
    }

    Ok(files)
}

/// Render with output path deduplication (last writer wins on path collisions).
pub fn render_specs_deduped(
    specs: &[TypeSpec],
    project_root: &Path,
    config: &TypewriterConfig,
    lang_filter: &[Language],
    skip_unavailable: bool,
) -> Result<Vec<GeneratedFile>> {
    let files = render_specs(specs, project_root, config, lang_filter, skip_unavailable)?;
    let mut by_path = BTreeMap::new();
    for file in files {
        by_path.insert(file.output_path.clone(), file);
    }
    Ok(by_path.into_values().collect())
}

pub fn write_generated_files(files: &[GeneratedFile]) -> Result<()> {
    for file in files {
        if let Some(parent) = file.output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file.output_path, &file.content)?;
    }
    Ok(())
}

pub fn language_label(language: Language) -> &'static str {
    match language {
        Language::TypeScript => "typescript",
        Language::Python => "python",
        Language::Go => "go",
        Language::Swift => "swift",
        Language::Kotlin => "kotlin",
        Language::GraphQL => "graphql",
    }
}

pub fn file_extension(language: Language) -> &'static str {
    match language {
        Language::TypeScript => "ts",
        Language::Python => "py",
        Language::Go => "go",
        Language::Swift => "swift",
        Language::Kotlin => "kt",
        Language::GraphQL => "graphql",
    }
}

pub fn output_dir_for_language(config: &TypewriterConfig, language: Language) -> &str {
    match language {
        Language::TypeScript => config.ts_output_dir(),
        Language::Python => config.py_output_dir(),
        Language::Go => config.go_output_dir(),
        Language::Swift => config.swift_output_dir(),
        Language::Kotlin => config.kotlin_output_dir(),
        Language::GraphQL => config.graphql_output_dir(),
    }
}

fn effective_targets(spec_targets: &[Language], lang_filter: &[Language]) -> Vec<Language> {
    if lang_filter.is_empty() {
        return spec_targets.to_vec();
    }

    spec_targets
        .iter()
        .copied()
        .filter(|lang| lang_filter.contains(lang))
        .collect()
}

fn render_single(
    type_def: &TypeDef,
    source_path: PathBuf,
    zod_override: Option<bool>,
    language: Language,
    project_root: &Path,
    config: &TypewriterConfig,
    _skip_unavailable: bool,
) -> Result<Vec<GeneratedFile>> {
    match language {
        Language::TypeScript => {
            #[cfg(feature = "typescript")]
            {
                let mut mapper = typewriter_typescript::TypeScriptMapper::new()
                    .with_readonly(config.ts_readonly());
                if let Some(style) = config.ts_file_style() {
                    mapper = mapper.with_file_style(style);
                }

                let output_dir = project_root.join(config.ts_output_dir());

                let type_file = render_with_mapper(
                    &mapper,
                    type_def,
                    source_path.clone(),
                    language,
                    output_dir.clone(),
                );

                let mut files = vec![type_file];

                if zod_override.unwrap_or(config.ts_zod_enabled()) {
                    files.push(GeneratedFile {
                        type_name: type_def.name().to_string(),
                        language,
                        output_path: output_dir.join(mapper.zod_output_filename(type_def.name())),
                        content: mapper.emit_zod_type_def(type_def),
                        source_path,
                    });
                }

                return Ok(files);
            }
            #[cfg(not(feature = "typescript"))]
            {
                if _skip_unavailable {
                    return Ok(vec![]);
                }
                anyhow::bail!("language 'typescript' is not enabled in this build")
            }
        }
        Language::Python => {
            #[cfg(feature = "python")]
            {
                let mut mapper = typewriter_python::PythonMapper::new();
                if let Some(style) = config.py_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = project_root.join(config.py_output_dir());
                return Ok(vec![render_with_mapper(
                    &mapper,
                    type_def,
                    source_path,
                    language,
                    output_dir,
                )]);
            }
            #[cfg(not(feature = "python"))]
            {
                if _skip_unavailable {
                    return Ok(vec![]);
                }
                anyhow::bail!("language 'python' is not enabled in this build")
            }
        }
        Language::Go => {
            #[cfg(feature = "go")]
            {
                let mut mapper =
                    typewriter_go::GoMapper::new().with_package_name(config.go_package_name());
                if let Some(style) = config.go_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = project_root.join(config.go_output_dir());
                return Ok(vec![render_with_mapper(
                    &mapper,
                    type_def,
                    source_path,
                    language,
                    output_dir,
                )]);
            }
            #[cfg(not(feature = "go"))]
            {
                if _skip_unavailable {
                    return Ok(vec![]);
                }
                anyhow::bail!("language 'go' is not enabled in this build")
            }
        }
        Language::Swift => {
            #[cfg(feature = "swift")]
            {
                let mut mapper = typewriter_swift::SwiftMapper::new();
                if let Some(style) = config.swift_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = project_root.join(config.swift_output_dir());
                return Ok(vec![render_with_mapper(
                    &mapper,
                    type_def,
                    source_path,
                    language,
                    output_dir,
                )]);
            }
            #[cfg(not(feature = "swift"))]
            {
                if _skip_unavailable {
                    return Ok(vec![]);
                }
                anyhow::bail!("language 'swift' is not enabled in this build")
            }
        }
        Language::Kotlin => {
            #[cfg(feature = "kotlin")]
            {
                let mut mapper = typewriter_kotlin::KotlinMapper::new()
                    .with_package_name(config.kotlin_package_name());
                if let Some(style) = config.kotlin_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = project_root.join(config.kotlin_output_dir());
                return Ok(vec![render_with_mapper(
                    &mapper,
                    type_def,
                    source_path,
                    language,
                    output_dir,
                )]);
            }
            #[cfg(not(feature = "kotlin"))]
            {
                if _skip_unavailable {
                    return Ok(vec![]);
                }
                anyhow::bail!("language 'kotlin' is not enabled in this build")
            }
        }
        Language::GraphQL => {
            #[cfg(feature = "graphql")]
            {
                let mut mapper = typewriter_graphql::GraphQLMapper::new();
                if let Some(style) = config.graphql_file_style() {
                    mapper = mapper.with_file_style(style);
                }
                let output_dir = project_root.join(config.graphql_output_dir());
                return Ok(vec![render_with_mapper(
                    &mapper,
                    type_def,
                    source_path,
                    language,
                    output_dir,
                )]);
            }
            #[cfg(not(feature = "graphql"))]
            {
                if _skip_unavailable {
                    return Ok(vec![]);
                }
                anyhow::bail!("language 'graphql' is not enabled in this build")
            }
        }
    }
}

fn render_with_mapper<M: TypeMapper>(
    mapper: &M,
    type_def: &TypeDef,
    source_path: PathBuf,
    language: Language,
    output_dir: PathBuf,
) -> GeneratedFile {
    let filename = mapper.output_filename(type_def.name());
    let content = mapper.emit_type_def(type_def);

    GeneratedFile {
        type_name: type_def.name().to_string(),
        language,
        output_path: output_dir.join(filename),
        content,
        source_path,
    }
}
