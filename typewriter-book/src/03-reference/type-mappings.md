# Type Mappings

Complete reference for Rust → target language type mappings.

## Primitive Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | GraphQL | JSON Schema |
|-----------|------------|--------|-----|-------|--------|---------|-------------|
| `String` | `string` | `str` | `string` | `String` | `String` | `String` | `string` |
| `bool` | `boolean` | `bool` | `bool` | `Bool` | `Boolean` | `Boolean` | `boolean` |
| `u8` | `number` | `int` | `uint8` | `UInt8` | `UByte` | `Int` | `integer` |
| `u16` | `number` | `int` | `uint16` | `UInt16` | `UShort` | `Int` | `integer` |
| `u32` | `number` | `int` | `uint32` | `UInt32` | `UInt` | `Int` | `integer` |
| `u64` | `bigint` | `int` | `uint64` | `UInt64` | `ULong` | `String` | `integer` |
| `i8` | `number` | `int` | `int8` | `Int8` | `Byte` | `Int` | `integer` |
| `i16` | `number` | `int` | `int16` | `Int16` | `Short` | `Int` | `integer` |
| `i32` | `number` | `int` | `int32` | `Int32` | `Int` | `Int` | `integer` |
| `i64` | `bigint` | `int` | `int64` | `Int64` | `Long` | `String` | `integer` |
| `f32` | `number` | `float` | `float32` | `Float` | `Float` | `Float` | `number` |
| `f64` | `number` | `float` | `float64` | `Double` | `Double` | `Float` | `number` |

## Special Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | GraphQL | JSON Schema |
|-----------|------------|--------|-----|-------|--------|---------|-------------|
| `Uuid` | `string` | `UUID` | `string` | `UUID` | `String` | `ID` | `string` (uuid) |
| `DateTime<Utc>` | `string` | `datetime` | `time.Time` | `Date` | `Instant` | `DateTime` | `string` (date-time) |
| `NaiveDate` | `string` | `date` | N/A | N/A | N/A | N/A | `string` (date) |
| `serde_json::Value` | `unknown` | `Any` | `interface{}` | `Any` | `JsonElement` | `JSON` | `{}` |

## Container Types

| Rust Type | TypeScript | Python | Go | Swift | Kotlin | GraphQL | JSON Schema |
|-----------|------------|--------|-----|-------|--------|---------|-------------|
| `Option<T>` | `T \| undefined` | `Optional[T] = None` | `*T` (omitempty) | `T?` | `T? = null` | Nullable | Not in `required` |
| `Vec<T>` | `T[]` | `list[T]` | `[]T` | `[T]` | `List<T>` | `[T!]` | `array` |
| `HashMap<K,V>` | `Record<K, V>` | `dict[K, V]` | `map[K]V` | `[K: V]` | `Map<K, V>` | `JSON` | `object` |
| `(A, B, ...)` | N/A | `Tuple[A, B, ...]` | N/A | N/A | N/A | N/A | `prefixItems` |

## Custom Types

Custom structs and enums are referenced by name in all languages.
