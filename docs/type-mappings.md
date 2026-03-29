# 📊 Type Mapping Reference

Complete reference of how Rust types map to TypeScript, Python, Go, Swift, Kotlin, GraphQL, and JSON Schema.

---

## Primitive Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | GraphQL | JSON Schema | Notes |
|---|---|---|---|---|---|---|---|---|
| `String` | `string` | `str` | `string` | `String` | `String` | `String` | `{ "type": "string" }` | |
| `&str` | `string` | `str` | `string` | `String` | `String` | `String` | `{ "type": "string" }` | Treated same as `String` |
| `bool` | `boolean` | `bool` | `bool` | `Bool` | `Boolean` | `Boolean` | `{ "type": "boolean" }` | |
| `u8` | `number` | `int` | `uint8` | `UInt8` | `UByte` | `Int` | `{ "type": "integer" }` | |
| `u16` | `number` | `int` | `uint16` | `UInt16` | `UShort` | `Int` | `{ "type": "integer" }` | |
| `u32` | `number` | `int` | `uint32` | `UInt32` | `UInt` | `Int` | `{ "type": "integer" }` | |
| `u64` | `bigint` | `int` | `uint64` | `UInt64` | `ULong` | `String` | `{ "type": "integer" }` | TS uses `bigint`; GQL uses `String` for safe transport |
| `u128` | `bigint` | `int` | `string` | `String` | `String` | `String` | `{ "type": "string" }` | Mapped to string where 128-bit unsupported |
| `i8` | `number` | `int` | `int8` | `Int8` | `Byte` | `Int` | `{ "type": "integer" }` | |
| `i16` | `number` | `int` | `int16` | `Int16` | `Short` | `Int` | `{ "type": "integer" }` | |
| `i32` | `number` | `int` | `int32` | `Int32` | `Int` | `Int` | `{ "type": "integer" }` | |
| `i64` | `bigint` | `int` | `int64` | `Int64` | `Long` | `String` | `{ "type": "integer" }` | |
| `i128` | `bigint` | `int` | `string` | `String` | `String` | `String` | `{ "type": "string" }` | Mapped to string where 128-bit unsupported |
| `f32` | `number` | `float` | `float32` | `Float` | `Float` | `Float` | `{ "type": "number" }` | |
| `f64` | `number` | `float` | `float64` | `Double` | `Double` | `Float` | `{ "type": "number" }` | |

---

## Special Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | GraphQL | JSON Schema | Notes |
|---|---|---|---|---|---|---|---|---|
| `Uuid` | `string` | `UUID` | `string` | `UUID` | `String` | `ID` | `{ "type": "string", "format": "uuid" }` | Python imports `from uuid import UUID`; Swift imports `Foundation` |
| `DateTime<Utc>` | `string` | `datetime` | `time.Time` | `Date` | `kotlinx.datetime.Instant` | `DateTime` | `{ "type": "string", "format": "date-time" }` | ISO 8601 string in TS; custom scalar in GQL |
| `NaiveDate` | `string` | `date` | `time.Time` | `Date` | `kotlinx.datetime.LocalDate` | `DateTime` | `{ "type": "string", "format": "date" }` | |
| `serde_json::Value` | `unknown` | `Any` | `interface{}` | `AnyCodable` | `JsonElement` | `JSON` | `{}` (any) | Custom scalar in GQL; unrestricted in JSON Schema |

---

## Collection Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | GraphQL | JSON Schema | Notes |
|---|---|---|---|---|---|---|---|---|
| `Vec<T>` | `T[]` | `list[T]` | `[]T` | `[T]` | `List<T>` | `[T!]` | `{ "type": "array", "items": T }` | |
| `HashMap<K, V>` | `Record<K, V>` | `dict[K, V]` | `map[K]V` | `[K: V]` | `Map<K, V>` | `JSON` | `{ "type": "object", "additionalProperties": V }` | Also works with `BTreeMap`; GQL maps to custom scalar |
| `(A, B)` | `[A, B]` | `tuple[A, B]` | `struct { Field0 A; Field1 B }` | `(A, B)` | `Pair<A, B>` | `JSON` | `{ "type": "array", "prefixItems": [...] }` | Tuples use Draft 2020-12 `prefixItems` |
| `(A, B, C)` | `[A, B, C]` | `tuple[A, B, C]` | `struct { Field0 A; ... }` | `(A, B, C)` | `Triple<A, B, C>` | `JSON` | `{ "type": "array", "prefixItems": [...] }` | |

---

## Wrapper Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | GraphQL | JSON Schema | Notes |
|---|---|---|---|---|---|---|---|---|
| `Option<T>` | `T \| undefined` | `Optional[T]` | `*T` | `T?` | `T?` | nullable (no `!`) | schema of `T` (not in `required`) | Field becomes `field?: T` in TS; `= None` in Python; `omitempty` in Go JSON tags. |
| `Box<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent — type is treated as `T` |
| `Arc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |
| `Rc<T>` | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | (unwrapped to `T`) | Transparent |

---

## Named Types

Any Rust type that isn't recognized as a primitive, generic, wrapper, or collection is treated as a **named type** — a reference to another struct or enum.

```rust
pub struct Order {
    pub user: UserProfile,   // → UserProfile in TS/Python/Go
    pub items: Vec<OrderItem>, // → OrderItem[] in TS, list[OrderItem] in Python, []OrderItem in Go
}
```

The name is kept as-is in all target languages. In JSON Schema, named types become `{ "$ref": "type_name.schema.json" }` references.

---

## The Unit Type

| Rust Type | TypeScript | Python | Go | GraphQL | JSON Schema |
|---|---|---|---|---|---|
| `()` | `void` | `None` | `interface{}` / empty struct | *not applicable* | `{ "type": "null" }` |

---

## Nested Types

All types can be nested arbitrarily:

| Rust Type | TypeScript | Python | Go | GraphQL | JSON Schema |
|---|---|---|---|---|---|
| `Option<Vec<String>>` | `string[] \| undefined` | `Optional[list[str]]` | `*[]string` | `[String!]` | `{ "type": "array", "items": { "type": "string" } }` |
| `HashMap<String, Vec<u32>>` | `Record<string, number[]>` | `dict[str, list[int]]` | `map[string][]uint32` | `JSON` | `{ "type": "object", "additionalProperties": ... }` |
| `Vec<Option<String>>` | `(string \| undefined)[]` | `list[Optional[str]]` | `[]*string` | `[String!]` | `{ "type": "array", "items": { "type": "string" } }` |
| `Option<HashMap<String, bool>>` | `Record<string, boolean> \| undefined` | `Optional[dict[str, bool]]` | `*map[string]bool` | `JSON` | `{ "type": "object", "additionalProperties": ... }` |
