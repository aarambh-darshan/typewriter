# Kotlin

Generates Kotlin data classes with kotlinx.serialization.

## Type Mappings

| Rust Type | Kotlin Type |
|-----------|-------------|
| `String` | `String` |
| `bool` | `Boolean` |
| `u8`, `u16`, `u32`, `u64` | `UByte`, `UShort`, `UInt`, `ULong` |
| `i8`, `i16`, `i32`, `i64` | `Byte`, `Short`, `Int`, `Long` |
| `f32`, `f64` | `Float`, `Double` |
| `Uuid` | `String` |
| `DateTime<Utc>` | `kotlinx.datetime.Instant` |
| `Option<T>` | `T? = null` |
| `Vec<T>` | `List<T>` |
| `HashMap<K, V>` | `Map<K, V>` |

## Example

**Rust:**
```rust
#[derive(TypeWriter)]
#[sync_to(kotlin)]
pub struct User {
    pub id: String,
    pub email: String,
    pub age: Option<u32>,
}
```

**Generated:**
```kotlin
package types

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class User(
    @SerialName("id") val id: String,
    @SerialName("email") val email: String,
    @SerialName("age") val age: Int? = null
)
```

## Package Name

Configure the Kotlin package name:

```toml
[kotlin]
package_name = "com.example.models"
```

## File Naming

Files use PascalCase by default: `UserProfile` → `UserProfile.kt`

```toml
[kotlin]
file_style = "snake_case"  # user_profile.kt
```
