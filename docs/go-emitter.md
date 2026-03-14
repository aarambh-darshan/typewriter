# 🐹 Go Emitter

The Go emitter translates Rust data structures into native Go structs, interfaces, and types.

## File Organization

All generated `.go` types are output into a single directory configured via `typewriter.toml` using `output_dir`.

Because Go treats all files within the same folder with the same `package` keyword as part of the exact same module, **typebridge does not generate or need cross-file imports** for your data structures. As long as the structs are generated into the associated directory, they will be globally valid across that package!

Typebridge generates standard library imports such as `"time"` and `"encoding/json"` dynamically based on the fields used within a specific generated file.

## Structs

Rust `struct`s become Go `struct`s.

```rust
#[derive(TypeWriter)]
#[sync_to(go)]
pub struct UserProfile {
    pub id: String,
    pub is_active: bool,
    pub age: Option<u32>,
}
```

```go
package types

// UserProfile
type UserProfile struct {
	Id        string  `json:"id"`
	Is_active bool    `json:"is_active"`
	Age       *uint32 `json:"age,omitempty"`
}
```

**Note:** Go requires fields to be Capitalized in order to be exported (and thus successfully serialized/deserialized by `encoding/json`). Typebridge will automatically capitalize the first letter of struct fields in Go generation while preserving the JSON tagging exactly as `serde` expects it to be formatted.
Optional fields will emit a pointer type `*` alongside an `omitempty` tag identifier.

## Simple Enums

Enums with only unit variants are generated as Go custom `string` types alongside an associated `const` block.

```rust
#[derive(TypeWriter)]
#[sync_to(go)]
pub enum Priority {
    High,
    Medium,
    Low,
}
```

```go
type Priority string

const (
	PriorityHigh   Priority = "High"
	PriorityMedium Priority = "Medium"
	PriorityLow    Priority = "Low"
)
```

## Data-Carrying Enums (Discriminated Unions)

Go does not support discriminated unions locally natively. As a workaround, typebridge utilizes Go interfaces paired with distinct structs that implement that interface alongside a generated native `UnmarshalJSON` function to bind them correctly.

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")]
#[sync_to(go)]
pub enum Event {
    Click { x: i32, y: i32 },
    Hover,
}
```

```go
type Event interface {
	isEvent()
}

type EventClick struct {
	X int32 `json:"x"`
	Y int32 `json:"y"`
}
func (x *EventClick) isEvent() {}

type EventHover struct {}
func (x *EventHover) isEvent() {}
```

*Note: For complex tagged variants, developers must cast and define specific data structures during json.Unmarshal un-packing based on their overarching `type` payload parameter. Typebridge generates the corresponding types as an interface.*

## Configuration

Configure the Go emitter in your `typewriter.toml`:

```toml
[go]
# Where generated .go files are written
# Default: "./generated/go"
output_dir = "../backend/types"

# File naming convention for output files
# Default: "snake_case"
file_style = "snake_case"

# Package name output inside the rendered files
# Default: "types"
package_name = "api_types"
```
