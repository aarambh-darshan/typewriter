//! Source scanning for `#[derive(TypeWriter)]` items.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use syn::{Data, DeriveInput, Item, ItemEnum, ItemStruct, ItemUnion};
use walkdir::WalkDir;

use crate::{parser, TypeSpec};

pub fn scan_project(project_root: &Path) -> Result<Vec<TypeSpec>> {
    let mut specs = Vec::new();
    for file in discover_rust_files(project_root) {
        specs.extend(scan_file(&file)?);
    }
    Ok(specs)
}

pub fn scan_file(path: &Path) -> Result<Vec<TypeSpec>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read source file {}", path.display()))?;
    let parsed = syn::parse_file(&content)
        .with_context(|| format!("failed to parse Rust source {}", path.display()))?;

    let mut specs = Vec::new();
    collect_items(&parsed.items, path, &mut specs)?;
    Ok(specs)
}

pub fn discover_rust_files(project_root: &Path) -> Vec<PathBuf> {
    WalkDir::new(project_root)
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(name == ".git" || name == "target")
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .map(|ext| ext == "rs")
                    .unwrap_or(false)
        })
        .map(|entry| entry.into_path())
        .collect()
}

fn collect_items(items: &[Item], source_path: &Path, specs: &mut Vec<TypeSpec>) -> Result<()> {
    for item in items {
        match item {
            Item::Struct(item_struct) => {
                maybe_collect_from_derive_input(
                    item_struct_to_derive(item_struct),
                    source_path,
                    specs,
                )?;
            }
            Item::Enum(item_enum) => {
                maybe_collect_from_derive_input(
                    item_enum_to_derive(item_enum),
                    source_path,
                    specs,
                )?;
            }
            Item::Union(item_union) => {
                maybe_collect_from_derive_input(
                    item_union_to_derive(item_union),
                    source_path,
                    specs,
                )?;
            }
            Item::Mod(item_mod) => {
                if let Some((_, inline_items)) = &item_mod.content {
                    collect_items(inline_items, source_path, specs)?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn maybe_collect_from_derive_input(
    input: DeriveInput,
    source_path: &Path,
    specs: &mut Vec<TypeSpec>,
) -> Result<()> {
    if !parser::has_typewriter_derive(&input.attrs) {
        return Ok(());
    }

    let type_def = parser::parse_type_def(&input)
        .map_err(|err| anyhow::anyhow!("{} ({})", err, source_path.display()))?;
    let targets = parser::parse_sync_to_attr(&input)
        .map_err(|err| anyhow::anyhow!("{} ({})", err, source_path.display()))?;
    let zod_schema = parser::parse_tw_zod_attr(&input)
        .map_err(|err| anyhow::anyhow!("{} ({})", err, source_path.display()))?;

    if targets.is_empty() {
        return Err(anyhow::anyhow!(
            "typewriter: #[sync_to(...)] attribute is required. Example: #[sync_to(typescript, python)] ({})",
            source_path.display()
        ));
    }

    specs.push(TypeSpec {
        type_def,
        targets,
        source_path: source_path.to_path_buf(),
        zod_schema,
    });

    Ok(())
}

fn item_struct_to_derive(item: &ItemStruct) -> DeriveInput {
    DeriveInput {
        attrs: item.attrs.clone(),
        vis: item.vis.clone(),
        ident: item.ident.clone(),
        generics: item.generics.clone(),
        data: Data::Struct(syn::DataStruct {
            struct_token: item.struct_token,
            fields: item.fields.clone(),
            semi_token: item.semi_token,
        }),
    }
}

fn item_enum_to_derive(item: &ItemEnum) -> DeriveInput {
    DeriveInput {
        attrs: item.attrs.clone(),
        vis: item.vis.clone(),
        ident: item.ident.clone(),
        generics: item.generics.clone(),
        data: Data::Enum(syn::DataEnum {
            enum_token: item.enum_token,
            brace_token: item.brace_token,
            variants: item.variants.clone(),
        }),
    }
}

fn item_union_to_derive(item: &ItemUnion) -> DeriveInput {
    DeriveInput {
        attrs: item.attrs.clone(),
        vis: item.vis.clone(),
        ident: item.ident.clone(),
        generics: item.generics.clone(),
        data: Data::Union(syn::DataUnion {
            union_token: item.union_token,
            fields: item.fields.clone(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_typewriter_items_from_file() {
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join("mod.rs");
        std::fs::write(
            &file,
            r#"
            #[derive(TypeWriter)]
            #[sync_to(typescript, python)]
            struct User {
                id: String,
            }
            "#,
        )
        .unwrap();

        let specs = scan_file(&file).unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].type_def.name(), "User");
        assert_eq!(specs[0].zod_schema, None);
    }

    #[test]
    fn scans_type_level_zod_override() {
        let temp = tempfile::tempdir().unwrap();
        let file = temp.path().join("mod.rs");
        std::fs::write(
            &file,
            r#"
            #[derive(TypeWriter)]
            #[sync_to(typescript)]
            #[tw(zod = false)]
            struct Address {
                id: String,
            }
            "#,
        )
        .unwrap();

        let specs = scan_file(&file).unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].type_def.name(), "Address");
        assert_eq!(specs[0].zod_schema, Some(false));
    }
}
