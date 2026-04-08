# Python

Generates Python Pydantic v2 models.

## Type Mappings

| Rust Type | Python Type |
|-----------|-------------|
| `String` | `str` |
| `bool` | `bool` |
| `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64` | `int` |
| `f32`, `f64` | `float` |
| `Uuid` | `UUID` |
| `DateTime<Utc>` | `datetime` |
| `Option<T>` | `Optional[T] = None` |
| `Vec<T>` | `list[T]` |
| `HashMap<K, V>` | `dict[K, V]` |

## Example

**Rust:**
```rust
#[derive(TypeWriter)]
#[sync_to(python)]
pub struct User {
    pub id: String,
    pub email: String,
    pub age: Option<u32>,
}
```

**Generated:**
```python
from pydantic import BaseModel
from typing import Optional

class User(BaseModel):
    id: str
    email: str
    age: Optional[int] = None
```

## Dataclass Mode

Use Python dataclasses instead of Pydantic:

```toml
[python]
use_dataclass = true
pydantic_v2 = false
```

```python
from dataclasses import dataclass
from typing import Optional

@dataclass
class User:
    id: str
    email: str
    age: Optional[int] = None
```

## File Naming

Files use snake_case by default: `UserProfile` → `user_profile.py`

```toml
[python]
file_style = "kebab-case"   # user-profile.py
# or
file_style = "PascalCase"   # UserProfile.py
```
