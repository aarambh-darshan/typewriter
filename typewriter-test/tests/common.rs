//! Shared test helpers for enum representation snapshot tests.

use typewriter_core::ir::*;

/// External enum with data-carrying variants.
pub fn external_enum_def() -> TypeDef {
    TypeDef::Enum(EnumDef {
        name: "Shape".to_string(),
        variants: vec![
            VariantDef {
                name: "Circle".to_string(),
                rename: None,
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "radius".to_string(),
                    ty: TypeKind::Primitive(PrimitiveType::F64),
                    optional: false,
                    rename: None,
                    doc: None,
                    skip: false,
                    flatten: false,
                    type_override: None,
                }]),
                doc: None,
            },
            VariantDef {
                name: "Rectangle".to_string(),
                rename: None,
                kind: VariantKind::Struct(vec![
                    FieldDef {
                        name: "width".to_string(),
                        ty: TypeKind::Primitive(PrimitiveType::F64),
                        optional: false,
                        rename: None,
                        doc: None,
                        skip: false,
                        flatten: false,
                        type_override: None,
                    },
                    FieldDef {
                        name: "height".to_string(),
                        ty: TypeKind::Primitive(PrimitiveType::F64),
                        optional: false,
                        rename: None,
                        doc: None,
                        skip: false,
                        flatten: false,
                        type_override: None,
                    },
                ]),
                doc: None,
            },
            VariantDef {
                name: "Point".to_string(),
                rename: None,
                kind: VariantKind::Unit,
                doc: None,
            },
        ],
        representation: EnumRepr::External,
        doc: Some("Geometric shape".to_string()),
    })
}

/// Adjacent enum with tag + content.
pub fn adjacent_enum_def() -> TypeDef {
    TypeDef::Enum(EnumDef {
        name: "Event".to_string(),
        variants: vec![
            VariantDef {
                name: "Click".to_string(),
                rename: Some("click".to_string()),
                kind: VariantKind::Struct(vec![
                    FieldDef {
                        name: "x".to_string(),
                        ty: TypeKind::Primitive(PrimitiveType::I32),
                        optional: false,
                        rename: None,
                        doc: None,
                        skip: false,
                        flatten: false,
                        type_override: None,
                    },
                    FieldDef {
                        name: "y".to_string(),
                        ty: TypeKind::Primitive(PrimitiveType::I32),
                        optional: false,
                        rename: None,
                        doc: None,
                        skip: false,
                        flatten: false,
                        type_override: None,
                    },
                ]),
                doc: None,
            },
            VariantDef {
                name: "KeyPress".to_string(),
                rename: Some("key_press".to_string()),
                kind: VariantKind::Tuple(vec![TypeKind::Primitive(PrimitiveType::String)]),
                doc: None,
            },
            VariantDef {
                name: "Resize".to_string(),
                rename: Some("resize".to_string()),
                kind: VariantKind::Unit,
                doc: None,
            },
        ],
        representation: EnumRepr::Adjacent {
            tag: "t".to_string(),
            content: "c".to_string(),
        },
        doc: None,
    })
}

/// Untagged enum — no discriminator.
pub fn untagged_enum_def() -> TypeDef {
    TypeDef::Enum(EnumDef {
        name: "StringOrNumber".to_string(),
        variants: vec![
            VariantDef {
                name: "Text".to_string(),
                rename: None,
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "value".to_string(),
                    ty: TypeKind::Primitive(PrimitiveType::String),
                    optional: false,
                    rename: None,
                    doc: None,
                    skip: false,
                    flatten: false,
                    type_override: None,
                }]),
                doc: None,
            },
            VariantDef {
                name: "Number".to_string(),
                rename: None,
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "value".to_string(),
                    ty: TypeKind::Primitive(PrimitiveType::F64),
                    optional: false,
                    rename: None,
                    doc: None,
                    skip: false,
                    flatten: false,
                    type_override: None,
                }]),
                doc: None,
            },
        ],
        representation: EnumRepr::Untagged,
        doc: None,
    })
}
