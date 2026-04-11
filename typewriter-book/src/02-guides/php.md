# PHP (Plugin)

The PHP emitter generates PHP 8.1+ `readonly` classes with constructor-promoted properties.

> **Note:** This is a plugin emitter. It must be built and installed as a shared library before use.

## Quick Start

```rust
#[derive(TypeWriter)]
#[sync_to(php)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: Option<u32>,
}
```

Output (`User.php`):

```php
<?php
declare(strict_types=1);

readonly class User
{
    public function __construct(
        public string $id,
        public string $name,
        public ?int $age = null,
    ) {}
}
```

## Type Mappings

| Rust | PHP |
|------|-----|
| `String` | `string` |
| `bool` | `bool` |
| `u32`, `i64`, etc. | `int` |
| `f64` | `float` |
| `Option<T>` | `?T` |
| `Vec<T>` | `array` |
| `HashMap<K, V>` | `array` |
| `Uuid` | `string` |
| `DateTime` | `\DateTimeInterface` |

## Enum Mapping

- **Unit enums** → `enum Role: string` (PHP 8.1 backed enum)
- **Complex enums** → `interface` + `readonly class` implementations

## Configuration

```toml
[php]
output_dir = "./generated/php"
file_style = "PascalCase"
```

See the full [PHP Emitter documentation](../../docs/php-emitter.md) for more details.
