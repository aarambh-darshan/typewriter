# Go

Generates Go structs with JSON tags.

## Type Mappings

| Rust Type | Go Type |
|-----------|---------|
| `String` | `string` |
| `bool` | `bool` |
| `u8`, `u16`, `u32` | `uint8`, `uint16`, `uint32` |
| `u64` | `uint64` |
| `i8`, `i16`, `i32`, `i64` | `int8`, `int16`, `int32`, `int64` |
| `f32`, `f64` | `float32`, `float64` |
| `Uuid` | `string` |
| `DateTime<Utc>` | `time.Time` |
| `Option<T>` | `*T` with `omitempty` |
| `Vec<T>` | `[]T` |
| `HashMap<K, V>` | `map[K]V` |

## Example

**Rust:**
```rust
#[derive(TypeWriter)]
#[sync_to(go)]
pub struct User {
    pub id: String,
    pub email: String,
    pub age: Option<u32>,
}
```

**Generated:**
```go
package types

type User struct {
    Id    string  `json:"id"`
    Email string  `json:"email"`
    Age   *uint32 `json:"age,omitempty"`
}
```

## Package Name

Configure the Go package name:

```toml
[go]
package_name = "models"
```

## File Naming

Files use snake_case by default: `UserProfile` → `user_profile.go`

```toml
[go]
file_style = "PascalCase"  # UserProfile.go
```
