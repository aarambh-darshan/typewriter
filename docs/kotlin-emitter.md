# 🚀 Kotlin Emitter

The Kotlin emitter translates Rust data structures into native Kotlin `data class`es and `sealed class`es utilizing `kotlinx.serialization`.

## File Organization

All generated `.kt` types are output into a single directory configured via `typewriter.toml` using `output_dir`. Typebridge generates one Kotlin file per Rust type.

## Structs

Rust `struct`s become Kotlin `data class`es annotated with `@Serializable`.

```rust
#[derive(TypeWriter)]
#[sync_to(kotlin)]
pub struct UserProfile {
    pub id: String,
    pub is_active: bool,
    pub age: Option<u32>,
}
```

```kotlin
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName

@Serializable
data class UserProfile(
    val id: String,
    @SerialName("is_active")
    val isActive: Boolean,
    val age: UInt? = null,
)
```

**Note:** Typebridge automatically converts Rust's `snake_case` fields to Kotlin's conventional `camelCase`. It adds `@SerialName("...")` annotations to ensure JSON serialization matches the original Rust structure perfectly. Optionals default to `null`.

## Simple Enums

Enums with only unit variants are generated as Kotlin `enum class`es.

```rust
#[derive(TypeWriter)]
#[sync_to(kotlin)]
pub enum Priority {
    High,
    Medium,
    Low,
}
```

```kotlin
@Serializable
enum class Priority {
    @SerialName("High")
    High,
    @SerialName("Medium")
    Medium,
    @SerialName("Low")
    Low
}
```

## Data-Carrying Enums

Kotlin represents Rust's advanced enums using `sealed class` hierarchies. Typebridge fully supports all Serde enum representations (`External`, `Internal`, `Adjacent`, `Untagged`).

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")] // Internal representation
#[sync_to(kotlin)]
pub enum Event {
    Click { x: i32, y: i32 },
    Hover,
}
```

```kotlin
@Serializable
@JsonClassDiscriminator("type")
sealed class Event {
    @SerialName("Click")
    @Serializable
    data class Click(
        val x: Int,
        val y: Int,
    ) : Event()

    @SerialName("Hover")
    @Serializable
    object Hover : Event()
}
```

## Configuration

Configure the Kotlin emitter in your `typewriter.toml`:

```toml
[kotlin]
# Where generated .kt files are written
# Default: "./generated/kotlin"
output_dir = "../android/app/src/main/java/models"

# File naming convention for output files
# Default: "PascalCase"
file_style = "PascalCase"
```
