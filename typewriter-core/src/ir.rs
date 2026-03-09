//! Internal Representation (IR) for typewriter.
//!
//! The IR is the language-agnostic bridge between Rust's AST and each language emitter.
//! Every emitter only needs to know about these types — it never touches `syn` or Rust AST directly.

/// Rust primitive types that map to language-specific equivalents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType {
    String,
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Uuid,
    DateTime,
    NaiveDate,
    JsonValue,
}

/// Every Rust type is mapped to one of these variants.
///
/// The IR is intentionally lossy — it captures only what is needed for
/// *type generation*, not execution semantics.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    /// A Rust primitive type (String, u32, bool, f64, etc.)
    Primitive(PrimitiveType),
    /// `Option<T>` — becomes optional/nullable in target languages
    Option(Box<TypeKind>),
    /// `Vec<T>` — becomes array/list in target languages
    Vec(Box<TypeKind>),
    /// `HashMap<K, V>` — becomes dict/record/map in target languages
    HashMap(Box<TypeKind>, Box<TypeKind>),
    /// Tuple types `(A, B, C)`
    Tuple(Vec<TypeKind>),
    /// A custom struct/enum referenced by name
    Named(String),
    /// A generic type `MyType<T, U>`
    Generic(String, Vec<TypeKind>),
    /// The unit type `()`
    Unit,
}

/// A single field inside a struct.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    /// The original Rust field name
    pub name: String,
    /// The type of this field in the IR
    pub ty: TypeKind,
    /// Whether this field is optional (`Option<T>` or `#[tw(optional)]`)
    pub optional: bool,
    /// Renamed field name from `#[serde(rename = "x")]` or `#[tw(rename = "x")]`
    pub rename: Option<String>,
    /// Documentation comment from `///` doc comments
    pub doc: Option<String>,
    /// Whether to skip this field in generated output (`#[tw(skip)]` or `#[serde(skip)]`)
    pub skip: bool,
    /// Whether this field is flattened (`#[serde(flatten)]`)
    pub flatten: bool,
}

/// A complete struct definition in the IR.
#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    /// The struct name
    pub name: String,
    /// All fields in this struct
    pub fields: Vec<FieldDef>,
    /// Documentation comment
    pub doc: Option<String>,
    /// Generic type parameter names, e.g. `["T", "U"]`
    pub generics: Vec<String>,
}

/// A single enum variant.
#[derive(Debug, Clone, PartialEq)]
pub struct VariantDef {
    /// The variant name
    pub name: String,
    /// Renamed variant name
    pub rename: Option<String>,
    /// The kind of data this variant carries
    pub kind: VariantKind,
    /// Documentation comment
    pub doc: Option<String>,
}

/// The kind of data an enum variant carries.
#[derive(Debug, Clone, PartialEq)]
pub enum VariantKind {
    /// A unit variant: `Variant`
    Unit,
    /// A tuple variant: `Variant(A, B)`
    Tuple(Vec<TypeKind>),
    /// A struct variant: `Variant { field: Type }`
    Struct(Vec<FieldDef>),
}

/// How the enum is represented in JSON (mirrors serde's options).
#[derive(Debug, Clone, PartialEq)]
pub enum EnumRepr {
    /// Default: `{ "VariantName": { ...data } }`
    External,
    /// `#[serde(tag = "type")]` → `{ "type": "variant_name", ...data }`
    Internal { tag: String },
    /// `#[serde(tag = "t", content = "c")]` → `{ "t": "variant_name", "c": ...data }`
    Adjacent { tag: String, content: String },
    /// `#[serde(untagged)]` — no discriminator
    Untagged,
}

/// A complete enum definition in the IR.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    /// The enum name
    pub name: String,
    /// All variants in this enum
    pub variants: Vec<VariantDef>,
    /// The JSON representation strategy
    pub representation: EnumRepr,
    /// Documentation comment
    pub doc: Option<String>,
}

/// Top-level item — either a struct or an enum.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeDef {
    Struct(StructDef),
    Enum(EnumDef),
}

impl TypeDef {
    /// Get the name of this type definition.
    pub fn name(&self) -> &str {
        match self {
            TypeDef::Struct(s) => &s.name,
            TypeDef::Enum(e) => &e.name,
        }
    }
}

/// The target languages that typewriter can generate code for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    TypeScript,
    Python,
    Go,
    Swift,
    Kotlin,
}

impl Language {
    /// Parse a language name from a string (case-insensitive).
    pub fn from_str(s: &str) -> Option<Language> {
        match s.to_lowercase().as_str() {
            "typescript" | "ts" => Some(Language::TypeScript),
            "python" | "py" => Some(Language::Python),
            "go" | "golang" => Some(Language::Go),
            "swift" => Some(Language::Swift),
            "kotlin" | "kt" => Some(Language::Kotlin),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_def_name() {
        let struct_def = TypeDef::Struct(StructDef {
            name: "User".to_string(),
            fields: vec![],
            doc: None,
            generics: vec![],
        });
        assert_eq!(struct_def.name(), "User");

        let enum_def = TypeDef::Enum(EnumDef {
            name: "Role".to_string(),
            variants: vec![],
            representation: EnumRepr::External,
            doc: None,
        });
        assert_eq!(enum_def.name(), "Role");
    }

    #[test]
    fn test_language_parsing() {
        assert_eq!(Language::from_str("typescript"), Some(Language::TypeScript));
        assert_eq!(Language::from_str("ts"), Some(Language::TypeScript));
        assert_eq!(Language::from_str("Python"), Some(Language::Python));
        assert_eq!(Language::from_str("GO"), Some(Language::Go));
        assert_eq!(Language::from_str("swift"), Some(Language::Swift));
        assert_eq!(Language::from_str("kotlin"), Some(Language::Kotlin));
        assert_eq!(Language::from_str("kt"), Some(Language::Kotlin));
        assert_eq!(Language::from_str("ruby"), None);
    }

    #[test]
    fn test_field_def_construction() {
        let field = FieldDef {
            name: "email".to_string(),
            ty: TypeKind::Primitive(PrimitiveType::String),
            optional: false,
            rename: None,
            doc: Some("User email address".to_string()),
            skip: false,
            flatten: false,
        };
        assert_eq!(field.name, "email");
        assert!(!field.optional);
        assert!(!field.skip);
    }

    #[test]
    fn test_option_type_kind() {
        let ty = TypeKind::Option(Box::new(TypeKind::Primitive(PrimitiveType::U32)));
        match &ty {
            TypeKind::Option(inner) => {
                assert_eq!(**inner, TypeKind::Primitive(PrimitiveType::U32));
            }
            _ => panic!("Expected Option type kind"),
        }
    }

    #[test]
    fn test_complex_type_kind() {
        // HashMap<String, Vec<u32>>
        let ty = TypeKind::HashMap(
            Box::new(TypeKind::Primitive(PrimitiveType::String)),
            Box::new(TypeKind::Vec(Box::new(TypeKind::Primitive(
                PrimitiveType::U32,
            )))),
        );
        match &ty {
            TypeKind::HashMap(k, v) => {
                assert_eq!(**k, TypeKind::Primitive(PrimitiveType::String));
                match v.as_ref() {
                    TypeKind::Vec(inner) => {
                        assert_eq!(**inner, TypeKind::Primitive(PrimitiveType::U32));
                    }
                    _ => panic!("Expected Vec type kind"),
                }
            }
            _ => panic!("Expected HashMap type kind"),
        }
    }

    #[test]
    fn test_enum_repr() {
        let repr = EnumRepr::Internal {
            tag: "type".to_string(),
        };
        match &repr {
            EnumRepr::Internal { tag } => assert_eq!(tag, "type"),
            _ => panic!("Expected internal repr"),
        }
    }
}
