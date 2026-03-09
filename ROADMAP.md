# 🗺️ Roadmap

> typewriter's development journey from MVP to ecosystem.

---

## Phase 1 — MVP `v0.1.0` ✅ *(Current)*

> **Goal:** Working proc macro that generates TypeScript and Python from Rust structs/enums.

- [x] `typewriter-core` IR: `TypeDef`, `FieldDef`, `EnumDef`, `TypeKind`
- [x] `typewriter-macros`: `#[derive(TypeWriter)]` proc macro entry point
- [x] Parser: `syn` Rust AST → IR conversion for structs and simple enums
- [x] TypeScript emitter: interfaces, optional fields, basic enums, discriminated unions
- [x] Python emitter: Pydantic v2 `BaseModel`, `Enum`, `Union` with `Literal` discriminators
- [x] Basic `#[serde(rename)]` and `#[serde(skip)]` compatibility
- [x] `typewriter.toml` config file parsing
- [x] Unit tests + snapshot tests (41 tests passing)
- [ ] Publish `typewriter v0.1.0` to crates.io

---

## Phase 2 — Core Completion `v0.2.0` *(Month 3–4)*

> **Goal:** All 5 language emitters. Full enum support. Generics. Feature-complete proc macro API.

- [ ] Go emitter: structs + json tags + pointer types for `Option<T>`
- [ ] Swift emitter: `Codable` structs + enums + `CodingKeys`
- [ ] Kotlin emitter: data class + `kotlinx.serialization` + `@SerialName`
- [ ] Enum support: all 4 serde representations (external, internal, adjacent, untagged)
- [ ] Generic type support: `MyType<T>`, `Vec<MyType<T>>` — full nesting
- [ ] Per-field attributes: `#[tw(rename)]`, `#[tw(skip)]`, `#[tw(type)]`
- [ ] `#[serde(flatten)]` support
- [ ] Snapshot test suite via `insta` for all 5 languages
- [ ] Compile error tests via `trybuild` with clear error messages
- [ ] Publish `typewriter v0.2.0`

---

## Phase 3 — CLI & Watch Mode `v0.3.0` *(Month 5–6)*

> **Goal:** Standalone CLI with watch mode and CI integration.

- [ ] `typewriter-cli`: `generate`, `check`, `watch` subcommands via `clap`
- [ ] Watch mode with `notify` crate — sub-20ms regeneration on file save
- [ ] `typewriter check --ci` for pipeline drift gate
- [ ] GitHub Actions example workflow
- [ ] Drift detection report in structured JSON format
- [ ] `cargo typewriter` subcommand (Cargo plugin registration)
- [ ] Colored, human-friendly terminal output
- [ ] Publish `typewriter-cli v0.1.0`

---

## Phase 4 — Ecosystem & Polish `v1.0.0` *(Month 7–9)*

> **Goal:** Stable API. Extended output formats. Great documentation. Community ready.

- [ ] Zod schema generation alongside TypeScript interfaces
- [ ] GraphQL SDL type generation
- [ ] JSON Schema output
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

## 🎬 YouTube Series Plan

| Part | Title | Content |
|---|---|---|
| **1** | *Building a Rust Proc Macro from Scratch* | `syn`, `DeriveInput`, `TokenStream`, derive macros |
| **2** | *Generating TypeScript from Rust Types* | `TypeMapper` trait, TS emitter, `insta` snapshots |
| **3** | *Polyglot Type Sync — Python, Go, Swift* | 3 more emitters, enum handling, serde compat |
| **4** | *Building the CLI with Watch Mode* | `clap`, `notify` file watcher, CI drift gate |
| **5** | *Shipping to crates.io — Full Demo* | Publishing, CI, live demo with Rust + Next.js |

---

<div align="center">

*Have a feature request? [Open an issue](https://github.com/aarambh-darshan/typewriter/issues) on GitHub!*

</div>
