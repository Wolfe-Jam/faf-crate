<!-- faf:start -->
<!-- faf: faf | Rust | library | Meta-crate for the FAF Rust ecosystem — one install, full stack -->
<!-- faf: claim=project.faf | family=FAF -->

# CLAUDE.md — faf

## What This Is

Meta-crate for the FAF Rust ecosystem — one install, full stack

## Stack

- **Language:** Rust
- **Backend:** Rust 2024 Edition
- **Hosting:** crates.io
- **CI/CD:** GitHub Actions
- **Build:** Cargo
- **Registry:** crates.io

## Context

- **Who:** Rust developers who want to read, score or compile .faf files from one dependency
- **What:** Meta-crate unifying FAF Rust ecosystem under one install
- **Why:** cargo add faf = full ecosystem. No hunting for individual crates.
- **Where:** crates.io, GitHub (Wolfe-Jam/faf-crate)
- **When:** In production on crates.io since March 2026; 0.4.0 moves to faf-rust-sdk 3.1
- **How:** Re-exports via pub use + optional feature-gated deps

---

*STATUS: SYNC ACTIVE — 2026-11-19T19:45:29.216Z*
<!-- faf:end -->

# CLAUDE.md - faf (meta-crate)

## PROJECT STATE: PRODUCTION
**Version:** 0.4.0
**Test Status:** `tests/readme.rs` compiles every README example; the SDK's own tests live in its crate
**Deployment:** crates.io, published by `.github/workflows/publish-crate.yml` (Trusted Publishing) on a GitHub release

---

## CORE CONTEXT

### Project Identity
- **Name:** faf
- **Version:** 0.4.0
- **Stack:** Rust 2024 (rust-version 1.85)
- **What:** Meta-crate unifying FAF Rust ecosystem under `cargo add faf`
- **Repo:** github.com/Wolfe-Jam/faf-crate

### What's Inside
| Feature | Crate | Version |
|---------|-------|---------|
| `sdk` (default) | faf-rust-sdk | 3.1 |
| `radio` | faf-radio-rust | 0.2.0 |
| `axum` | faf-rust-sdk/axum | 3.1 |

### Key Files
- `Cargo.toml` — Feature flags + dep versions (the whole point)
- `src/lib.rs` — Re-exports (`pub use faf_rust_sdk::*` + conditional radio)

---

## DEVELOPMENT

### Commands
```bash
cargo check                        # Default (sdk only)
cargo check --features radio       # With Radio Protocol
cargo check --features radio,axum  # Full stack
cargo test --locked --all-features # README examples compile
```

### Release
Run `/pubcrate faf`. It bumps the version in a release PR (Cargo.toml, Cargo.lock, CHANGELOG, README), dry-runs, then publishes by a GitHub release `v<version>`, which runs the publish workflow (tests first, then `cargo publish --locked` via OIDC). Never `cargo publish` locally, never `--allow-dirty`.

### Important Notes
- **Tests:** `tests/readme.rs` type-checks the README examples per feature; the SDK's behaviour is tested in faf-rust-sdk
- **Repo URL in Cargo.toml** must point to `Wolfe-Jam/faf-crate` (not `Wolfe-Jam/faf`)
- `Wolfe-Jam/faf` = the spec repo. `Wolfe-Jam/faf-crate` = this crate.

---

## ECOSYSTEM POSITION

```
cargo add faf                     → faf-rust-sdk (parse, validate, compile)
cargo add faf --features radio    → + faf-radio-rust (broadcast, tune, listen)
cargo add faf --features axum     → + Axum middleware (FafContext, FafLayer)
cargo add faf --features radio,axum → everything
```

### Sub-Crate Versions (track here)
| Crate | Current | crates.io |
|-------|---------|-----------|
| faf-rust-sdk | 3.1.1 | always-33 kernel + FAFb + Axum |
| faf-radio-rust | 0.2.0 | 50/50 tests |

---

**STATUS: BI-SYNC ACTIVE — Synchronized with project.faf**

*Last Sync: 2026-03-06*
