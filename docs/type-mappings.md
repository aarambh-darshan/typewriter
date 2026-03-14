# 📊 Type Mapping Reference

Complete reference of how Rust types map to TypeScript and Python.

---

## Primitive Types

| Rust Type | TypeScript | Python | Go | Notes |
|---|---|---|---|---|
| `String` | `string` | `str` | `string` | |
| `&str` | `string` | `str` | `string` | Treated same as `String` |
| `bool` | `boolean` | `bool` | `bool` | |
| `u8` | `number` | `int` | `uint8` | |
| `u16` | `number` | `int` | `uint16` | |
| `u32` | `number` | `int` | `uint32` | |
| `u64` | `bigint` | `int` | `uint64` | TS uses `bigint` for 64-bit+ integers |
| `u128` | `bigint` | `int` | `string` | Go does not have native u128, mapped to string |
| `i8` | `number` | `int` | `int8` | |
| `i16` | `number` | `int` | `int16` | |
| `i32` | `number` | `int` | `int32` | |
| `i64` | `bigint` | `int` | `int64` | |
| `i128` | `bigint` | `int` | `string` | Go does not have native i128, mapped to string |
| `f32` | `number` | `float` | `float32` | |
| `f64` | `number` | `float` | `float64` | |

---

## Special Types

| Rust Type | TypeScript | Python | Go | Notes |
|---|---|---|---|---|
| `Uuid` | `string` | `UUID` | `string` | Python imports `from uuid import UUID` |
| `DateTime<Utc>` | `string` | `datetime` | `time.Time` | ISO 8601 string in TS; Python imports `datetime`; Go imports `time` |
| `NaiveDate` | `string` | `date` | `time.Time` | Python imports `date`; Go imports `time` |
| `serde_json::Value` | `unknown` | `Any` | `interface{}` | Python imports `from typing import Any` |

---

## Collection Types

| Rust Type | TypeScript | Python | Go | Notes |
|---|---|---|---|---|
| `Vec<T>` | `T[]` | `list[T]` | `[]T` | |
| `HashMap<K, V>` | `Record<K, V>` | `dict[K, V]` | `map[K]V` | Also works with `BTreeMap` |
| `(A, B)` | `[A, B]` | `tuple[A, B]` | `struct { Field0 A; Field1 B }` | Tuples are translated to anonymous structs element-wise. |
| `(A, B, C)` | `[A, B, C]` | `tuple[A, B, C]` | `struct { Field0 A; ... }` | |

---

## Wrapper Types

| Rust Type | TypeScript | Python | Go | Notes |
|---|---|---|---|---|
| `Option<T>` | `T \| undefined` | `Optional[T]` | `*T` | Field becomes `field?: T` in TS; `= None` deep in Python; `omitempty` in Go JSON tags. |
| `Box<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent — type is treated as `T` |
| `Arc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |
| `Rc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |

---

## Named Types

Any Rust type that isn't recognized as a primitive, generic, wrapper, or collection is treated as a **named type** — a reference to another struct or enum.

```rust
pub struct Order {
    pub user: UserProfile,   // → UserProfile in TS/Python/Go
    pub items: Vec<OrderItem>, // → OrderItem[] in TS, list[OrderItem] in Python, []OrderItem in Go
}
```

The name is kept as-is in all target languages.

---

## The Unit Type

| Rust Type | TypeScript | Python | Go |
|---|---|---|---|
| `()` | `void` | `None` | `interface{}` / empty struct |

---

## Nested Types

All types can be nested arbitrarily:

| Rust Type | TypeScript | Python | Go |
|---|---|---|---|
| `Option<Vec<String>>` | `string[] \| undefined` | `Optional[list[str]]` | `*[]string` |
| `HashMap<String, Vec<u32>>` | `Record<string, number[]>` | `dict[str, list[int]]` | `map[string][]uint32` |
| `Vec<Option<String>>` | `(string \| undefined)[]` | `list[Optional[str]]` | `[]*string` |
| `Option<HashMap<String, bool>>` | `Record<string, boolean> \| undefined` | `Optional[dict[str, bool]]` | `*map[string]bool` |
