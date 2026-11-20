# Changelog

## Unreleased

### Fixed
- **WJTTC `pit::msrv_agrees` failed when run from the published crate:** it read `.github/workflows/ci.yml`, which the package leaves out. It now checks only files that ship (Cargo.toml, the README text and badge). Found by `/dev-notes` running `cargo test` inside faf 0.4.1.

### Changed
- **CI:** the msrv job reads `rust-version` from Cargo.toml instead of naming 1.85, and is now called `msrv`. A new job, `tests from the package`, unpacks `cargo package` output and runs `cargo test --all-features` there.

## [0.4.1] - 2026-11-19

**0.4.1: crates.io gets the new front door — badges, Rust 1.85, a starter project.faf, where to report issues — plus a WJTTC suite, a business card and test receipts.**

No code change: `faf` still re-exports faf-rust-sdk 3.1 and, with `radio`, faf-radio-rust 0.2.

### Added
- **WJTTC suite** (`tests/wjttc.rs`, 12 tests): `faf::score` gives faf-cli 8.10.0's numbers on four fixtures (brake); versions agree across Cargo.toml, CHANGELOG, project.faf, agent.fafa and the README (brake); every README and docs path resolves under its feature (engine); MSRV and the docs pin agree (pit).
- `agent.fafa`: the crate's business card (score, parse, FAFb, Axum, Radio), with its badge in the README.
- **TAF test receipts:** every push to `main` runs the all-features tests under cargo-nextest and records a receipt on the `taf-receipts` branch (faf-taf-git `receipt.yml` v2.7.0), red runs included.

### Docs
- **README:** badges (crates.io, docs.rs, CI, MSRV); Rust 1.85 stated under Install; a starter `project.faf` and `npx faf-cli init` before the Quick Start; the Quick Start is a complete `main`; scoring named in the intro and the `sdk` row; Radio frequencies and the `grok()` preset explained; where to report issues (faf-rust, faf-radio-rust, here); one star request, not two.
- **SECURITY.md:** reports go to `team@faf.one`.
- `tests/readme.rs` parses, scores and compiles the README's starter `project.faf`.

## [0.4.0] - 2026-11-19 — The Current Engine Edition

**0.4.0: `cargo add faf` gives you the current engine — faf-rust-sdk 3.1, the always-33 kernel, the same score as faf-cli.**

### Changed (breaking)
- **faf-rust-sdk 1.3 → 3.1.** `faf` re-exports the current SDK: the always-33 kernel (`faf::score`), FAFb (`faf::binary`) and the Axum middleware (`faf::axum`).
- **`binary::compile` takes options:** `binary::compile(&yaml, &binary::CompileOptions::default())`.
- **Score with `faf::score(&yaml)`**: it computes the always-33 score (score, tier, populated/active slots). `parse(..).score()` still exists and only reads the file's own `ai_score` field, so the README no longer uses it.
- `radio` stays: `faf-radio-rust` 0.2, unchanged.

### Fixed
- **README, Quick Start:** now scores with the kernel and passes `CompileOptions`.
- **README, Axum example:** it imported `faf::{FafLayer, FafContext}` and called `FafLayer::discover()`; neither exists. It is now `use faf::axum::{FafContext, FafLayer}` with `FafLayer::new()`.
- **README:** the Ecosystem table's version column was stale (it listed `rust-faf-mcp` 0.1.0). The table now lists the crates, and one line says what `faf` re-exports. The Zig link pointed at a private repository and is removed.
- **docs.rs:** two broken links in the crate docs; docs now build with all features, so `radio` and `axum` are documented.
- **Author contact:** `team@faf.one`.

### Added
- `tests/readme.rs` compiles every README example under its feature, and runs the Quick Start on this repo's project.faf, so a README that drifts from the API fails `cargo test`.
- CI builds on Rust 1.85, the `rust-version` this crate declares.

### Changed
- **Publishing:** the release workflow tests the commit before publishing, publishes with `--locked`, and a manual run must start from a `v<version>` tag.
- **Since 0.3.0, not published until now:** edition 2024, `rust-version = "1.85"`, Trusted Publishing (OIDC), CI on all features, RustSec audit on pull requests, README tagline.
- The package leaves out `.github/` and `CLAUDE.md`.

## [0.3.0] - 2026-03-07
## [0.2.0] - 2026-03-06
## [0.1.0] - 2026-03-06

Published before this changelog started. See the [crates.io versions page](https://crates.io/crates/faf/versions).
