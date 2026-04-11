# Plugin Configuration

Plugins are configured through `typewriter.toml`.

## Plugin Discovery

```toml
[plugins]
# Directory to scan for .so/.dylib/.dll files
dir = "~/.typewriter/plugins/"

# Explicit paths to plugin libraries
paths = [
    "./my-plugins/libtypewriter_plugin_ruby.so",
]
```

## Plugin-Specific Settings

Each plugin can have its own TOML section using its `language_id` as the key:

```toml
[ruby]
output_dir = "./generated/ruby"
file_style = "snake_case"

[php]
output_dir = "./generated/php"
file_style = "PascalCase"

[dart]
output_dir = "./generated/dart"
file_style = "snake_case"
```

### Standard Keys

All plugins support these standard keys:

| Key | Type | Description |
|-----|------|-------------|
| `output_dir` | `String` | Output directory for generated files |
| `file_style` | `String` | File naming style: `snake_case`, `kebab-case`, `PascalCase` |

Additional keys are passed to the plugin via `PluginConfig.extra`.
