# Plugin CLI Commands

Manage plugins using the `typewriter plugin` subcommand.

## `typewriter plugin list`

List all loaded plugins:

```bash
$ typewriter plugin list
Plugins: 3 plugin(s) loaded:

  ● Ruby (Sorbet) v0.1.0
    Language ID:  ruby
    Extension:    .rbi
    Output dir:   ./generated/ruby

  ● PHP v0.1.0
    Language ID:  php
    Extension:    .php
    Output dir:   ./generated/php
```

## `typewriter plugin validate <path>`

Validate a plugin shared library before installing:

```bash
$ typewriter plugin validate ./target/release/libtypewriter_plugin_ruby.so
✓ Plugin is valid!
  Name:       Ruby (Sorbet)
  Language:   ruby
  Version:    0.1.0
  Extension:  .rbi
```

## `typewriter plugin info <name>`

Show details about a loaded plugin:

```bash
$ typewriter plugin info ruby
Plugin: Ruby (Sorbet)
  Language ID:  ruby
  Version:      0.1.0
  Extension:    .rbi
  Output dir:   ./generated/ruby
  Loaded from:  /home/user/.typewriter/plugins/libtypewriter_plugin_ruby.so
```
