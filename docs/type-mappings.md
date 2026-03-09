# 📊 Type Mapping Reference

Complete reference of how Rust types map to TypeScript and Python.

---

## Primitive Types

| Rust Type | TypeScript | Python | Notes |
|---|---|---|---|
| `String` | `string` | `str` | |
| `&str` | `string` | `str` | Treated same as `String` |
| `bool` | `boolean` | `bool` | |
| `u8` | `number` | `int` | |
| `u16` | `number` | `int` | |
| `u32` | `number` | `int` | |
| `u64` | `bigint` | `int` | TS uses `bigint` for 64-bit+ integers |
| `u128` | `bigint` | `int` | |
| `i8` | `number` | `int` | |
| `i16` | `number` | `int` | |
| `i32` | `number` | `int` | |
| `i64` | `bigint` | `int` | |
| `i128` | `bigint` | `int` | |
| `f32` | `number` | `float` | |
| `f64` | `number` | `float` | |

---

## Special Types

| Rust Type | TypeScript | Python | Notes |
|---|---|---|---|
| `Uuid` | `string` | `UUID` | Python imports `from uuid import UUID` |
| `DateTime<Utc>` | `string` | `datetime` | ISO 8601 string in TS; Python imports `from datetime import datetime` |
| `NaiveDate` | `string` | `date` | Python imports `from datetime import date` |
| `serde_json::Value` | `unknown` | `Any` | Python imports `from typing import Any` |

---

## Collection Types

| Rust Type | TypeScript | Python | Notes |
|---|---|---|---|
| `Vec<T>` | `T[]` | `list[T]` | |
| `HashMap<K, V>` | `Record<K, V>` | `dict[K, V]` | Also works with `BTreeMap` |
| `(A, B)` | `[A, B]` | `tuple[A, B]` | Tuples of any length |
| `(A, B, C)` | `[A, B, C]` | `tuple[A, B, C]` | |

---

## Wrapper Types

| Rust Type | TypeScript | Python | Notes |
|---|---|---|---|
| `Option<T>` | `T \| undefined` | `Optional[T]` | Field becomes `field?: T` in TS; `= None` default in Python |
| `Box<T>` | (unwrapped to `T`) | (unwrapped to `T`) | Transparent — type is treated as `T` |
| `Arc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |
| `Rc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |

---

## Named Types

Any Rust type that isn't recognized as a primitive, generic, wrapper, or collection is treated as a **named type** — a reference to another struct or enum.

```rust
pub struct Order {
    pub user: UserProfile,   // → UserProfile in TS, UserProfile in Python
    pub items: Vec<OrderItem>, // → OrderItem[] in TS, list[OrderItem] in Python
}
```

The name is kept as-is in all target languages.

---

## The Unit Type

| Rust Type | TypeScript | Python |
|---|---|---|
| `()` | `void` | `None` |

---

## Nested Types

All types can be nested arbitrarily:

| Rust Type | TypeScript | Python |
|---|---|---|
| `Option<Vec<String>>` | `string[] \| undefined` | `Optional[list[str]]` |
| `HashMap<String, Vec<u32>>` | `Record<string, number[]>` | `dict[str, list[int]]` |
| `Vec<Option<String>>` | `(string \| undefined)[]` | `list[Optional[str]]` |
| `Option<HashMap<String, bool>>` | `Record<string, boolean> \| undefined` | `Optional[dict[str, bool]]` |
