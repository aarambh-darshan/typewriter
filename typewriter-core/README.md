# typewriter-core

> Core IR types, traits, and configuration for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

[![Crates.io](https://img.shields.io/crates/v/typewriter-core.svg)](https://crates.io/crates/typewriter-core)
[![Docs.rs](https://docs.rs/typewriter-core/badge.svg)](https://docs.rs/typewriter-core)

## What's Inside

- **IR Types** (`TypeDef`, `StructDef`, `EnumDef`, `FieldDef`, `TypeKind`) — the language-agnostic intermediate representation
- **`TypeMapper` trait** — the contract every language emitter implements
- **Config** — `typewriter.toml` parsing with defaults

## Usage

This crate is used internally by typewriter's emitters. Most users should depend on the main [`typewriter`](https://crates.io/crates/typewriter) crate instead.

```rust
use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;
```

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
