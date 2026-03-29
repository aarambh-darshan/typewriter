# 🗺️ Roadmap

> typewriter's development journey from MVP to ecosystem.

---

## Phase 1 — MVP `v0.1.1` ✅

> **Goal:** Working proc macro that generates TypeScript and Python from Rust structs/enums.

- [x] `typewriter-core` IR: `TypeDef`, `FieldDef`, `EnumDef`, `TypeKind`
- [x] `typewriter-macros`: `#[derive(TypeWriter)]` proc macro entry point
- [x] Parser: `syn` Rust AST → IR conversion for structs and simple enums
- [x] TypeScript emitter: interfaces, optional fields, basic enums, discriminated unions
- [x] Python emitter: Pydantic v2 `BaseModel`, `Enum`, `Union` with `Literal` discriminators
- [x] Basic `#[serde(rename)]` and `#[serde(skip)]` compatibility
- [x] `typewriter.toml` config file parsing
- [x] Unit tests + snapshot tests
- [x] Publish to crates.io as [`typebridge`](https://crates.io/crates/typebridge) v0.1.1

---

## Phase 2 — Core Completion `v0.2.0` ✅

> **Goal:** All 5 language emitters. Full enum support. Generics. Feature-complete proc macro API.

- [x] Go emitter: structs + interfaces (`*T` for `Option<T>`, tags)
- [x] Swift emitter: `Codable` structs + enums + `CodingKeys`
- [x] Kotlin emitter: data class + `kotlinx.serialization` + `@SerialName`
- [x] Enum support: all 4 serde representations (external, internal, adjacent, untagged)
- [x] Generic type support: `MyType<T>`, `Vec<MyType<T>>` — full nesting *(v0.1.2)*
- [x] Cross-file import generation: auto `import type` (TS) / `from .x import X` (Python) *(v0.1.2)*
- [x] Per-field attributes: `#[tw(rename)]`, `#[tw(skip)]`, `#[tw(type)]`
- [x] `#[serde(flatten)]` support
- [x] Snapshot test suite via `insta` for all 5 languages
- [x] Compile error tests via `trybuild` with clear error messages
- [x] Publish `typewriter v0.2.0`

---

## Phase 3 — CLI & Watch Mode `v0.3.0` ✅

> **Goal:** Standalone CLI with watch mode and CI integration.

- [x] `typebridge-cli`: `generate`, `check`, `watch` subcommands via `clap`
- [x] Watch mode with `notify` crate — sub-20ms regeneration on file save
- [x] `typewriter check --ci` for pipeline drift gate
- [x] GitHub Actions example workflow
- [x] Drift detection report in structured JSON format
- [x] `cargo typewriter` subcommand (Cargo plugin registration)
- [x] Colored, human-friendly terminal output
- [x] Publish `typebridge-cli v0.1.0`

---

## Phase 4 — Ecosystem & Polish `v1.0.0` *(Current)*

> **Goal:** Stable API. Extended output formats. Great documentation. Community ready.

- [x] Zod schema generation alongside TypeScript interfaces
- [x] GraphQL SDL type generation
- [x] JSON Schema output
- [ ] VSCode extension: hover over struct → see generated output
- [ ] Neovim plugin via LSP
- [ ] Comprehensive `docs.rs` documentation
- [ ] `mdBook`-based guide site
- [ ] Publish `typewriter v1.0.0` — stable public API guarantee

---

## Phase 5 — Advanced Features *(Month 10–12)*

> **Goal:** Plugin ecosystem. Community-contributed backends. Monorepo support.

- [ ] Custom emitter plugin API
- [ ] Ruby, PHP, Dart (Flutter) emitters via plugin system
- [ ] Migration guide generator (type change → changelog per language)
- [ ] Bidirectional sync: detect foreign language types, suggest Rust structs
- [ ] Web playground: paste Rust → see all language outputs live
- [ ] Full workspace support for Cargo monorepos

---

<div align="center">

*Have a feature request? [Open an issue](https://github.com/aarambh-darshan/typewriter/issues) on GitHub!*

</div>
