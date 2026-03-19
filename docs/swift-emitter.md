# 🦅 Swift Emitter

The Swift emitter translates Rust data structures into native Swift `struct`s and `enum`s that conform to the `Codable` protocol.

## File Organization

All generated `.swift` types are output into a single directory configured via `typewriter.toml` using `output_dir`. Typebridge generates one Swift file per Rust type.

## Structs

Rust `struct`s become Swift `struct`s conforming to `Codable`.

```rust
#[derive(TypeWriter)]
#[sync_to(swift)]
pub struct UserProfile {
    pub id: String,
    pub is_active: bool,
    pub age: Option<u32>,
}
```

```swift
/// UserProfile
struct UserProfile: Codable {
    let id: String
    let isActive: Bool
    let age: UInt32?

    enum CodingKeys: String, CodingKey {
        case id
        case isActive = "is_active"
        case age
    }
}
```

**Note:** Typebridge automatically converts Rust's `snake_case` fields to Swift's conventional `camelCase`. It generates `CodingKeys` to ensure JSON serialization matches the original Rust structure perfectly.

## Simple Enums

Enums with only unit variants are generated as Swift `String` backed enums.

```rust
#[derive(TypeWriter)]
#[sync_to(swift)]
pub enum Priority {
    High,
    Medium,
    Low,
}
```

```swift
enum Priority: String, Codable {
    case high = "High"
    case medium = "Medium"
    case low = "Low"
}
```

## Data-Carrying Enums

Typebridge fully supports all Serde enum representations (`External`, `Internal`, `Adjacent`, `Untagged`) for Swift.

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")] // Internal representation
#[sync_to(swift)]
pub enum Event {
    Click { x: i32, y: i32 },
    Hover,
}
```

```swift
enum Event: Codable {
    case click(x: Int32, y: Int32)
    case hover

    enum CodingKeys: String, CodingKey {
        case type = "type"
    }
    
    enum ClickKeys: String, CodingKey {
        case x
        case y
    }
    // ... custom encoding/decoding is handled by the Swift compiler where possible,
    // or through explicit representations if generated.
}
```

## Configuration

Configure the Swift emitter in your `typewriter.toml`:

```toml
[swift]
# Where generated .swift files are written
# Default: "./generated/swift"
output_dir = "../ios/App/Models"

# File naming convention for output files
# Default: "PascalCase"
file_style = "PascalCase"
```
