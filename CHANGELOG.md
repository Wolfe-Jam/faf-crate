# Changelog

## [0.3.1] - 2026-11-19

**0.3.1: the README examples compile, and the release is checked before it publishes.**

### Fixed
- **README, Axum example:** it imported `faf::{FafLayer, FafContext}` and called `FafLayer::discover()`; neither exists. It is now `use faf::axum::{FafContext, FafLayer}` with `FafLayer::new()`.
- **README:** the Ecosystem table's version column was stale (it listed `rust-faf-mcp` 0.1.0). The table now lists the crates, and one line says what `faf` re-exports: `faf-rust-sdk` 1.3 and, with `radio`, `faf-radio-rust` 0.2. The Zig link pointed at a private repository and is removed.
- **docs.rs:** two broken links in the crate docs; docs now build with all features, so `radio` and `axum` are documented.
- **Author contact:** `team@faf.one`.

### Added
- `tests/readme.rs` compiles every README example under its feature, so a README that drifts from the API fails `cargo test`.
- CI builds on Rust 1.85, the `rust-version` this crate declares.

### Changed
- **Publishing:** the release workflow tests the commit before publishing, publishes with `--locked`, and a manual run must start from a `v<version>` tag.
- **Since 0.3.0, not published until now:** edition 2024, `rust-version = "1.85"`, Trusted Publishing (OIDC), CI on all features, RustSec audit on pull requests, README tagline.
- The package leaves out `.github/` and `CLAUDE.md`.

## [0.3.0] - 2026-03-07
## [0.2.0] - 2026-03-06
## [0.1.0] - 2026-03-06

Published before this changelog started. See the [crates.io versions page](https://crates.io/crates/faf/versions).
