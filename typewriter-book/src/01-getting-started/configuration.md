# Configuration

Create a `typewriter.toml` at your project root to customize output directories, file naming styles, and other options.

## Basic Configuration

```toml
[typescript]
output_dir = "../frontend/src/types"
file_style = "kebab-case"
readonly = false
zod = true

[python]
output_dir = "../api/schemas"
file_style = "snake_case"
pydantic_v2 = true

[go]
output_dir = "../backend/types"
package_name = "api_types"

[graphql]
output_dir = "../schema/types"

[kotlin]
package_name = "com.example.types"
```

## All Configuration Options

### TypeScript

```toml
[typescript]
output_dir = "./generated/typescript"  # Output directory
file_style = "kebab-case"                # kebab-case | snake_case | PascalCase
readonly = false                        # Make all fields readonly
zod = true                              # Generate Zod schema files
```

### Python

```toml
[python]
output_dir = "./generated/python"
file_style = "snake_case"               # snake_case | kebab-case | PascalCase
pydantic_v2 = true                     # Use Pydantic v2
use_dataclass = false                   # Use @dataclass instead of BaseModel
```

### Go

```toml
[go]
output_dir = "./generated/go"
file_style = "snake_case"
package_name = "types"                  # Go package name
```

### Swift

```toml
[swift]
output_dir = "./generated/swift"
file_style = "PascalCase"              # PascalCase | snake_case | kebab-case
```

### Kotlin

```toml
[kotlin]
output_dir = "./generated/kotlin"
file_style = "PascalCase"
package_name = "types"                  # Kotlin package name
```

### GraphQL

```toml
[graphql]
output_dir = "./generated/graphql"
file_style = "snake_case"
```

### JSON Schema

```toml
[json_schema]
output_dir = "./generated/json-schema"
file_style = "snake_case"
```

## File Naming Styles

| Style | Example |
|-------|---------|
| `kebab-case` | `user-profile.ts` |
| `snake_case` | `user_profile.py` |
| `PascalCase` | `UserProfile.swift` |

## Default Values

If not specified:

| Setting | Default |
|---------|---------|
| TypeScript output_dir | `./generated/typescript` |
| Python output_dir | `./generated/python` |
| Go output_dir | `./generated/go` |
| Swift output_dir | `./generated/swift` |
| Kotlin output_dir | `./generated/kotlin` |
| GraphQL output_dir | `./generated/graphql` |
| JSON Schema output_dir | `./generated/json-schema` |
| TypeScript zod | `true` |
| Python pydantic_v2 | `true` |
| Go package_name | `types` |
| Kotlin package_name | `types` |
