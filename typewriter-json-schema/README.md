# typewriter-json-schema

> JSON Schema Draft 2020-12 emitter for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

[![Crates.io](https://img.shields.io/crates/v/typewriter-json-schema.svg)](https://crates.io/crates/typewriter-json-schema)
[![Docs.rs](https://docs.rs/typewriter-json-schema/badge.svg)](https://docs.rs/typewriter-json-schema)

## What It Generates

| Rust | JSON Schema |
|---|---|
| `struct` | `{ "type": "object", "properties": {...} }` |
| Simple `enum` | `{ "type": "string", "enum": [...] }` |
| Tagged `enum` | `{ "oneOf": [ ... sub schemas ] }` |
| `Option<T>` | Omits field from `required` array |
| `Vec<T>` | `array` with `items` |
| `HashMap<K,V>` | `object` with `additionalProperties` |
| `Uuid` | `string`, `format: "uuid"` |
| `DateTime<Utc>` | `string`, `format: "date-time"` |

## Usage

Used internally by `typewriter-macros`. Most users should depend on the main [`typebridge`](https://crates.io/crates/typebridge) crate.

## License

Apache-2.0 — [Aarambh Dev Hub](https://github.com/aarambh-darshan)
