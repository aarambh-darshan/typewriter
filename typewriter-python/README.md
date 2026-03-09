# typewriter-python

> Python Pydantic emitter for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

[![Crates.io](https://img.shields.io/crates/v/typewriter-python.svg)](https://crates.io/crates/typewriter-python)
[![Docs.rs](https://docs.rs/typewriter-python/badge.svg)](https://docs.rs/typewriter-python)

## What It Generates

| Rust | Python |
|---|---|
| `struct` | Pydantic `BaseModel` class |
| Simple `enum` | `class Role(str, Enum)` |
| Tagged `enum` | `Union[...]` with `Literal` discriminators |
| `Option<T>` | `Optional[T] = None` |
| `Vec<T>` | `list[T]` |
| `HashMap<K,V>` | `dict[K, V]` |

## Example Output

```python
from pydantic import BaseModel
from typing import Optional

class UserProfile(BaseModel):
    id: str
    email: str
    age: Optional[int] = None
    tags: list[str]
```

## Usage

Used internally by `typewriter-macros`. Most users should depend on the main [`typewriter`](https://crates.io/crates/typewriter) crate.

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
