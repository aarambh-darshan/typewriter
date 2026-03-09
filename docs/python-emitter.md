# 🐍 Python Emitter

Detailed guide for the Python Pydantic code generation emitter.

---

## Overview

The Python emitter generates `.py` files containing Pydantic v2 `BaseModel` classes for structs and `Enum` / `Union` types for enums. All generated files include proper import statements.

---

## Struct → Pydantic BaseModel

Every Rust struct becomes a Pydantic `BaseModel` class:

```rust
#[derive(TypeWriter)]
#[sync_to(python)]
pub struct Order {
    pub id: String,
    pub total: f64,
    pub items: Vec<OrderItem>,
    pub discount: Option<f64>,
    pub metadata: HashMap<String, String>,
}
```

```python
from __future__ import annotations
from pydantic import BaseModel
from typing import Optional

class Order(BaseModel):
    id: str
    total: float
    items: list[OrderItem]
    discount: Optional[float] = None
    metadata: dict[str, str]

    class Config:
        populate_by_name = True
```

### Key behaviors:

- `Option<T>` fields become `Optional[T] = None` (with default)
- `Vec<T>` becomes `list[T]`
- `HashMap<K, V>` becomes `dict[K, V]`
- `Uuid` → `UUID` (imports `from uuid import UUID`)
- `DateTime` → `datetime` (imports `from datetime import datetime`)
- Doc comments become Python docstrings
- File naming: **snake_case**
- Every class includes `class Config: populate_by_name = True`

---

## Automatic Import Collection

The emitter automatically collects and deduplicates all needed imports:

```python
from __future__ import annotations
from datetime import datetime
from pydantic import BaseModel
from typing import Optional
from uuid import UUID
```

Only the imports actually needed by the struct's fields are included.

---

## Simple Enum → Python Enum

Enums with only unit variants become Python `Enum` classes:

```rust
#[derive(TypeWriter)]
#[sync_to(python)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}
```

```python
from enum import Enum

class Priority(str, Enum):
    LOW = "Low"
    MEDIUM = "Medium"
    HIGH = "High"
    CRITICAL = "Critical"
```

### Naming: Variant names become `UPPER_SNAKE_CASE` constants.

---

## Tagged Enum → Union with Literal Discriminators

Enums with data and `#[serde(tag = "...")]` become unions of BaseModel subclasses:

```rust
#[derive(TypeWriter)]
#[serde(tag = "event_type")]
#[sync_to(python)]
pub enum WebhookEvent {
    UserCreated { user_id: String, email: String },
    OrderPlaced { order_id: String, total: f64 },
    PaymentFailed { reason: String },
}
```

```python
from __future__ import annotations
from pydantic import BaseModel
from typing import Literal, Union

class UserCreated(BaseModel):
    event_type: Literal["UserCreated"] = "UserCreated"
    user_id: str
    email: str

class OrderPlaced(BaseModel):
    event_type: Literal["OrderPlaced"] = "OrderPlaced"
    order_id: str
    total: float

class PaymentFailed(BaseModel):
    event_type: Literal["PaymentFailed"] = "PaymentFailed"
    reason: str

WebhookEvent = Union[UserCreated, OrderPlaced, PaymentFailed]
```

### Usage in Python:

```python
import json

data = json.loads('{"event_type": "UserCreated", "user_id": "123", "email": "a@b.com"}')
event = WebhookEvent.model_validate(data)  # Pydantic v2 discriminated union
```

---

## File Naming

Default: **snake_case**

| Rust Type | Output File |
|---|---|
| `UserProfile` | `user_profile.py` |
| `APIResponse` | `api_response.py` |
| `OrderItem` | `order_item.py` |

---

## Output Directory

Default: `./generated/python`

Configure globally:
```toml
[python]
output_dir = "../api/schemas"
```

---

## Pydantic Version

typewriter generates **Pydantic v2** code by default:

- `BaseModel` with `model_validate()` support
- `class Config: populate_by_name = True` (v2 syntax)
- Modern Python type annotations (`list[T]`, `dict[K, V]`)

Pydantic v1 support may be added in a future release.
