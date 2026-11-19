# faf

[![crates.io](https://img.shields.io/crates/v/faf.svg)](https://crates.io/crates/faf)
[![docs.rs](https://img.shields.io/docsrs/faf)](https://docs.rs/faf)
[![CI](https://github.com/Wolfe-Jam/faf-crate/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Wolfe-Jam/faf-crate/actions/workflows/ci.yml)
[![MSRV 1.85](https://img.shields.io/badge/rustc-1.85+-blue.svg)](https://github.com/Wolfe-Jam/faf-crate/blob/main/Cargo.toml)

**Persistent Project Context for Rust. `cargo add faf` and you're in.**

**FAF defines. AGENTS.md instructs. AI codes.**

One crate, full ecosystem. Parse, validate, score, compile, and broadcast `.faf` files in Rust.

FAF (Foundational AI-context Format) is the IANA-registered format for persistent AI project context (`application/vnd.faf+yaml`). This meta-crate re-exports everything you need.

## Install

```bash
cargo add faf
```

Needs Rust 1.85 or newer (edition 2024). CI builds on 1.85 to keep that true.

With Radio Protocol + Axum middleware:

```bash
cargo add faf --features radio,axum
```

## Quick Start

No `project.faf` yet? Create one with `npx faf-cli init`, or start from this:

```yaml
faf_version: '3.3'
project:
  name: my-app
  goal: What it does, and for whom
  main_language: Rust
```

Then:

```rust
use faf::{binary, parse, score};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = std::fs::read_to_string("project.faf")?;

    // Parse
    let doc = parse(&yaml)?;

    // Score with the always-33 kernel (the same score as faf-cli)
    let result = score(&yaml)?;
    println!("{} — {}% ({}), {}/{} slots", doc.project_name(), result.score, result.tier, result.populated, result.active);

    // Compile to FAFb binary
    let fafb = binary::compile(&yaml, &binary::CompileOptions::default())?;
    println!("{} bytes YAML → {} bytes FAFb", yaml.len(), fafb.len());
    Ok(())
}
```

## Feature Flags

| Feature | Default | Crate | What you get |
|---------|---------|-------|--------------|
| `sdk` | yes | [faf-rust-sdk](https://crates.io/crates/faf-rust-sdk) | Parse, validate, score (always-33 kernel), compile to the FAFb binary format |
| `radio` | no | [faf-radio-rust](https://crates.io/crates/faf-radio-rust) | Radio Protocol — broadcast AI context via WebSocket |
| `axum` | no | faf-rust-sdk/axum | `FafContext` extractor + `FafLayer` middleware |

## Radio Protocol

Broadcast AI context once, every tool receives. A frequency (40.0 to 108.0) is a channel: every client tuned to it gets what is broadcast there. `RadioConfig::grok()` connects to the hosted FAF beacon.

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
use axum::{routing::get, Router};
use faf::axum::{FafContext, FafLayer};

let app = Router::new()
    .route("/", get(handler))
    .layer(FafLayer::new());

async fn handler(faf: FafContext) -> String {
    format!("Project: {}", faf.project_name())
}
```

## Ecosystem

| Crate | What |
|-------|------|
| [`faf`](https://crates.io/crates/faf) | This meta-crate |
| [`faf-rust-sdk`](https://crates.io/crates/faf-rust-sdk) | Core SDK — parse, validate, compile, FAFb |
| [`faf-radio-rust`](https://crates.io/crates/faf-radio-rust) | Radio Protocol client |
| [`rust-faf-mcp`](https://crates.io/crates/rust-faf-mcp) | MCP server for .faf |

`faf` 0.4.0, The Current Engine Edition, re-exports `faf-rust-sdk` 3.1 and, with `radio`, `faf-radio-rust` 0.2.

Also available in [TypeScript](https://www.npmjs.com/package/faf-cli) and [Python](https://pypi.org/project/faf-python-sdk/).

If `faf` has been useful, consider starring the repo — it helps others find it.

## Issues and contributing

`faf` only re-exports other crates, so report a bug where the code lives:

- Parsing, scoring, FAFb or Axum: [faf-rust](https://github.com/Wolfe-Jam/faf-rust/issues) (`faf-rust-sdk`)
- Radio Protocol: [faf-radio-rust](https://github.com/Wolfe-Jam/faf-radio-rust/issues)
- Features, versions, docs or this README: [faf-crate](https://github.com/Wolfe-Jam/faf-crate/issues)

Pull requests are welcome. `cargo test --all-features` checks that every example in this README compiles. Security reports: see [SECURITY.md](SECURITY.md).

## Links

- **Website:** [faf.one](https://faf.one)
- **IANA:** [application/vnd.faf+yaml](https://www.iana.org/assignments/media-types/application/vnd.faf+yaml)
- **Docs:** [docs.rs/faf](https://docs.rs/faf)
- **Platform:** [mcpaas.live](https://mcpaas.live)

## License

MIT

---

Built by [Wolfe James](https://github.com/Wolfe-Jam) | [faf.one](https://faf.one)
