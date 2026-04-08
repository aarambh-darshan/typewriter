# Swift

Generates Swift structs with Codable protocol.

## Type Mappings

| Rust Type | Swift Type |
|-----------|------------|
| `String` | `String` |
| `bool` | `Bool` |
| `u8`, `u16`, `u32`, `u64` | `UInt8`, `UInt16`, `UInt32`, `UInt64` |
| `i8`, `i16`, `i32`, `i64` | `Int8`, `Int16`, `Int32`, `Int64` |
| `f32`, `f64` | `Float`, `Double` |
| `Uuid` | `UUID` |
| `DateTime<Utc>` | `Date` |
| `Option<T>` | `T?` |
| `Vec<T>` | `[T]` |
| `HashMap<K, V>` | `[K: V]` |

## Example

**Rust:**
```rust
#[derive(TypeWriter)]
#[sync_to(swift)]
pub struct User {
    pub id: String,
    pub email: String,
    pub age: Option<u32>,
}
```

**Generated:**
```swift
struct User: Codable {
    let id: String
    let email: String
    let age: UInt32?
}
```

## File Naming

Files use PascalCase by default: `UserProfile` → `UserProfile.swift`

```toml
[swift]
file_style = "snake_case"  # user_profile.swift
```
