//! The README's examples, compiled. Nothing here runs against a network or a
//! file: each example is a function the test only type-checks, so a README
//! that drifts from the API fails `cargo test`.

#![allow(dead_code)]

/// README "Quick Start".
fn quick_start() -> Result<(), Box<dyn std::error::Error>> {
    use faf::{binary, parse};

    let yaml = std::fs::read_to_string("project.faf")?;

    // Parse
    let doc = parse(&yaml)?;
    println!(
        "{} — {}% AI-ready",
        doc.project_name(),
        doc.score().unwrap_or(0)
    );

    // Compile to FAFb binary
    let fafb = binary::compile(&yaml)?;
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

#[test]
fn readme_examples_compile() {
    // Referencing each example keeps it type-checked under the enabled features.
    let _ = quick_start;
    #[cfg(feature = "radio")]
    let _ = radio;
    #[cfg(feature = "axum")]
    let _ = axum_example::app;
}
