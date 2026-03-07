# CLAUDE.md - faf (meta-crate)

## PROJECT STATE: PRODUCTION
**Version:** 0.3.0
**Test Status:** Meta-crate (no own tests — delegates to sub-crates)
**Deployment:** Published to crates.io (faf@0.3.0)

---

## CORE CONTEXT

### Project Identity
- **Name:** faf
- **Version:** 0.3.0
- **Stack:** Rust 2021
- **What:** Meta-crate unifying FAF Rust ecosystem under `cargo add faf`
- **Repo:** github.com/Wolfe-Jam/faf-crate

### What's Inside
| Feature | Crate | Version |
|---------|-------|---------|
| `sdk` (default) | faf-rust-sdk | 1.3.0 |
| `radio` | faf-radio-rust | 0.2.0 |
| `axum` | faf-rust-sdk/axum | 1.3.0 |

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
cargo publish --allow-dirty        # Publish (no git required)
```

### Version Bump Protocol
1. Update dep version in `Cargo.toml` (e.g., `faf-radio-rust = "0.2"`)
2. Bump own version
3. `cargo check --features radio,axum`
4. `cargo publish --allow-dirty`

### Important Notes
- **No tests in this crate** — tests live in the sub-crates
- **Repo URL in Cargo.toml** must point to `Wolfe-Jam/faf-crate` (not `Wolfe-Jam/faf`)
- `Wolfe-Jam/faf` = the spec repo. `Wolfe-Jam/faf-crate` = this crate.
- Not a git repo historically — use `--allow-dirty` or commit first

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
| faf-rust-sdk | 1.3.0 | 145/145 tests |
| faf-radio-rust | 0.2.0 | 50/50 tests |

---

**STATUS: BI-SYNC ACTIVE — Synchronized with project.faf**

*Last Sync: 2026-03-06*
