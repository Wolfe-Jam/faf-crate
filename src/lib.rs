//! # FAF — Foundational AI-context Format
//!
//! Project DNA for any AI. IANA-registered as `application/vnd.faf+yaml`.
//!
//! This is the meta-crate for the FAF Rust ecosystem. It re-exports
//! the core SDK and optionally the Radio Protocol client.
//!
//! ## Quick Start
//!
//! ```toml
//! [dependencies]
//! faf = "0.3"
//! ```
//!
//! ```rust,no_run
//! let yaml = std::fs::read_to_string("project.faf").unwrap();
//! let file = faf::parse(&yaml).unwrap();
//! println!("{}: {:?}", file.project_name(), file.score());
//! ```
//!
//! ## Features
//!
//! - **`sdk`** (default) — Parse, validate, and compile .faf files via [`faf-rust-sdk`]
//! - **`axum`** — Axum middleware: inject project context into every request
//! - **`radio`** — Radio Protocol client for MCPaaS via [`faf-radio-rust`]
//!
//! ## Ecosystem
//!
//! | Crate | What |
//! |-------|------|
//! | [`faf`](https://crates.io/crates/faf) | This meta-crate |
//! | [`faf-rust-sdk`](https://crates.io/crates/faf-rust-sdk) | Core SDK |
//! | [`faf-radio-rust`](https://crates.io/crates/faf-radio-rust) | Radio Protocol client |
//! | [`rust-faf-mcp`](https://crates.io/crates/rust-faf-mcp) | MCP server |

// Re-export the core SDK
pub use faf_rust_sdk::*;

// Re-export the radio client when the feature is enabled
#[cfg(feature = "radio")]
pub use faf_radio_rust;
