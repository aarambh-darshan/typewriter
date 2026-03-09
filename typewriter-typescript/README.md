# typewriter-typescript

> TypeScript emitter for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

[![Crates.io](https://img.shields.io/crates/v/typewriter-typescript.svg)](https://crates.io/crates/typewriter-typescript)
[![Docs.rs](https://docs.rs/typewriter-typescript/badge.svg)](https://docs.rs/typewriter-typescript)

## What It Generates

| Rust | TypeScript |
|---|---|
| `struct` | `export interface` |
| Simple `enum` | String literal union (`"A" \| "B"`) |
| Tagged `enum` | Discriminated union (`{ type: "A"; ... }`) |
| `Option<T>` | `field?: T \| undefined` |
| `Vec<T>` | `T[]` |
| `HashMap<K,V>` | `Record<K, V>` |

## Example Output

```typescript
export interface UserProfile {
  id: string;
  email: string;
  age?: number | undefined;
  tags: string[];
}
```

## Usage

Used internally by `typewriter-macros`. Most users should depend on the main [`typewriter`](https://crates.io/crates/typewriter) crate.

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
