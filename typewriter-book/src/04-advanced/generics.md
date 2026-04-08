# Generics

typewriter supports generic type parameters.

## Basic Generics

```rust
#[derive(TypeWriter)]
#[sync_to(typescript, python)]
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}
```

**TypeScript:**
```typescript
export interface Pagination<T> {
  items: T[];
  total: bigint;
  page: number;
  page_size: number;
}
```

**Python:**
```python
from typing import Generic, TypeVar, Optional

T = TypeVar("T")

class Pagination(BaseModel, Generic[T]):
    items: list[T]
    total: int
    page: int
    page_size: int
```

## Multiple Type Parameters

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct Map<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
}
```

## Nested Generics

Generics can contain other generic types:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct Nested {
    pub users: Vec<Pagination<User>>,
    pub configs: HashMap<String, Vec<Config>>,
}
```

## Constraints

Generic constraints (e.g., `where T: Clone`) are not currently preserved in output.
