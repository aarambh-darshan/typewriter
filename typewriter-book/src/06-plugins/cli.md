# Plugin CLI Commands

Plugin commands are experimental in v1.0.0. They support local plugin inspection and validation only; there is no `plugin add` command or registry.

## `typebridge plugin list`

```bash
typebridge plugin list
typebridge --format json plugin list
```

## `typebridge plugin validate <path>`

```bash
typebridge plugin validate ./target/release/libtypewriter_plugin_ruby.so
```

## `typebridge plugin info <name>`

```bash
typebridge plugin info ruby
typebridge --format json plugin info ruby
```
