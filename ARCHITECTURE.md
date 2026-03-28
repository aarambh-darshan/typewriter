# 🏗️ Architecture

> Technical deep-dive into how typewriter works internally.

---

## Overview

typewriter is a **Cargo workspace** split into focused, independently publishable crates. This separation is required because proc macro crates have special compilation rules in Rust — they must be in their own crate.

```
typewriter/
│
├── typewriter-core/            ← 🧱 Shared IR types, traits, config (zero proc-macro deps)
├── typewriter-engine/          ← ⚙️ Shared scan/parse/emit + drift orchestration
├── typewriter-macros/          ← 🔧 Proc macro crate (#[derive(TypeWriter)])
├── typewriter-typescript/      ← 🟦 TypeScript emitter
├── typewriter-python/          ← 🐍 Python Pydantic emitter
├── typewriter-go/              ← 🐹 Go emitter
├── typewriter-swift/           ← 🦅 Swift emitter
├── typewriter-kotlin/          ← 🚀 Kotlin emitter
├── typewriter-graphql/         ← 🔷 GraphQL SDL emitter
├── typewriter-cli/             ← 💻 CLI (typewriter + cargo-typewriter binaries)
├── typewriter/                 ← 📦 Main user-facing crate (re-exports)
├── typewriter-test/            ← 🧪 Integration + snapshot tests
└── example/                    ← 📝 Working examples
```

### Why this structure?

- **`typewriter-core` has zero proc-macro dependencies** — it can be used in build scripts, CLI tools, and regular code.
- **Each language emitter is its own crate** — users only compile the emitters they need via feature flags.
- **`typewriter-macros` is isolated** — proc macro crates are special in Rust's compilation model.

---

## Data Flow

This is how your Rust struct becomes a TypeScript interface or Python model:

```
  ┌─────────────────────────────────────────────┐
  │  Your Rust source file (.rs)                │
  │                                             │
  │  #[derive(TypeWriter)]                      │
  │  #[sync_to(typescript, python)]             │
  │  pub struct UserProfile { ... }             │
  └──────────────────────┬──────────────────────┘
                         │  cargo build triggers proc macro
                         ▼
  ┌─────────────────────────────────────────────┐
  │  typewriter-macros :: parser.rs             │
  │                                             │
  │  syn 2.x parses TokenStream → DeriveInput  │
  │  Reads struct fields, types, attributes     │
  │  Reads #[serde(rename)], #[tw(skip)], etc.  │
  └──────────────────────┬──────────────────────┘
                         │
                         ▼
  ┌─────────────────────────────────────────────┐
  │  typewriter-core :: IR                      │
  │                                             │
  │  StructDef {                                │
  │    name: "UserProfile",                     │
  │    fields: [                                │
  │      FieldDef { name: "id", ty: Uuid },     │
  │      FieldDef { name: "age",                │
  │                 ty: Option(u32),            │
  │                 optional: true },           │
  │    ]                                        │
  │  }                                          │
  └──────────────────────┬──────────────────────┘
                         │  dispatched to each requested emitter
           ┌──────────────┼──────────────┐
           ▼              ▼              ▼
     TypeScript       Python         Go, Swift,
     emitter          emitter        Kotlin, GraphQL
           │              │
           ▼              ▼
   user-profile.ts  user_profile.py
   (written to configured output directories)
```

---

## Internal Representation (IR)

The IR sits in `typewriter-core/src/ir.rs` and is the language-agnostic bridge between Rust's AST and each emitter. Emitters never touch `syn` or Rust AST types directly.

### Core Types

| Type | Purpose |
|---|---|
| `PrimitiveType` | Enum of all supported primitive types (String, Bool, U8–U128, F32, F64, Uuid, DateTime, etc.) |
| `TypeKind` | Recursive type representation: `Primitive`, `Option`, `Vec`, `HashMap`, `Tuple`, `Named`, `Generic`, `Unit` |
| `FieldDef` | A struct field: name, type, optional flag, rename, doc, skip, flatten |
| `StructDef` | A complete struct: name, fields, doc, generics |
| `VariantDef` | An enum variant: name, rename, kind (Unit/Tuple/Struct), doc |
| `EnumRepr` | JSON representation strategy: `External`, `Internal`, `Adjacent`, `Untagged` |
| `EnumDef` | A complete enum: name, variants, representation, doc |
| `TypeDef` | Top-level: either `Struct(StructDef)` or `Enum(EnumDef)` |

### Design Principle

The IR is intentionally **lossy** — it captures only what is needed for type generation, not Rust execution semantics. This keeps every language emitter simple and focused.

---

## TypeMapper Trait

Every language emitter implements `TypeMapper` (defined in `typewriter-core/src/mapper.rs`). Adding a new language to typewriter means implementing this one trait:

```rust
pub trait TypeMapper {
    fn map_primitive(&self, ty: &PrimitiveType) -> String;
    fn map_option(&self, inner: &TypeKind) -> String;
    fn map_vec(&self, inner: &TypeKind) -> String;
    fn map_hashmap(&self, key: &TypeKind, value: &TypeKind) -> String;
    fn map_tuple(&self, elements: &[TypeKind]) -> String;
    fn map_named(&self, name: &str) -> String;
    fn emit_struct(&self, def: &StructDef) -> String;
    fn emit_enum(&self, def: &EnumDef) -> String;
    fn file_header(&self, type_name: &str) -> String;
    fn file_extension(&self) -> &str;
    fn file_naming(&self, type_name: &str) -> String;

    // Default implementations provided:
    fn map_type(&self, ty: &TypeKind) -> String;
    fn emit_type_def(&self, def: &TypeDef) -> String;
    fn output_filename(&self, type_name: &str) -> String;
    fn file_footer(&self) -> String;
}
```

---

## Parser (Proc Macro)

`typewriter-macros/src/parser.rs` converts `syn::DeriveInput` → IR using the **syn 2.x** API:

- **`parse_type_def`** — Entry point. Routes to struct or enum parsing.
- **`parse_fields`** — Iterates `syn::Fields::Named`, maps each `syn::Type` → `TypeKind`.
- **`parse_type`** — Recursive type mapping. Handles `Option<T>`, `Vec<T>`, `HashMap<K,V>`, `Box<T>` (unwrapped), `Arc<T>`, primitives, and named types.
- **`parse_enum_repr`** — Reads `#[serde(tag, content, untagged)]` attributes.
- **`get_rename`** / **`has_serde_skip`** — Attribute extraction using `parse_nested_meta`.

---

## Emitter Dispatch

`typewriter-macros/src/emitter.rs` handles the file I/O:

1. Creates the correct `TypeMapper` implementation based on target language
2. Calls `emit_type_def()` to generate the content string
3. Creates the output directory if it doesn't exist
4. Writes the file with the correct name and extension

Emitters are **feature-gated** — if the `typescript` feature is disabled, the TypeScript emitter code isn't even compiled.

---

## Crate Dependency Graph

```
typewriter-core (no proc-macro deps)
    ↑
    ├── typewriter-typescript (depends on core)
    ├── typewriter-python (depends on core)
    ├── typewriter-go (depends on core)
    ├── typewriter-swift (depends on core)
    ├── typewriter-kotlin (depends on core)
    ├── typewriter-graphql (depends on core)
    │
    └── typewriter-engine (depends on core + all emitters via features)
            ↑
            ├── typewriter-macros (depends on core + engine)
            │       ↑
            │       └── typewriter (re-exports macros + core)
            │
            └── typewriter-cli (depends on core + engine)
```

---

## crates.io Publishing Order

Order matters. Each crate must be published before any crate that depends on it:

```
1. typewriter-core
2. typewriter-typescript
3. typewriter-python
4. typewriter-go
5. typewriter-swift
6. typewriter-kotlin
7. typewriter-graphql
8. typewriter-engine
9. typewriter-macros
10. typewriter (typebridge)
11. typewriter-cli (typebridge-cli)
```
