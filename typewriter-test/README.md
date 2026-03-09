# typewriter-test

> Integration and snapshot tests for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

## Running Tests

```bash
# Run all snapshot tests
cargo test -p typewriter-test

# Accept new/changed snapshots
cargo insta test --accept -p typewriter-test
```

## Test Coverage

| Language | Tests | What's Covered |
|---|---|---|
| TypeScript | 5 snapshots | Simple struct, collections, simple enum, tagged enum, readonly |
| Python | 4 snapshots | Simple struct, collections, simple enum, tagged enum |

Snapshots are stored in `tests/snapshots/` and committed to git.

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
