//! WJTTC suite for `faf`: tests what this crate owns, not the SDK again.
//!
//! - **brake**: the claims that must never break. `faf::score` gives faf-cli's
//!   numbers, and every version in the repo agrees.
//! - **engine**: every path the README and the crate docs name resolves under
//!   its feature.
//! - **pit**: the package and the promises around it (MSRV, docs pin).
//!
//! The expected scores come from faf-cli 8.10.0 (`faf score <file> --json`),
//! the reference scorer. If faf-cli and the kernel ever disagree, these fail.

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn read(path: &str) -> String {
    std::fs::read_to_string(format!("{ROOT}/{path}")).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The first line in `text` that starts with `prefix`, with the prefix removed.
fn after<'a>(text: &'a str, prefix: &str) -> &'a str {
    text.lines()
        .find_map(|l| l.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("no line starting with {prefix:?}"))
        .trim()
}

mod brake {
    use super::*;

    /// (fixture, score, tier, populated, active), from faf-cli 8.10.0.
    const PARITY: &[(&str, u32, &str, u32, u32)] = &[
        ("name-only.faf", 3, "RED", 1, 33),
        ("starter.faf", 9, "RED", 3, 33),
        ("half-web.faf", 30, "RED", 10, 33),
        ("library-trophy.faf", 100, "TROPHY", 13, 13),
    ];

    fn check(fixture: &str) {
        let (_, score, tier, populated, active) = *PARITY.iter().find(|p| p.0 == fixture).unwrap();
        let yaml = read(&format!("tests/fixtures/{fixture}"));
        let r = faf::score(&yaml).unwrap();
        assert_eq!(
            (r.score, r.tier.as_str(), r.populated, r.active, r.total),
            (score, tier, populated, active, 33),
            "{fixture}: faf::score disagrees with faf-cli"
        );
    }

    #[test]
    fn score_parity_name_only() {
        check("name-only.faf");
    }

    #[test]
    fn score_parity_readme_starter() {
        check("starter.faf");
    }

    #[test]
    fn score_parity_half_filled_web_app() {
        check("half-web.faf");
    }

    #[test]
    fn score_parity_library_trophy() {
        check("library-trophy.faf");
    }

    /// The README's starter file and the fixture are the same file, so the
    /// parity number covers what a new user pastes.
    #[test]
    fn readme_starter_is_the_fixture() {
        let readme = read("README.md");
        let start = readme.find("```yaml\n").unwrap() + "```yaml\n".len();
        let end = start + readme[start..].find("```").unwrap();
        assert_eq!(&readme[start..end], read("tests/fixtures/starter.faf"));
    }

    /// Cargo.toml, CHANGELOG, project.faf, agent.fafa and the README agree.
    #[test]
    fn versions_in_lockstep() {
        let v = env!("CARGO_PKG_VERSION");
        let changelog = read("CHANGELOG.md");
        let top = after(&changelog, "## [").split(']').next().unwrap();
        assert_eq!(top, v, "CHANGELOG's newest entry");
        let faf = read("project.faf");
        assert_eq!(after(&faf, "  version: "), v, "project.faf");
        let card = read("agent.fafa");
        assert_eq!(after(&card, "  version: "), v, "agent.fafa");
        let readme = read("README.md");
        assert!(
            readme.contains(&format!("`faf` {v},")),
            "README's re-export line names {v}"
        );
    }
}

mod engine {
    #[test]
    fn sdk_paths_resolve() {
        let _parse = faf::parse;
        let _score = faf::score;
        let _compile = faf::binary::compile;
        let _ = faf::binary::CompileOptions::default();
    }

    #[test]
    fn parse_score_compile_round() {
        let yaml = super::read("tests/fixtures/library-trophy.faf");
        let doc = faf::parse(&yaml).unwrap();
        assert_eq!(doc.project_name(), "faf");
        let bytes = faf::binary::compile(&yaml, &faf::binary::CompileOptions::default()).unwrap();
        assert!(!bytes.is_empty());
    }

    #[cfg(feature = "axum")]
    #[test]
    fn axum_paths_resolve() {
        let _ = faf::axum::FafLayer::new();
        let _ = std::any::type_name::<faf::axum::FafContext>();
    }

    #[cfg(feature = "radio")]
    #[test]
    fn radio_paths_resolve() {
        let _ = faf::faf_radio_rust::RadioClient::new(faf::faf_radio_rust::RadioConfig::grok());
    }
}

mod pit {
    use super::*;

    /// rust-version in Cargo.toml is what the README states and what CI builds.
    #[test]
    fn msrv_agrees() {
        let cargo = read("Cargo.toml");
        let msrv = after(&cargo, "rust-version = ").trim_matches('"');
        assert!(read("README.md").contains(&format!("Needs Rust {msrv} or newer")));
        assert!(
            read(".github/workflows/ci.yml").contains(&format!("dtolnay/rust-toolchain@{msrv}"))
        );
    }

    /// The crate docs' `faf = "x.y"` pin matches this version's major.minor.
    #[test]
    fn docs_pin_matches_version() {
        let v = env!("CARGO_PKG_VERSION");
        let minor = v.rsplit_once('.').unwrap().0;
        assert!(read("src/lib.rs").contains(&format!("faf = \"{minor}\"")));
    }
}
