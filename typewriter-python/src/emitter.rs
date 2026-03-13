//! Python code rendering helpers.

use std::collections::BTreeSet;

use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;

use crate::mapper::PythonMapper;

/// Render a Rust struct as a Pydantic v2 `BaseModel` class.
pub fn render_model(mapper: &PythonMapper, def: &StructDef) -> String {
    let mut output = String::new();

    // Collect imports
    let imports = collect_imports_from_struct(def);
    output.push_str(&render_imports(&imports));

    // TypeVar declarations for generic structs
    if !def.generics.is_empty() {
        output.push('\n');
        for param in &def.generics {
            output.push_str(&format!("{} = TypeVar(\"{}\")\n", param, param));
        }
    }

    // Class definition with optional Generic[T] base
    let class_bases = if def.generics.is_empty() {
        "BaseModel".to_string()
    } else {
        format!("BaseModel, Generic[{}]", def.generics.join(", "))
    };

    if let Some(doc) = &def.doc {
        output.push_str(&format!("\n\nclass {}({}):\n", def.name, class_bases));
        output.push_str(&format!("    \"\"\"{}\"\"\"\n\n", doc.trim()));
    } else {
        output.push_str(&format!("\n\nclass {}({}):\n", def.name, class_bases));
    }

    let visible_fields: Vec<&FieldDef> = def.fields.iter().filter(|f| !f.skip).collect();

    if visible_fields.is_empty() {
        output.push_str("    pass\n");
    } else {
        for field in visible_fields {
            let field_name = field.rename.as_deref().unwrap_or(&field.name);
            let type_str = mapper.map_type(&field.ty);

            if let Some(doc) = &field.doc {
                output.push_str(&format!("    # {}\n", doc.trim()));
            }

            if field.optional {
                output.push_str(&format!("    {}: {} = None\n", field_name, type_str));
            } else {
                output.push_str(&format!("    {}: {}\n", field_name, type_str));
            }
        }
    }

    // Config class for Pydantic v2
    output.push_str("\n    class Config:\n");
    output.push_str("        populate_by_name = True\n");

    output
}

/// Render a Rust enum as Python types.
///
/// - Simple (all-unit) enums → `class Role(str, Enum)`
/// - Data-carrying enums → Union of BaseModel subclasses
pub fn render_enum(mapper: &PythonMapper, def: &EnumDef) -> String {
    let all_unit = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    if all_unit {
        render_string_enum(def)
    } else {
        render_union_enum(mapper, def)
    }
}

/// Render a simple enum as a Python string enum.
///
/// ```python
/// class Role(str, Enum):
///     ADMIN = "Admin"
///     USER = "User"
/// ```
fn render_string_enum(def: &EnumDef) -> String {
    let mut output = String::new();

    output.push_str("from enum import Enum\n");

    if let Some(doc) = &def.doc {
        output.push_str(&format!(
            "\n\nclass {}(str, Enum):\n    \"\"\"{}\"\"\"\n\n",
            def.name,
            doc.trim()
        ));
    } else {
        output.push_str(&format!("\n\nclass {}(str, Enum):\n", def.name));
    }

    for variant in &def.variants {
        let display_name = variant.rename.as_deref().unwrap_or(&variant.name);
        let const_name = to_upper_snake(&variant.name);
        output.push_str(&format!("    {} = \"{}\"\n", const_name, display_name));
    }

    output
}

/// Render a data-carrying enum as a Union of BaseModel subclasses.
///
/// For internally tagged enums (`#[serde(tag = "type")]`):
/// ```python
/// class Pending(BaseModel):
///     type: Literal["pending"]
///
/// class Completed(BaseModel):
///     type: Literal["completed"]
///     transaction_id: str
///
/// PaymentStatus = Union[Pending, Completed]
/// ```
fn render_union_enum(mapper: &PythonMapper, def: &EnumDef) -> String {
    let mut output = String::new();

    // Imports
    let mut imports = BTreeSet::new();
    imports.insert("from __future__ import annotations".to_string());
    imports.insert("from pydantic import BaseModel".to_string());
    imports.insert("from typing import Literal, Union".to_string());

    // Collect field-level imports
    for variant in &def.variants {
        if let VariantKind::Struct(fields) = &variant.kind {
            for field in fields {
                collect_type_imports(&field.ty, &mut imports);
            }
        }
    }

    for import in &imports {
        output.push_str(import);
        output.push('\n');
    }

    let tag_field = match &def.representation {
        EnumRepr::Internal { tag } => tag.as_str(),
        _ => "type",
    };

    let mut variant_names = Vec::new();

    for variant in &def.variants {
        let display_name = variant.rename.as_deref().unwrap_or(&variant.name);
        let class_name = &variant.name;
        variant_names.push(class_name.clone());

        output.push_str(&format!("\n\nclass {}(BaseModel):\n", class_name));
        output.push_str(&format!(
            "    {}: Literal[\"{}\"] = \"{}\"\n",
            tag_field, display_name, display_name
        ));

        match &variant.kind {
            VariantKind::Unit => {}
            VariantKind::Struct(fields) => {
                for field in fields {
                    if field.skip {
                        continue;
                    }
                    let fname = field.rename.as_deref().unwrap_or(&field.name);
                    let ftype = mapper.map_type(&field.ty);
                    if field.optional {
                        output.push_str(&format!("    {}: {} = None\n", fname, ftype));
                    } else {
                        output.push_str(&format!("    {}: {}\n", fname, ftype));
                    }
                }
            }
            VariantKind::Tuple(types) => {
                for (i, ty) in types.iter().enumerate() {
                    let ftype = mapper.map_type(ty);
                    output.push_str(&format!("    value_{}: {}\n", i, ftype));
                }
            }
        }
    }

    // Union type alias
    output.push_str(&format!(
        "\n\n{} = Union[{}]\n",
        def.name,
        variant_names.join(", ")
    ));

    output
}

/// Collect Python import statements needed for a struct's field types.
fn collect_imports_from_struct(def: &StructDef) -> BTreeSet<String> {
    let mut imports = BTreeSet::new();
    imports.insert("from __future__ import annotations".to_string());
    imports.insert("from pydantic import BaseModel".to_string());

    // Generic struct needs Generic + TypeVar
    if !def.generics.is_empty() {
        imports.insert("from typing import Generic, TypeVar".to_string());
    }

    for field in &def.fields {
        if field.skip {
            continue;
        }
        collect_type_imports(&field.ty, &mut imports);
    }

    imports
}

/// Recursively collect imports needed for a TypeKind.
fn collect_type_imports(ty: &TypeKind, imports: &mut BTreeSet<String>) {
    match ty {
        TypeKind::Primitive(p) => match p {
            PrimitiveType::Uuid => {
                imports.insert("from uuid import UUID".to_string());
            }
            PrimitiveType::DateTime => {
                imports.insert("from datetime import datetime".to_string());
            }
            PrimitiveType::NaiveDate => {
                imports.insert("from datetime import date".to_string());
            }
            PrimitiveType::JsonValue => {
                imports.insert("from typing import Any".to_string());
            }
            _ => {}
        },
        TypeKind::Option(inner) => {
            imports.insert("from typing import Optional".to_string());
            collect_type_imports(inner, imports);
        }
        TypeKind::Vec(inner) => {
            collect_type_imports(inner, imports);
        }
        TypeKind::HashMap(k, v) => {
            collect_type_imports(k, imports);
            collect_type_imports(v, imports);
        }
        TypeKind::Tuple(elements) => {
            for e in elements {
                collect_type_imports(e, imports);
            }
        }
        TypeKind::Generic(_name, params) => {
            for p in params {
                collect_type_imports(p, imports);
            }
        }
        _ => {}
    }
}

/// Render import statements.
fn render_imports(imports: &BTreeSet<String>) -> String {
    let mut output = String::new();
    for import in imports {
        output.push_str(import);
        output.push('\n');
    }
    output
}

/// Convert PascalCase to UPPER_SNAKE_CASE for enum variant constants.
fn to_upper_snake(name: &str) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            let prev_lower = name.chars().nth(i - 1).map_or(false, |p| p.is_lowercase());
            let next_lower = name.chars().nth(i + 1).map_or(false, |n| n.is_lowercase());
            if prev_lower || next_lower {
                result.push('_');
            }
        }
        result.push(c.to_uppercase().next().unwrap());
    }
    result
}
