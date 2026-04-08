# Your First Type

## Basic Struct

Add your first `#[derive(TypeWriter)]` struct:

```rust
use typebridge::TypeWriter;
use serde::{Serialize, Deserialize};

/// A user in the system.
#[derive(Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript, python, go)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub age: Option<u32>,
    pub is_active: bool,
    pub tags: Vec<String>,
}
```

## Building

Run `cargo build` to generate the types:

```bash
cargo build
```

You'll see output like:

```
  typewriter: User → ./generated/typescript/user.ts
  typewriter: User → ./generated/typescript/user.schema.ts
  typewriter: User → ./generated/python/user.py
  typewriter: User → ./generated/go/user.go
```

## Generated Output

### TypeScript

```typescript
export interface User {
  id: string;
  email: string;
  name: string;
  age?: number | undefined;
  is_active: boolean;
  tags: string[];
}
```

### Python

```python
from pydantic import BaseModel
from typing import Optional

class User(BaseModel):
    id: str
    email: str
    name: str
    age: Optional[int] = None
    is_active: bool
    tags: list[str]
```

### Go

```go
package types

type User struct {
    Id        string   `json:"id"`
    Email     string   `json:"email"`
    Name      string   `json:"name"`
    Age       *uint32  `json:"age,omitempty"`
    Is_active bool     `json:"is_active"`
    Tags      []string `json:"tags"`
}
```

## What Each Part Does

| Part | Purpose |
|------|---------|
| `use typebridge::TypeWriter;` | Import the derive macro |
| `#[derive(... TypeWriter)]` | Enable type generation |
| `#[sync_to(typescript, python)]` | Target languages |
| `/// A user...` | Doc comment becomes JSDoc/docstring |

## Next Steps

- [Configuration](configuration.md) — Customize output directories
- [Type Mappings](../03-reference/type-mappings.md) — See all supported types
- [Attributes Reference](../03-reference/attributes.md) — Fine-tune output
