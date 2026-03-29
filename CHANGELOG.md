# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

*No unreleased changes yet.*

---

## [0.4.1] - 2026-03-29

### Added

#### JSON Schema Emitter (`typewriter-json-schema`)
- **New `typewriter-json-schema` crate** — generates `.schema.json` files conforming to JSON Schema Draft 2020-12 from Rust structs and enums.
- **Struct → `object`** — Rust structs map to `{ "type": "object", "properties": {...}, "required": [...] }` with `additionalProperties: false`.
- **Simple enum → `string` enum** — all-unit Rust enums map to `{ "type": "string", "enum": [...] }`.
- **Data-carrying enum → `oneOf`** — each variant becomes a sub-schema, tied together with `oneOf` composition.
- **All serde enum representations** — supports `External`, `Internal`, `Adjacent`, and `Untagged` representations with appropriate discriminator `const` fields.
- **Format annotations** — `Uuid` → `"format": "uuid"`, `DateTime` → `"format": "date-time"`, `NaiveDate` → `"format": "date"`.
- **Type mapping** — `String`→`string`, `bool`→`boolean`, integer types→`integer`, float types→`number`, `Vec<T>`→`array`, `HashMap<K,V>`→`object` with `additionalProperties`, tuples→`prefixItems`.
- **Feature-gated** — `json_schema` feature flag in `typewriter-engine`, `typewriter-macros`, and `typebridge` crates (enabled by default).
- **Configuration** — `[json_schema]` section in `typewriter.toml` with `output_dir` and `file_style` settings.
- Added `Language::JsonSchema` variant to core IR with `"json_schema"` / `"jsonschema"` string parsing.
- 15 unit tests in `typewriter-json-schema` and 11 snapshot tests in `typewriter-test`.

### Changed
- Bumped CLI version to `0.2.1`.

## [0.4.0] - 2026-03-28

### Added

#### GraphQL SDL Emitter (`typewriter-graphql`)
- **New `typewriter-graphql` crate** — generates `.graphql` Schema Definition Language files from Rust types.
- **Struct → `type`** — Rust structs map to GraphQL `type` declarations with `!` for non-null fields and nullable for `Option<T>`.
- **Simple enum → `enum`** — all-unit Rust enums map to GraphQL `enum` declarations.
- **Data-carrying enum → `union` + types** — each variant becomes its own `type`, tied together with a `union` declaration.
- **Custom scalars** — `DateTime` and `JSON` custom scalar declarations are auto-emitted when needed.
- **Doc comments** — Rust `///` doc comments render as GraphQL `"""` description blocks.
- **All serde enum representations** — supports `External`, `Internal`, `Adjacent`, and `Untagged` representations with appropriate discriminator fields.
- **Type mapping** — `String`→`String`, `bool`→`Boolean`, `u32`→`Int`, `f64`→`Float`, `Uuid`→`ID`, `HashMap`→`JSON`, `Vec<T>`→`[T!]`.
- **Feature-gated** — `graphql` feature flag in `typewriter-engine`, `typewriter-macros`, and `typebridge` crates (enabled by default).
- **Configuration** — `[graphql]` section in `typewriter.toml` with `output_dir` and `file_style` settings.
- Added `Language::GraphQL` variant to core IR with `"graphql"` / `"gql"` string parsing.
- 9 unit tests in `typewriter-graphql` and 11 snapshot tests in `typewriter-test`.

---

## [0.3.1] - 2026-03-25

### Added
- Phase 4.1 complete: TypeScript generation now emits sibling Zod schema files (`<type>.schema.ts`) alongside interface/union files (`<type>.ts`) by default.
- Zod schema generation supports structs, generic schema factories, and all serde enum representations (`External`, `Internal`, `Adjacent`, `Untagged`).
- `typewriter-engine`, CLI commands (`generate`, `check`, `watch`), and proc-macro generation now include schema artifacts in render and drift flows.
- Added Zod schema controls: global `[typescript].zod = false` to disable schema output, plus per-type `#[tw(zod)]`/`#[tw(zod = false)]` overrides for selective generation.

### Changed
- Updated Rust edition to 2024.

---

## [0.3.0] - 2026-03-22

### Added
- New shared crate **`typewriter-engine`** for reusable AST parsing, source scanning, rendering, and drift detection.
- New standalone **`typebridge-cli`** binary with `generate`, `check`, and `watch` subcommands.
- New Cargo plugin binary **`cargo-typewriter`** for `cargo typewriter ...` command parity.
- Structured JSON drift reporting with `typewriter check --json` and `--json-out <path>`.
- GitHub Actions workflow example at `.github/workflows/typewriter-check.yml` using `typewriter check --ci`.
- Published `typebridge-cli v0.1.0` to crates.io.

### Changed
- `typewriter-macros` now delegates parse + emit orchestration to `typewriter-engine`.
- CLI `check --ci` now exits non-zero when drift is detected (`out_of_sync`, `missing`, `orphaned`).
- Human-friendly colored terminal output added across CLI commands.

---

## [0.2.0] - 2026-03-19

### Added

#### New Emitters
- **Swift Emitter** (`typewriter-swift`) — generates `Codable` structs and enums.
- **Kotlin Emitter** (`typewriter-kotlin`) — generates `data class`es and `sealed class`es with `kotlinx.serialization`.

#### Core Features
- **Comprehensive Enum Support**: All 5 emitters now fully support serde's `External`, `Internal`, `Adjacent`, and `Untagged` representation formats.
- **Type Overrides**: Added `#[tw(type = "X")]` attribute parsing to force specific generated types per-field.
- **Flatten Support**: Added support for `#[serde(flatten)]` by emitting `// @flatten` comments as a standardized signal in generated code.
- **Testing Infrastructure**: Added strict `trybuild` compile-error tests to prevent misuse of `#[derive(TypeWriter)]` attributes. Over 15 new snapshot tests for advanced enum representations.

---

## [0.1.3] - 2026-03-14

### Added

#### Go Emitter (`typewriter-go`)
- **Native struct generation** — mapping Rust structures strictly to JSON-tagged structs with capital case exportability rules.
- **Interface discriminated unions** — mapped enum data types to an internal interface type with custom structs binding the generic trait.
- **`omitempty` pointers** — translating `Option<T>` fields to `*T` with `json:\"field,omitempty\"` tagging natively in Go.
- **Unit enum constants** — implemented simple enum variations via string aliases and block `const` structures.
- Updated all cross-workspace macro plumbing (`typewriter-core`, `typewriter-macros`, `typewriter`) to integrate the `go` configuration key.
- Full workspace test snapshot integration.

---

## [0.1.2] - 2026-03-13

### Added

#### Generic Type Support (Phase 2)
- **Full generic struct support** — `Pagination<T>` generates `export interface Pagination<T>` (TS) and `class Pagination(BaseModel, Generic[T])` (Python)
- `map_generic()` method on `TypeMapper` trait — default `Name<A, B>` format, Python overrides with `Name[A, B]`
- Generic structs in Python auto-generate `TypeVar` declarations and `from typing import Generic, TypeVar` imports
- Nested generics work: `Vec<Pagination<User>>` → `Pagination<User>[]` (TS) / `list[Pagination[User]]` (Python)

#### Cross-File Import Generation
- **Auto-generated imports** when a struct references another type:
  - TypeScript: `import type { FilterUserDto } from './filter-user-dto';`
  - Python: `from .filter_user_dto import FilterUserDto`
- `collect_referenced_types()` on `TypeDef` — recursively extracts external type names from all fields
- `emit_imports()` on `TypeMapper` trait — language-specific import rendering
- Works with `Vec<X>`, `Option<X>`, `HashMap<K, V>`, `Generic<X>`, and deeply nested references
- Excludes the struct's own generic params (`T`, `U`) from imports

#### Example
- Added `Pagination<T>` generic struct to the example crate

---

## [0.1.1] - 2026-03-10

### Fixed

- **`file_style` configuration now works** — `snake_case` and `PascalCase` were being ignored in `typewriter.toml`; only `kebab-case` was generated regardless of config. The `file_naming()` method in both TypeScript and Python mappers now correctly reads and applies the configured style.

### Added

#### Core (`typewriter-core`)
- New `naming` module with shared `FileStyle` enum and `to_file_style()` conversion function
- `ts_file_style()` and `py_file_style()` helper methods on `TypewriterConfig`
- `file_style` field added to `PythonConfig` (TypeScript already had it)
- 5 new unit tests for file naming utilities

#### TypeScript Emitter (`typewriter-typescript`)
- `file_style` field on `TypeScriptMapper` with `with_file_style()` builder
- 4 new tests: `test_file_naming_snake`, `test_file_naming_pascal`, `test_output_filename_pascal`

#### Python Emitter (`typewriter-python`)
- `file_style` field on `PythonMapper` with `with_file_style()` builder
- 5 new tests: `test_file_naming_kebab`, `test_file_naming_pascal`, `test_output_filename_pascal`

#### Emitter Dispatcher
- Now passes `file_style` from `typewriter.toml` config to both mappers

### Changed

- All code examples updated: `use typewriter::TypeWriter` → `use typebridge::TypeWriter` across all `.rs` files, docs, READMEs, and content files
- Example crate dependency key renamed from `typewriter` to `typebridge` in `Cargo.toml`
- Removed duplicate `to_kebab_case()` and `to_snake_case()` from individual mappers — now uses shared `naming` module
- **53 total tests — all passing** (was 41)

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
- `ROADMAP.md` with 5 phases
- `CHANGELOG.md` (this file)
- `docs/` folder with 7 detailed guides
- Working `example/` crate with 5 use cases

---

[unreleased]: https://github.com/aarambh-darshan/typewriter/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/aarambh-darshan/typewriter/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/aarambh-darshan/typewriter/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/aarambh-darshan/typewriter/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/aarambh-darshan/typewriter/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/aarambh-darshan/typewriter/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/aarambh-darshan/typewriter/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/aarambh-darshan/typewriter/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/aarambh-darshan/typewriter/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/aarambh-darshan/typewriter/releases/tag/v0.1.0

