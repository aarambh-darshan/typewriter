# typewriter

**Cross-Language Type Synchronization SDK for Rust**

Define your types once in Rust. Get perfectly matching types in TypeScript, Python, Go, Swift, Kotlin, GraphQL, and JSON Schema — automatically, forever.

## Features

- **Annotate once → generate everywhere**: Add `#[derive(TypeWriter)]` to your Rust structs/enums
- **Zero drift**: Types stay in sync automatically on every build
- **Multiple languages**: TypeScript, Python, Go, Swift, Kotlin, GraphQL, JSON Schema
- **Zod schemas**: Automatic validation schemas for TypeScript
- **CLI tools**: Project-wide generation, drift checking, watch mode

## Quick Example

```rust
use typebridge::TypeWriter;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript, python, go)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub age: Option<u32>,
}
```

This generates:

**TypeScript:**
```typescript
export interface User {
  id: string;
  email: string;
  name: string;
  age?: number | undefined;
}
```

**Python:**
```python
class User(BaseModel):
    id: str
    email: str
    name: str
    age: Optional[int] = None
```

**Go:**
```go
type User struct {
    Id    string  `json:"id"`
    Email string  `json:"email"`
    Name  string  `json:"name"`
    Age   *uint32 `json:"age,omitempty"`
}
```

## Next Steps

- [Installation](01-getting-started/installation.md) — Add typewriter to your project
- [Your First Type](01-getting-started/first-steps.md) — Generate your first types
- [Language Guides](02-guides/typescript.md) — Language-specific details

## Resources

- [crates.io](https://crates.io/crates/typebridge)
- [docs.rs](https://docs.rs/typebridge)
- [GitHub](https://github.com/aarambh-darshan/typewriter)
