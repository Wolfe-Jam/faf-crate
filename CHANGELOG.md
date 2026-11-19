# Changelog

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
