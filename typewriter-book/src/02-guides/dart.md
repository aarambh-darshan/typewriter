# Dart/Flutter (Plugin)

The Dart emitter generates `json_serializable`-compatible Dart classes for Flutter projects.

> **Note:** This is a plugin emitter. It must be built and installed as a shared library before use.

## Quick Start

```rust
#[derive(TypeWriter)]
#[sync_to(dart)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub age: Option<u32>,
}
```

Output (`user.dart`):

```dart
import 'package:json_annotation/json_annotation.dart';

part 'user.g.dart';

@JsonSerializable()
class User {
  final String id;
  @JsonKey(name: 'user_name')
  final String userName;
  final int? age;

  const User({
    required this.id,
    required this.userName,
    this.age,
  });

  factory User.fromJson(Map<String, dynamic> json) =>
      _$UserFromJson(json);
  Map<String, dynamic> toJson() => _$UserToJson(this);
}
```

## Type Mappings

| Rust | Dart |
|------|------|
| `String` | `String` |
| `bool` | `bool` |
| `u32`, `i64`, etc. | `int` |
| `f64` | `double` |
| `Option<T>` | `T?` |
| `Vec<T>` | `List<T>` |
| `HashMap<K, V>` | `Map<K, V>` |
| `Uuid` | `String` |
| `DateTime` | `DateTime` |

## Enum Mapping

- **Unit enums** → `enum` with `@JsonValue()` annotations
- **Complex enums** → `sealed class` hierarchy (Dart 3.0+)

## Flutter Integration

After generating types, run `build_runner` to generate the `*.g.dart` files:

```bash
dart run build_runner build
```

## Configuration

```toml
[dart]
output_dir = "./generated/dart"
file_style = "snake_case"
```

See the full [Dart Emitter documentation](../../docs/dart-emitter.md) for more details.
