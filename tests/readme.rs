//! The README's examples, compiled. Nothing here runs against a network or a
//! file: each example is a function the test only type-checks, so a README
//! that drifts from the API fails `cargo test`.

#![allow(dead_code)]

/// README "Quick Start".
fn quick_start() -> Result<(), Box<dyn std::error::Error>> {
    use faf::{binary, parse, score};

    let yaml = std::fs::read_to_string("project.faf")?;

    // Parse
    let doc = parse(&yaml)?;

    // Score with the always-33 kernel (the same score as faf-cli)
    let result = score(&yaml)?;
    println!(
        "{} — {}% ({}), {}/{} slots",
        doc.project_name(),
        result.score,
        result.tier,
        result.populated,
        result.active
    );

    // Compile to FAFb binary
    let fafb = binary::compile(&yaml, &binary::CompileOptions::default())?;
    println!("{} bytes YAML → {} bytes FAFb", yaml.len(), fafb.len());
    Ok(())
}

/// README "Radio Protocol".
#[cfg(feature = "radio")]
async fn radio() -> Result<(), Box<dyn std::error::Error>> {
    use faf::faf_radio_rust::{RadioClient, RadioConfig};

    let mut client = RadioClient::new(RadioConfig::grok());
    client.connect().await?;
    client.tune(vec!["91.0".into()]).await?;

    // Broadcast metadata on a frequency
    client
        .broadcast(
            "91.0",
            serde_json::json!({
                "type": "fafb",
                "project": "my-app",
                "size": 220,
            }),
        )
        .await?;
    Ok(())
}

/// README "Axum Middleware".
#[cfg(feature = "axum")]
mod axum_example {
    use axum::{Router, routing::get};
    use faf::axum::{FafContext, FafLayer};

    pub fn app() -> Router {
        Router::new()
            .route("/", get(handler))
            .layer(FafLayer::new())
    }

    async fn handler(faf: FafContext) -> String {
        format!("Project: {}", faf.project_name())
    }
}

/// The Quick Start scores this repo's own project.faf: it runs, not only compiles.
#[test]
fn quick_start_runs_on_this_repo() {
    let yaml =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/project.faf")).unwrap();
    faf::parse(&yaml).unwrap();
    let result = faf::score(&yaml).unwrap();
    assert!(result.score <= 100);
    let fafb = faf::binary::compile(&yaml, &faf::binary::CompileOptions::default()).unwrap();
    assert!(!fafb.is_empty());
}

#[test]
fn readme_examples_compile() {
    // Referencing each example keeps it type-checked under the enabled features.
    let _ = quick_start;
    #[cfg(feature = "radio")]
    let _ = radio;
    #[cfg(feature = "axum")]
    let _ = axum_example::app;
}

/// README "Quick Start": the starter project.faf parses, scores and compiles.
/// Read from the README itself, so the sample there can't drift.
#[test]
fn readme_starter_faf_works() {
    let readme =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md")).unwrap();
    let start = readme.find("```yaml\n").expect("README has a yaml sample") + "```yaml\n".len();
    let end = start + readme[start..].find("```").unwrap();
    let yaml = &readme[start..end];
    let doc = faf::parse(yaml).unwrap();
    assert_eq!(doc.project_name(), "my-app");
    faf::score(yaml).unwrap();
    faf::binary::compile(yaml, &faf::binary::CompileOptions::default()).unwrap();
}
