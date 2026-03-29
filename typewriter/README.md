# typebridge

> **Cross-Language Type Synchronization SDK for Rust**
> Define your types once in Rust. Get perfectly matching types in TypeScript, Python, Go, Swift, and Kotlin — automatically, forever.

[![Crates.io](https://img.shields.io/crates/v/typebridge.svg)](https://crates.io/crates/typebridge)
[![Docs.rs](https://docs.rs/typebridge/badge.svg)](https://docs.rs/typebridge)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://github.com/aarambh-darshan/typewriter/blob/main/LICENSE)

## Quick Start

```toml
[dependencies]
typebridge = "0.4.2"
serde = { version = "1", features = ["derive"] }
```

```rust
use typebridge::TypeWriter;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript, python)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub age: Option<u32>,
}
// cargo build generates:
// ✅ ./generated/typescript/user-profile.ts
// ✅ ./generated/python/user_profile.py
```

## Features

- **TypeScript** — `export interface`, discriminated unions, optional fields
- **Python** — Pydantic v2 `BaseModel`, `Enum`, `Union` with `Literal` discriminators
- **Serde compatible** — auto-reads `#[serde(rename, skip, tag, flatten)]`
- **Custom attributes** — `#[tw(skip)]`, `#[tw(rename)]`, `#[tw(optional)]`
- **Zero-config** — works out of the box, optional `typewriter.toml` for customization

See the full [documentation](https://github.com/aarambh-darshan/typewriter) for more.

## License

Apache-2.0 — [Aarambh Dev Hub](https://github.com/aarambh-darshan)
