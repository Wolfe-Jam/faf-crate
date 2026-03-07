# faf

**FAF (Foundational AI-context Format)** — Project DNA for any AI.

IANA-registered as `application/vnd.faf+yaml`. One crate, full ecosystem.

## Install

```toml
[dependencies]
faf = "0.1"
```

With Radio Protocol:

```toml
[dependencies]
faf = { version = "0.1", features = ["radio"] }
```

## What's Inside

| Feature | Crate | What |
|---------|-------|------|
| `sdk` (default) | [faf-rust-sdk](https://crates.io/crates/faf-rust-sdk) | Parse, validate, compile .faf files |
| `radio` | [faf-radio-rust](https://crates.io/crates/faf-radio-rust) | Radio Protocol client for MCPaaS |

## Also

| Crate | What |
|-------|------|
| [rust-faf-mcp](https://crates.io/crates/rust-faf-mcp) | MCP server for .faf |

## Links

- **Website:** [faf.one](https://faf.one)
- **Platform:** [mcpaas.live](https://mcpaas.live)
- **IANA:** [application/vnd.faf+yaml](https://www.iana.org/assignments/media-types/application/vnd.faf+yaml)
- **Contact:** team@faf.one

## License

MIT

---

Built by [Wolfe James](https://github.com/Wolfe-Jam)
