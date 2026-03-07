# faf

**One crate, full ecosystem.** Parse, validate, compile, and broadcast `.faf` files in Rust.

FAF (Foundational AI-context Format) is the IANA-registered format for persistent AI project context (`application/vnd.faf+yaml`). This meta-crate re-exports everything you need.

## Install

```bash
cargo add faf
```

With Radio Protocol + Axum middleware:

```bash
cargo add faf --features radio,axum
```

## Quick Start

```rust
use faf::{parse, binary};

let yaml = std::fs::read_to_string("project.faf")?;

// Parse
let doc = parse(&yaml)?;
println!("{} — {}% AI-ready", doc.project_name(), doc.score().unwrap_or(0));

// Compile to FAFb binary
let fafb = binary::compile(&yaml)?;
println!("{} bytes YAML → {} bytes FAFb", yaml.len(), fafb.len());
```

## Feature Flags

| Feature | Default | Crate | What you get |
|---------|---------|-------|--------------|
| `sdk` | yes | [faf-rust-sdk](https://crates.io/crates/faf-rust-sdk) | Parse, validate, compile, FAFb binary format |
| `radio` | no | [faf-radio-rust](https://crates.io/crates/faf-radio-rust) | Radio Protocol — broadcast AI context via WebSocket |
| `axum` | no | faf-rust-sdk/axum | `FafContext` extractor + `FafLayer` middleware |

## Radio Protocol

Broadcast AI context once, every tool receives:

```rust
use faf::faf_radio_rust::{RadioClient, RadioConfig};

let mut client = RadioClient::new(RadioConfig::grok());
client.connect().await?;
client.tune(vec!["91.0".into()]).await?;

// Broadcast metadata on a frequency
client.broadcast("91.0", serde_json::json!({
    "type": "fafb",
    "project": "my-app",
    "size": 220,
})).await?;
```

## Axum Middleware

Inject `.faf` project context into every request:

```rust
use faf::{FafLayer, FafContext};

let app = Router::new()
    .route("/", get(handler))
    .layer(FafLayer::discover());

async fn handler(faf: FafContext) -> String {
    format!("Project: {}", faf.project_name())
}
```

## Ecosystem

| Crate | Version | What |
|-------|---------|------|
| [`faf`](https://crates.io/crates/faf) | 0.3.0 | This meta-crate |
| [`faf-rust-sdk`](https://crates.io/crates/faf-rust-sdk) | 1.3.0 | Core SDK — parse, validate, compile, FAFb |
| [`faf-radio-rust`](https://crates.io/crates/faf-radio-rust) | 0.2.0 | Radio Protocol client |
| [`rust-faf-mcp`](https://crates.io/crates/rust-faf-mcp) | 0.1.0 | MCP server for .faf |

Also available in [TypeScript](https://www.npmjs.com/package/faf-cli), [Python](https://pypi.org/project/faf-python-sdk/), and [Zig](https://github.com/Wolfe-Jam/xai-faf-zig).

## Links

- **Website:** [faf.one](https://faf.one)
- **IANA:** [application/vnd.faf+yaml](https://www.iana.org/assignments/media-types/application/vnd.faf+yaml)
- **Docs:** [docs.rs/faf](https://docs.rs/faf)
- **Platform:** [mcpaas.live](https://mcpaas.live)

## License

MIT

---

Built by [Wolfe James](https://github.com/Wolfe-Jam) | [faf.one](https://faf.one)
