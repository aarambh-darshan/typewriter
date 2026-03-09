# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Planned
- Go emitter (`typewriter-go`)
- Swift emitter (`typewriter-swift`)
- Kotlin emitter (`typewriter-kotlin`)
- CLI tool with `generate`, `check`, `watch` commands

---

## [0.1.0] - 2026-03-09

### 🎉 Initial Release — MVP

> Phase 1 complete: TypeScript and Python type generation from Rust structs and enums.

### Added

#### Core (`typewriter-core`)
- Internal Representation (IR) types: `TypeDef`, `StructDef`, `EnumDef`, `FieldDef`, `TypeKind`, `VariantDef`, `EnumRepr`
- `TypeMapper` trait — the core contract every language emitter implements
- `typewriter.toml` configuration parsing with sensible defaults
- `Language` enum with string parsing (`typescript`, `python`, `go`, `swift`, `kotlin`)

#### Proc Macro (`typewriter-macros`)
- `#[derive(TypeWriter)]` proc macro using `syn` 2.x
- `#[sync_to(typescript, python)]` attribute for target language selection
- Full `syn::DeriveInput` → IR parser for structs and enums
- Serde attribute compatibility:
  - `#[serde(rename = "x")]` — field/variant renaming
  - `#[serde(skip)]` / `#[serde(skip_serializing)]` — field exclusion
  - `#[serde(tag = "type")]` — internally tagged enums
  - `#[serde(tag = "t", content = "c")]` — adjacently tagged enums
  - `#[serde(untagged)]` — untagged enums
  - `#[serde(flatten)]` — field flattening detection
- Custom `#[tw(...)]` attributes:
  - `#[tw(skip)]` — exclude field from output
  - `#[tw(rename = "x")]` — override field name
  - `#[tw(optional)]` — force optional even if not `Option<T>`
- Doc comment extraction (`///` → JSDoc / Python comments)
- Smart type unwrapping for `Box<T>`, `Arc<T>`, `Rc<T>`
- Automatic file generation on `cargo build`
- Feature-gated emitter dispatch

#### TypeScript Emitter (`typewriter-typescript`)
- `export interface` generation from Rust structs
- Optional fields: `Option<T>` → `field?: T | undefined`
- String literal union types for simple enums
- Discriminated union types for tagged enums
- `readonly` mode support
- Kebab-case file naming (`UserProfile` → `user-profile.ts`)
- JSDoc comments from Rust doc comments
- Full type mapping: `String`→`string`, `u32`→`number`, `u64`→`bigint`, `Vec<T>`→`T[]`, `HashMap<K,V>`→`Record<K,V>`, etc.

#### Python Emitter (`typewriter-python`)
- Pydantic v2 `BaseModel` class generation
- `Optional[T] = None` for `Option<T>` fields
- `class Role(str, Enum)` for simple enums
- `Union[...]` with `Literal` discriminators for tagged enums
- Automatic import collection and deduplication
- Snake_case file naming (`UserProfile` → `user_profile.py`)
- Python comments from Rust doc comments
- Full type mapping: `String`→`str`, `u32`→`int`, `Vec<T>`→`list[T]`, `HashMap<K,V>`→`dict[K,V]`, `Uuid`→`UUID`, etc.

#### Testing
- 11 unit tests for `typewriter-core` (IR types, config parsing)
- 11 unit tests for `typewriter-typescript` (type mappings, emission)
- 10 unit tests for `typewriter-python` (type mappings, emission)
- 9 `insta` snapshot tests for integration testing
- **41 total tests — all passing**

#### Documentation
- `README.md` with quick start, examples, type mapping reference
- `ARCHITECTURE.md` with data flow diagrams and crate relationships
- `CONTRIBUTING.md` with development setup and emitter guide
- `ROADMAP.md` with 5 phases and YouTube series plan
- `CHANGELOG.md` (this file)
- `docs/` folder with 7 detailed guides
- Working `example/` crate with 5 use cases

---

[unreleased]: https://github.com/aarambh-darshan/typewriter/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/aarambh-darshan/typewriter/releases/tag/v0.1.0
