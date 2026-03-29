# typewriter-macros

> Proc macro crate for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

[![Crates.io](https://img.shields.io/crates/v/typewriter-macros.svg)](https://crates.io/crates/typewriter-macros)
[![Docs.rs](https://docs.rs/typewriter-macros/badge.svg)](https://docs.rs/typewriter-macros)

## What's Inside

- **`#[derive(TypeWriter)]`** — the main proc macro that drives code generation
- **Parser** — converts `syn::DeriveInput` → typewriter IR using syn 2.x
- **Emitter dispatcher** — routes to feature-gated language emitters

## Usage

Most users should depend on the main [`typebridge`](https://crates.io/crates/typebridge) crate, which re-exports this macro.

```rust
use typebridge::TypeWriter;

#[derive(TypeWriter)]
#[sync_to(typescript, python)]
pub struct MyType { /* ... */ }
```

## Features

| Feature | Default | Description |
|---|---|---|
| `typescript` | ✅ | Enable TypeScript emitter |
| `python` | ✅ | Enable Python emitter |

## License

Apache-2.0 — [Aarambh Dev Hub](https://github.com/aarambh-darshan)
