//! TypeScript code rendering helpers.

use std::collections::BTreeMap;

use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;

use crate::mapper::TypeScriptMapper;

/// Render a Rust struct as a TypeScript `export interface`.
pub fn render_interface(mapper: &TypeScriptMapper, def: &StructDef) -> String {
    let mut output = String::new();

    // Doc comment
    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    // Interface declaration with optional generic params
    if def.generics.is_empty() {
        output.push_str(&format!("export interface {} {{\n", def.name));
    } else {
        output.push_str(&format!(
            "export interface {}<{}> {{\n",
            def.name,
            def.generics.join(", ")
        ));
    }

    for field in &def.fields {
        if field.skip {
            continue;
        }

        // Field doc comment
        if let Some(doc) = &field.doc {
            output.push_str(&format!("  /** {} */\n", doc.trim()));
        }

        let field_name = field.rename.as_deref().unwrap_or(&field.name);
        let type_str = field
            .type_override
            .clone()
            .unwrap_or_else(|| mapper.map_type(&field.ty));

        let readonly_prefix = if mapper.readonly { "readonly " } else { "" };

        if field.flatten {
            output.push_str("  // @flatten\n");
        }

        if field.optional {
            output.push_str(&format!(
                "  {}{}?: {};\n",
                readonly_prefix, field_name, type_str
            ));
        } else {
            output.push_str(&format!(
                "  {}{}: {};\n",
                readonly_prefix, field_name, type_str
            ));
        }
    }

    output.push_str("}\n");
    output
}

/// Render a Rust enum as TypeScript types.
///
/// - Simple (all-unit) enums → string literal union: `type Role = "Admin" | "User" | "Guest"`
/// - Data-carrying enums → discriminated union types based on `EnumRepr`
pub fn render_enum(mapper: &TypeScriptMapper, def: &EnumDef) -> String {
    let all_unit = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    if all_unit {
        render_string_enum(def)
    } else {
        render_discriminated_union(mapper, def)
    }
}

/// Render a simple enum as a string literal union.
///
/// ```typescript
/// export type Role = "Admin" | "User" | "Guest";
/// ```
fn render_string_enum(def: &EnumDef) -> String {
    let mut output = String::new();

    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    let variants: Vec<String> = def
        .variants
        .iter()
        .map(|v| {
            let name = v.rename.as_deref().unwrap_or(&v.name);
            format!("\"{}\"", name)
        })
        .collect();

    output.push_str(&format!(
        "export type {} =\n  | {};\n",
        def.name,
        variants.join("\n  | ")
    ));

    output
}

/// Render a data-carrying enum as a discriminated union.
///
/// Dispatches based on `EnumRepr`:
/// - **External**: `{ "VariantName": { ...data } }`
/// - **Internal**: `{ type: "variant", ...data }`
/// - **Adjacent**: `{ t: "variant", c: { ...data } }`
/// - **Untagged**: `{ ...data }` — no discriminator
fn render_discriminated_union(mapper: &TypeScriptMapper, def: &EnumDef) -> String {
    let mut output = String::new();

    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    let variants: Vec<String> = def
        .variants
        .iter()
        .map(|v| render_variant_type(mapper, def, v))
        .collect();

    output.push_str(&format!(
        "export type {} =\n  | {};\n",
        def.name,
        variants.join("\n  | ")
    ));

    output
}

/// Render a single variant based on the enum representation.
fn render_variant_type(mapper: &TypeScriptMapper, def: &EnumDef, v: &VariantDef) -> String {
    let variant_name = v.rename.as_deref().unwrap_or(&v.name);

    match &def.representation {
        EnumRepr::External => render_variant_external(mapper, v, variant_name),
        EnumRepr::Internal { tag } => render_variant_internal(mapper, v, variant_name, tag),
        EnumRepr::Adjacent { tag, content } => {
            render_variant_adjacent(mapper, v, variant_name, tag, content)
        }
        EnumRepr::Untagged => render_variant_untagged(mapper, v),
    }
}

/// External: `{ "VariantName": { ...data } }` or `{ "VariantName": value }`
fn render_variant_external(
    mapper: &TypeScriptMapper,
    v: &VariantDef,
    variant_name: &str,
) -> String {
    match &v.kind {
        VariantKind::Unit => {
            format!("{{ \"{}\": {{}} }}", variant_name)
        }
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!(
                "{{ \"{}\": {{ {} }} }}",
                variant_name,
                field_strs.join("; ")
            )
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
            if types.len() == 1 {
                format!("{{ \"{}\": {} }}", variant_name, type_strs[0])
            } else {
                format!("{{ \"{}\": [{}] }}", variant_name, type_strs.join(", "))
            }
        }
    }
}

/// Internal: `{ tag: "variant", ...data }` — data fields are inlined alongside tag
fn render_variant_internal(
    mapper: &TypeScriptMapper,
    v: &VariantDef,
    variant_name: &str,
    tag: &str,
) -> String {
    match &v.kind {
        VariantKind::Unit => {
            format!("{{ {}: \"{}\" }}", tag, variant_name)
        }
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!(
                "{{ {}: \"{}\"; {} }}",
                tag,
                variant_name,
                field_strs.join("; ")
            )
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
            if types.len() == 1 {
                format!(
                    "{{ {}: \"{}\"; value: {} }}",
                    tag, variant_name, type_strs[0]
                )
            } else {
                format!(
                    "{{ {}: \"{}\"; value: [{}] }}",
                    tag,
                    variant_name,
                    type_strs.join(", ")
                )
            }
        }
    }
}

/// Adjacent: `{ tag: "variant", content: { ...data } }`
fn render_variant_adjacent(
    mapper: &TypeScriptMapper,
    v: &VariantDef,
    variant_name: &str,
    tag: &str,
    content: &str,
) -> String {
    match &v.kind {
        VariantKind::Unit => {
            format!("{{ {}: \"{}\" }}", tag, variant_name)
        }
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!(
                "{{ {}: \"{}\"; {}: {{ {} }} }}",
                tag,
                variant_name,
                content,
                field_strs.join("; ")
            )
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
            if types.len() == 1 {
                format!(
                    "{{ {}: \"{}\"; {}: {} }}",
                    tag, variant_name, content, type_strs[0]
                )
            } else {
                format!(
                    "{{ {}: \"{}\"; {}: [{}] }}",
                    tag,
                    variant_name,
                    content,
                    type_strs.join(", ")
                )
            }
        }
    }
}

/// Untagged: no discriminator — just the data shape
fn render_variant_untagged(mapper: &TypeScriptMapper, v: &VariantDef) -> String {
    match &v.kind {
        VariantKind::Unit => "{}".to_string(),
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!("{{ {} }}", field_strs.join("; "))
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
            if types.len() == 1 {
                type_strs[0].clone()
            } else {
                format!("[{}]", type_strs.join(", "))
            }
        }
    }
}

/// Helper to render struct fields as `name: type` strings.
fn render_field_strs(mapper: &TypeScriptMapper, fields: &[FieldDef]) -> Vec<String> {
    fields
        .iter()
        .filter(|f| !f.skip)
        .map(|f| {
            let fname = f.rename.as_deref().unwrap_or(&f.name);
            let ftype = f
                .type_override
                .clone()
                .unwrap_or_else(|| mapper.map_type(&f.ty));

            let mut field_decl = String::new();
            if f.flatten {
                field_decl.push_str("  // @flatten\n  ");
            }
            if f.optional {
                field_decl.push_str(&format!("{}?: {}", fname, ftype));
            } else {
                field_decl.push_str(&format!("{}: {}", fname, ftype));
            }
            field_decl
        })
        .collect()
}

/// Render runtime imports for a Zod schema file.
pub fn render_zod_imports(mapper: &TypeScriptMapper, def: &TypeDef) -> String {
    let mut output = String::from("import { z } from 'zod';\n");
    let refs = def.collect_referenced_types();

    for name in refs {
        if name == def.name() {
            continue;
        }

        let file_name = mapper.file_naming(&name);
        output.push_str(&format!(
            "import {{ {}Schema }} from './{}.schema';\n",
            name, file_name
        ));
    }

    output
}

/// Render a `TypeDef` as a Zod schema declaration.
pub fn render_zod_type_def(mapper: &TypeScriptMapper, def: &TypeDef) -> String {
    match def {
        TypeDef::Struct(s) => render_zod_struct(mapper, s),
        TypeDef::Enum(e) => render_zod_enum(mapper, e),
    }
}

struct ZodContext<'a> {
    current_type: &'a str,
    generic_schemas: BTreeMap<String, String>,
}

impl<'a> ZodContext<'a> {
    fn for_type(current_type: &'a str) -> Self {
        Self {
            current_type,
            generic_schemas: BTreeMap::new(),
        }
    }

    fn for_struct(def: &'a StructDef) -> Self {
        let generic_schemas = def
            .generics
            .iter()
            .map(|g| (g.clone(), generic_schema_arg_name(g)))
            .collect();

        Self {
            current_type: &def.name,
            generic_schemas,
        }
    }
}

fn render_zod_struct(mapper: &TypeScriptMapper, def: &StructDef) -> String {
    let mut output = String::new();
    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    let ctx = ZodContext::for_struct(def);
    let schema_expr = render_zod_object_schema(mapper, &def.fields, &ctx);

    if def.generics.is_empty() {
        output.push_str(&format!(
            "export const {}Schema = {};\n",
            def.name, schema_expr
        ));
        return output;
    }

    let generic_type_params: Vec<String> = def
        .generics
        .iter()
        .map(|g| format!("{}Schema extends z.ZodTypeAny", g))
        .collect();
    let generic_args: Vec<String> = def
        .generics
        .iter()
        .map(|g| format!("{}: {}Schema", generic_schema_arg_name(g), g))
        .collect();

    output.push_str(&format!(
        "export const {}Schema = <{}>({}) => {};\n",
        def.name,
        generic_type_params.join(", "),
        generic_args.join(", "),
        schema_expr
    ));

    output
}

fn render_zod_enum(mapper: &TypeScriptMapper, def: &EnumDef) -> String {
    let mut output = String::new();
    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    let ctx = ZodContext::for_type(&def.name);
    let all_unit = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    let schema_expr = if all_unit {
        render_zod_string_enum(def)
    } else {
        render_zod_data_enum(mapper, def, &ctx)
    };

    output.push_str(&format!(
        "export const {}Schema = {};\n",
        def.name, schema_expr
    ));
    output
}

fn render_zod_string_enum(def: &EnumDef) -> String {
    let variants: Vec<String> = def
        .variants
        .iter()
        .map(|v| quote_string(v.rename.as_deref().unwrap_or(&v.name)))
        .collect();

    format!("z.enum([{}])", variants.join(", "))
}

fn render_zod_data_enum(mapper: &TypeScriptMapper, def: &EnumDef, ctx: &ZodContext<'_>) -> String {
    let variants: Vec<String> = def
        .variants
        .iter()
        .map(|v| render_zod_variant(mapper, def, v, ctx))
        .collect();

    match &def.representation {
        EnumRepr::Internal { tag } | EnumRepr::Adjacent { tag, .. } => {
            if variants.len() == 1 {
                variants[0].clone()
            } else {
                format!(
                    "z.discriminatedUnion({}, [{}])",
                    quote_string(tag),
                    variants.join(", ")
                )
            }
        }
        EnumRepr::External | EnumRepr::Untagged => render_zod_union(&variants),
    }
}

fn render_zod_union(variants: &[String]) -> String {
    if variants.is_empty() {
        return "z.never()".to_string();
    }
    if variants.len() == 1 {
        return variants[0].clone();
    }
    format!("z.union([{}])", variants.join(", "))
}

fn render_zod_variant(
    mapper: &TypeScriptMapper,
    def: &EnumDef,
    variant: &VariantDef,
    ctx: &ZodContext<'_>,
) -> String {
    let variant_name = variant.rename.as_deref().unwrap_or(&variant.name);

    match &def.representation {
        EnumRepr::External => render_zod_variant_external(mapper, variant, variant_name, ctx),
        EnumRepr::Internal { tag } => {
            render_zod_variant_internal(mapper, variant, variant_name, tag, ctx)
        }
        EnumRepr::Adjacent { tag, content } => {
            render_zod_variant_adjacent(mapper, variant, variant_name, tag, content, ctx)
        }
        EnumRepr::Untagged => render_zod_variant_untagged(mapper, variant, ctx),
    }
}

fn render_zod_variant_external(
    mapper: &TypeScriptMapper,
    variant: &VariantDef,
    variant_name: &str,
    ctx: &ZodContext<'_>,
) -> String {
    let key = quote_object_key(variant_name);
    match &variant.kind {
        VariantKind::Unit => format!("z.object({{{}: z.object({{}})}})", key),
        VariantKind::Struct(fields) => format!(
            "z.object({{{}: {}}})",
            key,
            render_zod_object_schema(mapper, fields, ctx)
        ),
        VariantKind::Tuple(types) => {
            if types.len() == 1 {
                format!(
                    "z.object({{{}: {}}})",
                    key,
                    map_zod_type(mapper, &types[0], ctx)
                )
            } else {
                format!(
                    "z.object({{{}: z.tuple([{}])}})",
                    key,
                    types
                        .iter()
                        .map(|t| map_zod_type(mapper, t, ctx))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

fn render_zod_variant_internal(
    mapper: &TypeScriptMapper,
    variant: &VariantDef,
    variant_name: &str,
    tag: &str,
    ctx: &ZodContext<'_>,
) -> String {
    let mut entries = vec![format!(
        "{}: {}",
        quote_object_key(tag),
        zod_literal(variant_name)
    )];

    match &variant.kind {
        VariantKind::Unit => {}
        VariantKind::Struct(fields) => {
            entries.extend(render_zod_field_entries(mapper, fields, ctx))
        }
        VariantKind::Tuple(types) => {
            let value_schema = if types.len() == 1 {
                map_zod_type(mapper, &types[0], ctx)
            } else {
                format!(
                    "z.tuple([{}])",
                    types
                        .iter()
                        .map(|t| map_zod_type(mapper, t, ctx))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            entries.push(format!("\"value\": {}", value_schema));
        }
    }

    format!("z.object({{{}}})", entries.join(", "))
}

fn render_zod_variant_adjacent(
    mapper: &TypeScriptMapper,
    variant: &VariantDef,
    variant_name: &str,
    tag: &str,
    content: &str,
    ctx: &ZodContext<'_>,
) -> String {
    let mut entries = vec![format!(
        "{}: {}",
        quote_object_key(tag),
        zod_literal(variant_name)
    )];

    match &variant.kind {
        VariantKind::Unit => {}
        VariantKind::Struct(fields) => entries.push(format!(
            "{}: {}",
            quote_object_key(content),
            render_zod_object_schema(mapper, fields, ctx)
        )),
        VariantKind::Tuple(types) => {
            let value_schema = if types.len() == 1 {
                map_zod_type(mapper, &types[0], ctx)
            } else {
                format!(
                    "z.tuple([{}])",
                    types
                        .iter()
                        .map(|t| map_zod_type(mapper, t, ctx))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            entries.push(format!("{}: {}", quote_object_key(content), value_schema));
        }
    }

    format!("z.object({{{}}})", entries.join(", "))
}

fn render_zod_variant_untagged(
    mapper: &TypeScriptMapper,
    variant: &VariantDef,
    ctx: &ZodContext<'_>,
) -> String {
    match &variant.kind {
        VariantKind::Unit => "z.object({})".to_string(),
        VariantKind::Struct(fields) => render_zod_object_schema(mapper, fields, ctx),
        VariantKind::Tuple(types) => {
            if types.len() == 1 {
                map_zod_type(mapper, &types[0], ctx)
            } else {
                format!(
                    "z.tuple([{}])",
                    types
                        .iter()
                        .map(|t| map_zod_type(mapper, t, ctx))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}

fn render_zod_object_schema(
    mapper: &TypeScriptMapper,
    fields: &[FieldDef],
    ctx: &ZodContext<'_>,
) -> String {
    let entries = render_zod_field_entries(mapper, fields, ctx);
    if entries.is_empty() {
        "z.object({})".to_string()
    } else {
        format!("z.object({{{}}})", entries.join(", "))
    }
}

fn render_zod_field_entries(
    mapper: &TypeScriptMapper,
    fields: &[FieldDef],
    ctx: &ZodContext<'_>,
) -> Vec<String> {
    fields
        .iter()
        .filter(|f| !f.skip)
        .map(|f| {
            let field_name = f.rename.as_deref().unwrap_or(&f.name);
            let schema = zod_field_schema(mapper, f, ctx);
            format!("{}: {}", quote_object_key(field_name), schema)
        })
        .collect()
}

fn zod_field_schema(mapper: &TypeScriptMapper, field: &FieldDef, ctx: &ZodContext<'_>) -> String {
    let base_schema = if field.type_override.is_some() {
        // `#[tw(type = "...")]` is a free-form TS string and cannot be safely parsed to Zod.
        "z.any()".to_string()
    } else if field.optional {
        match &field.ty {
            TypeKind::Option(inner) => map_zod_type(mapper, inner, ctx),
            other => map_zod_type(mapper, other, ctx),
        }
    } else {
        map_zod_type(mapper, &field.ty, ctx)
    };

    if field.optional {
        format!("{}.optional()", base_schema)
    } else {
        base_schema
    }
}

fn map_zod_type(mapper: &TypeScriptMapper, ty: &TypeKind, ctx: &ZodContext<'_>) -> String {
    match ty {
        TypeKind::Primitive(p) => map_zod_primitive(p),
        TypeKind::Option(inner) => format!("{}.optional()", map_zod_type(mapper, inner, ctx)),
        TypeKind::Vec(inner) => format!("z.array({})", map_zod_type(mapper, inner, ctx)),
        TypeKind::HashMap(key, value) => format!(
            "z.record({}, {})",
            map_zod_type(mapper, key, ctx),
            map_zod_type(mapper, value, ctx)
        ),
        TypeKind::Tuple(elements) => format!(
            "z.tuple([{}])",
            elements
                .iter()
                .map(|e| map_zod_type(mapper, e, ctx))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeKind::Named(name) => map_zod_named(name, ctx),
        TypeKind::Generic(name, params) => map_zod_generic(mapper, name, params, ctx),
        TypeKind::Unit => "z.void()".to_string(),
    }
}

fn map_zod_primitive(primitive: &PrimitiveType) -> String {
    match primitive {
        PrimitiveType::String => "z.string()".to_string(),
        PrimitiveType::Bool => "z.boolean()".to_string(),
        PrimitiveType::U8
        | PrimitiveType::U16
        | PrimitiveType::U32
        | PrimitiveType::I8
        | PrimitiveType::I16
        | PrimitiveType::I32
        | PrimitiveType::F32
        | PrimitiveType::F64 => "z.number()".to_string(),
        PrimitiveType::U64 | PrimitiveType::U128 | PrimitiveType::I64 | PrimitiveType::I128 => {
            "z.bigint()".to_string()
        }
        PrimitiveType::Uuid | PrimitiveType::DateTime | PrimitiveType::NaiveDate => {
            "z.string()".to_string()
        }
        PrimitiveType::JsonValue => "z.unknown()".to_string(),
    }
}

fn map_zod_named(name: &str, ctx: &ZodContext<'_>) -> String {
    if let Some(schema_arg) = ctx.generic_schemas.get(name) {
        return schema_arg.clone();
    }

    let schema_ident = if name == ctx.current_type {
        format!("{}Schema", ctx.current_type)
    } else {
        format!("{}Schema", name)
    };

    format!("z.lazy(() => {})", schema_ident)
}

fn map_zod_generic(
    mapper: &TypeScriptMapper,
    name: &str,
    params: &[TypeKind],
    ctx: &ZodContext<'_>,
) -> String {
    let args = params
        .iter()
        .map(|p| map_zod_type(mapper, p, ctx))
        .collect::<Vec<_>>();

    let call_expr = if args.is_empty() {
        format!("{}Schema()", name)
    } else {
        format!("{}Schema({})", name, args.join(", "))
    };

    format!("z.lazy(() => {})", call_expr)
}

fn generic_schema_arg_name(generic: &str) -> String {
    let mut chars = generic.chars();
    match chars.next() {
        Some(first) => {
            let mut out = String::new();
            out.push(first.to_ascii_lowercase());
            out.push_str(chars.as_str());
            out.push_str("Schema");
            out
        }
        None => "schema".to_string(),
    }
}

fn quote_object_key(value: &str) -> String {
    format!("\"{}\"", escape_string(value))
}

fn quote_string(value: &str) -> String {
    format!("\"{}\"", escape_string(value))
}

fn zod_literal(value: &str) -> String {
    format!("z.literal({})", quote_string(value))
}

fn escape_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
