# typewriter-go

> Go (Golang) emitter for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

[![Crates.io](https://img.shields.io/crates/v/typewriter-go.svg)](https://crates.io/crates/typewriter-go)
[![Docs.rs](https://docs.rs/typewriter-go/badge.svg)](https://docs.rs/typewriter-go)

## What It Generates

| Rust | Go |
|---|---|
| `struct` | `type Name struct` |
| Simple `enum` | Custom `string` type with `const` block |
| Tagged `enum` | Internal `interface` with structurally typed variants |
| `Option<T>` | Pointer `*T` with `omitempty` JSON tag |
| `Vec<T>` | Slice `[]T` |
| `HashMap<K,V>` | Map `map[K]V` |

## Example Output

```go
package types

type UserProfile struct {
	Id    string  `json:"id"`
	Email string  `json:"email"`
	Age   *uint32 `json:"age,omitempty"`
	Tags  []string `json:"tags"`
}
```

## Usage

Used internally by `typewriter-macros`. Most users should depend on the main [`typewriter`](https://crates.io/crates/typewriter) crate.

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
