# 📊 Type Mapping Reference

Complete reference of how Rust types map to TypeScript and Python.

---

## Primitive Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | Notes |
|---|---|---|---|---|---|---|
| `String` | `string` | `str` | `string` | `String` | `String` | |
| `&str` | `string` | `str` | `string` | `String` | `String` | Treated same as `String` |
| `bool` | `boolean` | `bool` | `bool` | `Bool` | `Boolean` | |
| `u8` | `number` | `int` | `uint8` | `UInt8` | `UByte` | |
| `u16` | `number` | `int` | `uint16` | `UInt16` | `UShort` | |
| `u32` | `number` | `int` | `uint32` | `UInt32` | `UInt` | |
| `u64` | `bigint` | `int` | `uint64` | `UInt64` | `ULong` | TS uses `bigint` for 64-bit+ integers |
| `u128` | `bigint` | `int` | `string` | `String` | `String` | Mapped to string where 128-bit unsupported |
| `i8` | `number` | `int` | `int8` | `Int8` | `Byte` | |
| `i16` | `number` | `int` | `int16` | `Int16` | `Short` | |
| `i32` | `number` | `int` | `int32` | `Int32` | `Int` | |
| `i64` | `bigint` | `int` | `int64` | `Int64` | `Long` | |
| `i128` | `bigint` | `int` | `string` | `String` | `String` | Mapped to string where 128-bit unsupported |
| `f32` | `number` | `float` | `float32` | `Float` | `Float` | |
| `f64` | `number` | `float` | `float64` | `Double` | `Double` | |

---

## Special Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | Notes |
|---|---|---|---|---|---|---|
| `Uuid` | `string` | `UUID` | `string` | `UUID` | `String` | Python imports `from uuid import UUID`; Swift imports `Foundation` |
| `DateTime<Utc>` | `string` | `datetime` | `time.Time` | `Date` | `kotlinx.datetime.Instant` | ISO 8601 string in TS; language-native otherwise |
| `NaiveDate` | `string` | `date` | `time.Time` | `Date` | `kotlinx.datetime.LocalDate` | |
| `serde_json::Value` | `unknown` | `Any` | `interface{}` | `AnyCodable` | `JsonElement` | |

---

## Collection Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | Notes |
|---|---|---|---|---|---|---|
| `Vec<T>` | `T[]` | `list[T]` | `[]T` | `[T]` | `List<T>` | |
| `HashMap<K, V>` | `Record<K, V>` | `dict[K, V]` | `map[K]V` | `[K: V]` | `Map<K, V>` | Also works with `BTreeMap` |
| `(A, B)` | `[A, B]` | `tuple[A, B]` | `struct { Field0 A; Field1 B }` | `(A, B)` | `Pair<A, B>` | Tuples are translated element-wise. |
| `(A, B, C)` | `[A, B, C]` | `tuple[A, B, C]` | `struct { Field0 A; ... }` | `(A, B, C)` | `Triple<A, B, C>` | |

---

## Wrapper Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | Notes |
|---|---|---|---|---|---|---|
| `Option<T>` | `T \| undefined` | `Optional[T]` | `*T` | `T?` | `T?` | Field becomes `field?: T` in TS; `= None` deep in Python; `omitempty` in Go JSON tags. |
| `Box<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent — type is treated as `T` |
| `Arc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |
| `Rc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |

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
