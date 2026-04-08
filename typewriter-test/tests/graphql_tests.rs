//! Snapshot tests for the GraphQL SDL emitter.

use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;
use typewriter_graphql::GraphQLMapper;

#[test]
fn test_simple_struct_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Struct(StructDef {
        name: "UserProfile".to_string(),
        fields: vec![
            FieldDef {
                name: "id".to_string(),
                ty: TypeKind::Primitive(PrimitiveType::Uuid),
                optional: false,
                rename: None,
                doc: Some("Unique identifier".to_string()),
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "email".to_string(),
                ty: TypeKind::Primitive(PrimitiveType::String),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "age".to_string(),
                ty: TypeKind::Option(Box::new(TypeKind::Primitive(PrimitiveType::U32))),
                optional: true,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "role".to_string(),
                ty: TypeKind::Named("UserRole".to_string()),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "created_at".to_string(),
                ty: TypeKind::Primitive(PrimitiveType::DateTime),
                optional: false,
                rename: Some("createdAt".to_string()),
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
        ],
        doc: Some("User profile information".to_string()),
        generics: vec![],
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_struct_with_collections_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Struct(StructDef {
        name: "ApiResponse".to_string(),
        fields: vec![
            FieldDef {
                name: "items".to_string(),
                ty: TypeKind::Vec(Box::new(TypeKind::Named("Item".to_string()))),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "metadata".to_string(),
                ty: TypeKind::HashMap(
                    Box::new(TypeKind::Primitive(PrimitiveType::String)),
                    Box::new(TypeKind::Primitive(PrimitiveType::JsonValue)),
                ),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "coordinates".to_string(),
                ty: TypeKind::Tuple(vec![
                    TypeKind::Primitive(PrimitiveType::F64),
                    TypeKind::Primitive(PrimitiveType::F64),
                ]),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
        ],
        doc: None,
        generics: vec![],
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_simple_enum_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Enum(EnumDef {
        name: "Role".to_string(),
        variants: vec![
            VariantDef {
                name: "Admin".to_string(),
                rename: None,
                kind: VariantKind::Unit,
                doc: None,
            },
            VariantDef {
                name: "User".to_string(),
                rename: None,
                kind: VariantKind::Unit,
                doc: None,
            },
            VariantDef {
                name: "Guest".to_string(),
                rename: None,
                kind: VariantKind::Unit,
                doc: None,
            },
        ],
        representation: EnumRepr::External,
        doc: None,
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_tagged_enum_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Enum(EnumDef {
        name: "PaymentStatus".to_string(),
        variants: vec![
            VariantDef {
                name: "Pending".to_string(),
                rename: Some("pending".to_string()),
                kind: VariantKind::Unit,
                doc: None,
            },
            VariantDef {
                name: "Completed".to_string(),
                rename: Some("completed".to_string()),
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "transaction_id".to_string(),
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
                name: "Failed".to_string(),
                rename: Some("failed".to_string()),
                kind: VariantKind::Struct(vec![
                    FieldDef {
                        name: "reason".to_string(),
                        ty: TypeKind::Primitive(PrimitiveType::String),
                        optional: false,
                        rename: None,
                        doc: None,
                        skip: false,
                        flatten: false,
                        type_override: None,
                    },
                    FieldDef {
                        name: "code".to_string(),
                        ty: TypeKind::Primitive(PrimitiveType::U32),
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
        ],
        representation: EnumRepr::Internal {
            tag: "type".to_string(),
        },
        doc: Some("Payment processing status".to_string()),
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

mod common;

#[test]
fn test_external_enum_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = common::external_enum_def();
    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_adjacent_enum_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = common::adjacent_enum_def();
    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_untagged_enum_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = common::untagged_enum_def();
    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_generic_struct_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Struct(StructDef {
        name: "Pagination".to_string(),
        fields: vec![
            FieldDef {
                name: "items".to_string(),
                ty: TypeKind::Vec(Box::new(TypeKind::Named("T".to_string()))),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "total".to_string(),
                ty: TypeKind::Primitive(PrimitiveType::U32),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "page".to_string(),
                ty: TypeKind::Primitive(PrimitiveType::U32),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
        ],
        doc: Some("Paginated response wrapper".to_string()),
        generics: vec!["T".to_string()],
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_skipped_field_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Struct(StructDef {
        name: "User".to_string(),
        fields: vec![
            FieldDef {
                name: "email".to_string(),
                ty: TypeKind::Primitive(PrimitiveType::String),
                optional: false,
                rename: None,
                doc: None,
                skip: false,
                flatten: false,
                type_override: None,
            },
            FieldDef {
                name: "password_hash".to_string(),
                ty: TypeKind::Primitive(PrimitiveType::String),
                optional: false,
                rename: None,
                doc: None,
                skip: true,
                flatten: false,
                type_override: None,
            },
        ],
        doc: None,
        generics: vec![],
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_renamed_field_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Struct(StructDef {
        name: "User".to_string(),
        fields: vec![FieldDef {
            name: "user_name".to_string(),
            ty: TypeKind::Primitive(PrimitiveType::String),
            optional: false,
            rename: Some("userName".to_string()),
            doc: None,
            skip: false,
            flatten: false,
            type_override: None,
        }],
        doc: None,
        generics: vec![],
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}

#[test]
fn test_tuple_variant_enum_snapshot() {
    let mapper = GraphQLMapper::new();
    let def = TypeDef::Enum(EnumDef {
        name: "Message".to_string(),
        variants: vec![
            VariantDef {
                name: "Text".to_string(),
                rename: None,
                kind: VariantKind::Tuple(vec![TypeKind::Primitive(PrimitiveType::String)]),
                doc: Some("A text message".to_string()),
            },
            VariantDef {
                name: "Binary".to_string(),
                rename: None,
                kind: VariantKind::Tuple(vec![TypeKind::Vec(Box::new(TypeKind::Primitive(
                    PrimitiveType::U8,
                )))]),
                doc: None,
            },
            VariantDef {
                name: "Ping".to_string(),
                rename: None,
                kind: VariantKind::Unit,
                doc: None,
            },
        ],
        representation: EnumRepr::External,
        doc: Some("WebSocket message types".to_string()),
    });

    let output = mapper.emit_type_def(&def);
    insta::assert_snapshot!(output);
}
