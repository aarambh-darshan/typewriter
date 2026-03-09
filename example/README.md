# typewriter — Example

> Working examples demonstrating the typewriter SDK.

## Running

```bash
cargo build -p typewriter-example
```

This generates TypeScript and Python types automatically in:
- `generated/typescript/` — TypeScript interfaces and unions
- `generated/python/` — Pydantic BaseModel classes and enums

## Examples Included

| # | Type | Demonstrates |
|---|---|---|
| 1 | `UserProfile` | Simple struct with optional fields and Vec |
| 2 | `ApiResponse` | Struct with various types |
| 3 | `UserRole` | Simple enum → string union / Python Enum |
| 4 | `Notification` | Tagged enum → discriminated union |
| 5 | `UserRecord` | `#[serde(skip)]` + `#[tw(skip)]` field exclusion |

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
