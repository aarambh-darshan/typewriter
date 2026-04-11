//! # typewriter-plugin-dart
//!
//! Dart/Flutter emitter plugin for typewriter.
//!
//! Generates `json_serializable`-compatible Dart classes from Rust structs and enums.
//!
//! ## Type Mappings
//!
//! | Rust Type | Dart Type |
//! |-----------|-----------|
//! | `String` | `String` |
//! | `bool` | `bool` |
//! | `u8`–`u128`, `i8`–`i128` | `int` |
//! | `f32`, `f64` | `double` |
//! | `Option<T>` | `T?` |
//! | `Vec<T>` | `List<T>` |
//! | `HashMap<K, V>` | `Map<K, V>` |
//! | `Uuid` | `String` |
//! | `DateTime` | `DateTime` |

use typewriter_plugin::prelude::*;

/// Dart language mapper.
pub struct DartMapper {
    file_style: FileStyle,
}

impl DartMapper {
    pub fn new() -> Self {
        Self {
            file_style: FileStyle::SnakeCase,
        }
    }

    pub fn with_file_style(mut self, style: FileStyle) -> Self {
        self.file_style = style;
        self
    }

    fn to_camel_case(name: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        for (i, c) in name.chars().enumerate() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else if i == 0 {
                result.push(c.to_lowercase().next().unwrap_or(c));
            } else {
                result.push(c);
            }
        }
        result
    }
}

impl Default for DartMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeMapper for DartMapper {
    fn map_primitive(&self, ty: &PrimitiveType) -> String {
        match ty {
            PrimitiveType::String => "String".to_string(),
            PrimitiveType::Bool => "bool".to_string(),
            PrimitiveType::U8 | PrimitiveType::U16 | PrimitiveType::U32
            | PrimitiveType::U64 | PrimitiveType::U128
            | PrimitiveType::I8 | PrimitiveType::I16 | PrimitiveType::I32
            | PrimitiveType::I64 | PrimitiveType::I128 => "int".to_string(),
            PrimitiveType::F32 | PrimitiveType::F64 => "double".to_string(),
            PrimitiveType::Uuid => "String".to_string(),
            PrimitiveType::DateTime | PrimitiveType::NaiveDate => "DateTime".to_string(),
            PrimitiveType::JsonValue => "dynamic".to_string(),
        }
    }

    fn map_option(&self, inner: &TypeKind) -> String {
        format!("{}?", self.map_type(inner))
    }

    fn map_vec(&self, inner: &TypeKind) -> String {
        format!("List<{}>", self.map_type(inner))
    }

    fn map_hashmap(&self, key: &TypeKind, value: &TypeKind) -> String {
        format!("Map<{}, {}>", self.map_type(key), self.map_type(value))
    }

    fn map_tuple(&self, elements: &[TypeKind]) -> String {
        // Dart doesn't have native tuples, use a Record type (Dart 3.0+)
        let inner: Vec<String> = elements.iter().map(|e| self.map_type(e)).collect();
        format!("({})", inner.join(", "))
    }

    fn map_named(&self, name: &str) -> String {
        name.to_string()
    }

    fn map_generic(&self, name: &str, params: &[TypeKind]) -> String {
        let param_strs: Vec<String> = params.iter().map(|p| self.map_type(p)).collect();
        format!("{}<{}>", name, param_strs.join(", "))
    }

    fn emit_struct(&self, def: &StructDef) -> String {
        let mut output = String::new();
        let file_name = self.file_naming(&def.name);

        // Import and part directive
        output.push_str("import 'package:json_annotation/json_annotation.dart';\n\n");
        output.push_str(&format!("part '{}.g.dart';\n\n", file_name));

        // Doc comment
        if let Some(doc) = &def.doc {
            output.push_str(&format!("/// {}\n", doc.trim()));
        }

        // Class declaration
        output.push_str("@JsonSerializable()\n");
        if def.generics.is_empty() {
            output.push_str(&format!("class {} {{\n", def.name));
        } else {
            output.push_str(&format!(
                "class {}<{}> {{\n",
                def.name,
                def.generics.join(", ")
            ));
        }

        // Fields
        for field in &def.fields {
            if field.skip { continue; }

            if let Some(doc) = &field.doc {
                output.push_str(&format!("  /// {}\n", doc.trim()));
            }

            let field_name = field.rename.as_deref().unwrap_or(&field.name);
            let dart_field_name = Self::to_camel_case(field_name);
            let type_str = field
                .type_override
                .clone()
                .unwrap_or_else(|| self.map_type(&field.ty));

            // Add @JsonKey if rename differs from dart field name
            if field.rename.is_some() || field_name != dart_field_name {
                output.push_str(&format!("  @JsonKey(name: '{}')\n", field_name));
            }

            output.push_str(&format!("  final {} {};\n", type_str, dart_field_name));
        }

        output.push('\n');

        // Constructor
        output.push_str(&format!("  const {}({{\n", def.name));
        for field in &def.fields {
            if field.skip { continue; }
            let field_name = field.rename.as_deref().unwrap_or(&field.name);
            let dart_field_name = Self::to_camel_case(field_name);

            if field.optional {
                output.push_str(&format!("    this.{},\n", dart_field_name));
            } else {
                output.push_str(&format!("    required this.{},\n", dart_field_name));
            }
        }
        output.push_str("  });\n\n");

        // fromJson / toJson
        output.push_str(&format!(
            "  factory {}.fromJson(Map<String, dynamic> json) =>\n      _${}FromJson(json);\n",
            def.name, def.name
        ));
        output.push_str(&format!(
            "  Map<String, dynamic> toJson() => _${}ToJson(this);\n",
            def.name
        ));

        output.push_str("}\n");
        output
    }

    fn emit_enum(&self, def: &EnumDef) -> String {
        let all_unit = def.variants.iter().all(|v| matches!(v.kind, VariantKind::Unit));

        if all_unit {
            self.emit_dart_enum(def)
        } else {
            self.emit_sealed_class(def)
        }
    }

    fn file_header(&self, type_name: &str) -> String {
        format!(
            "// Auto-generated by typewriter v0.5.2. DO NOT EDIT.\n\
             // Source: {}\n\
             // Regenerate: cargo typewriter generate\n\n",
            type_name
        )
    }

    fn file_extension(&self) -> &str {
        "dart"
    }

    fn file_naming(&self, type_name: &str) -> String {
        to_file_style(type_name, self.file_style)
    }

    fn emit_imports(&self, _def: &TypeDef) -> String {
        String::new()
    }
}

impl DartMapper {
    fn emit_dart_enum(&self, def: &EnumDef) -> String {
        let mut output = String::new();

        if let Some(doc) = &def.doc {
            output.push_str(&format!("/// {}\n", doc.trim()));
        }

        output.push_str(&format!("enum {} {{\n", def.name));

        for (i, variant) in def.variants.iter().enumerate() {
            let name = variant.rename.as_deref().unwrap_or(&variant.name);
            let dart_name = Self::to_camel_case(name);

            if let Some(doc) = &variant.doc {
                output.push_str(&format!("  /// {}\n", doc.trim()));
            }

            // Use @JsonValue annotation for the string mapping
            output.push_str(&format!("  @JsonValue('{}')\n", name));

            let trailing = if i < def.variants.len() - 1 { "," } else { ";" };
            output.push_str(&format!("  {}{}\n", dart_name, trailing));
        }

        output.push_str("}\n");
        output
    }

    fn emit_sealed_class(&self, def: &EnumDef) -> String {
        let mut output = String::new();
        let file_name = self.file_naming(&def.name);

        // Imports
        output.push_str("import 'package:json_annotation/json_annotation.dart';\n\n");
        output.push_str(&format!("part '{}.g.dart';\n\n", file_name));

        if let Some(doc) = &def.doc {
            output.push_str(&format!("/// {}\n", doc.trim()));
        }

        output.push_str(&format!("sealed class {} {{\n", def.name));
        output.push_str(&format!("  const {}();\n", def.name));
        output.push_str("}\n\n");

        for variant in &def.variants {
            let variant_name = variant.rename.as_deref().unwrap_or(&variant.name);

            match &variant.kind {
                VariantKind::Unit => {
                    output.push_str(&format!(
                        "class {} extends {} {{\n  const {}();\n}}\n\n",
                        variant_name, def.name, variant_name
                    ));
                }
                VariantKind::Struct(fields) => {
                    output.push_str("@JsonSerializable()\n");
                    output.push_str(&format!(
                        "class {} extends {} {{\n",
                        variant_name, def.name
                    ));

                    for field in fields.iter().filter(|f| !f.skip) {
                        let fname = field.rename.as_deref().unwrap_or(&field.name);
                        let dart_fname = Self::to_camel_case(fname);
                        let ftype = field
                            .type_override
                            .clone()
                            .unwrap_or_else(|| self.map_type(&field.ty));
                        output.push_str(&format!("  final {} {};\n", ftype, dart_fname));
                    }

                    output.push_str(&format!("\n  const {}({{\n", variant_name));
                    for field in fields.iter().filter(|f| !f.skip) {
                        let fname = field.rename.as_deref().unwrap_or(&field.name);
                        let dart_fname = Self::to_camel_case(fname);
                        if field.optional {
                            output.push_str(&format!("    this.{},\n", dart_fname));
                        } else {
                            output.push_str(&format!("    required this.{},\n", dart_fname));
                        }
                    }
                    output.push_str("  });\n\n");

                    output.push_str(&format!(
                        "  factory {}.fromJson(Map<String, dynamic> json) =>\n      _${}FromJson(json);\n",
                        variant_name, variant_name
                    ));
                    output.push_str(&format!(
                        "  Map<String, dynamic> toJson() => _${}ToJson(this);\n",
                        variant_name
                    ));
                    output.push_str("}\n\n");
                }
                VariantKind::Tuple(types) => {
                    output.push_str(&format!(
                        "class {} extends {} {{\n",
                        variant_name, def.name
                    ));
                    for (i, ty) in types.iter().enumerate() {
                        output.push_str(&format!(
                            "  final {} value{};\n",
                            self.map_type(ty), i
                        ));
                    }
                    output.push_str(&format!("\n  const {}(", variant_name));
                    let params: Vec<String> = (0..types.len())
                        .map(|i| format!("this.value{}", i))
                        .collect();
                    output.push_str(&params.join(", "));
                    output.push_str(");\n");
                    output.push_str("}\n\n");
                }
            }
        }

        output
    }
}

/// Dart/Flutter emitter plugin entry point.
pub struct DartPlugin;

impl DartPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DartPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl EmitterPlugin for DartPlugin {
    fn language_id(&self) -> &str {
        "dart"
    }

    fn language_name(&self) -> &str {
        "Dart/Flutter"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn default_output_dir(&self) -> &str {
        "./generated/dart"
    }

    fn file_extension(&self) -> &str {
        "dart"
    }

    fn mapper(&self, config: &PluginConfig) -> Box<dyn TypeMapper> {
        let mut mapper = DartMapper::new();
        if let Some(style_str) = config.file_style.as_deref() {
            if let Some(style) = FileStyle::from_str(style_str) {
                mapper = mapper.with_file_style(style);
            }
        }
        Box::new(mapper)
    }
}

typewriter_plugin::declare_plugin!(DartPlugin);

#[cfg(test)]
mod tests {
    use super::*;

    fn mapper() -> DartMapper {
        DartMapper::new()
    }

    #[test]
    fn test_primitive_mappings() {
        let m = mapper();
        assert_eq!(m.map_primitive(&PrimitiveType::String), "String");
        assert_eq!(m.map_primitive(&PrimitiveType::Bool), "bool");
        assert_eq!(m.map_primitive(&PrimitiveType::U32), "int");
        assert_eq!(m.map_primitive(&PrimitiveType::I64), "int");
        assert_eq!(m.map_primitive(&PrimitiveType::F64), "double");
        assert_eq!(m.map_primitive(&PrimitiveType::Uuid), "String");
        assert_eq!(m.map_primitive(&PrimitiveType::DateTime), "DateTime");
        assert_eq!(m.map_primitive(&PrimitiveType::JsonValue), "dynamic");
    }

    #[test]
    fn test_option_mapping() {
        let m = mapper();
        assert_eq!(
            m.map_option(&TypeKind::Primitive(PrimitiveType::U32)),
            "int?"
        );
    }

    #[test]
    fn test_vec_mapping() {
        let m = mapper();
        assert_eq!(
            m.map_vec(&TypeKind::Primitive(PrimitiveType::String)),
            "List<String>"
        );
    }

    #[test]
    fn test_hashmap_mapping() {
        let m = mapper();
        assert_eq!(
            m.map_hashmap(
                &TypeKind::Primitive(PrimitiveType::String),
                &TypeKind::Primitive(PrimitiveType::U32)
            ),
            "Map<String, int>"
        );
    }

    #[test]
    fn test_emit_simple_struct() {
        let m = mapper();
        let def = StructDef {
            name: "User".to_string(),
            fields: vec![
                FieldDef {
                    name: "id".to_string(),
                    ty: TypeKind::Primitive(PrimitiveType::String),
                    optional: false, rename: None, doc: None,
                    skip: false, flatten: false, type_override: None,
                },
                FieldDef {
                    name: "age".to_string(),
                    ty: TypeKind::Option(Box::new(TypeKind::Primitive(PrimitiveType::U32))),
                    optional: true, rename: None, doc: None,
                    skip: false, flatten: false, type_override: None,
                },
            ],
            doc: None,
            generics: vec![],
        };

        let output = m.emit_struct(&def);
        assert!(output.contains("@JsonSerializable()"));
        assert!(output.contains("class User {"));
        assert!(output.contains("final String id;"));
        assert!(output.contains("final int? age;"));
        assert!(output.contains("required this.id,"));
        assert!(output.contains("this.age,"));
        assert!(output.contains("_$UserFromJson"));
        assert!(output.contains("_$UserToJson"));
    }

    #[test]
    fn test_emit_simple_enum() {
        let m = mapper();
        let def = EnumDef {
            name: "Role".to_string(),
            variants: vec![
                VariantDef { name: "Admin".to_string(), rename: None, kind: VariantKind::Unit, doc: None },
                VariantDef { name: "User".to_string(), rename: None, kind: VariantKind::Unit, doc: None },
            ],
            representation: EnumRepr::External,
            doc: None,
        };

        let output = m.emit_enum(&def);
        assert!(output.contains("enum Role {"));
        assert!(output.contains("@JsonValue('Admin')"));
        assert!(output.contains("@JsonValue('User')"));
    }

    #[test]
    fn test_camel_case_conversion() {
        assert_eq!(DartMapper::to_camel_case("user_name"), "userName");
        assert_eq!(DartMapper::to_camel_case("id"), "id");
        assert_eq!(DartMapper::to_camel_case("created_at"), "createdAt");
    }

    #[test]
    fn test_plugin_metadata() {
        let plugin = DartPlugin::new();
        assert_eq!(plugin.language_id(), "dart");
        assert_eq!(plugin.language_name(), "Dart/Flutter");
        assert_eq!(plugin.file_extension(), "dart");
    }
}
