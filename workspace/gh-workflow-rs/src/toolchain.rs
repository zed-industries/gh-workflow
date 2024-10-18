use std::collections::HashMap;

use derive_setters::Setters;

use crate::schema::*;

pub enum Version {
    Stable,
    Beta,
    Nightly,
}
impl Version {
    pub fn to_string(&self) -> String {
        match self {
            Version::Stable => "stable".to_string(),
            Version::Beta => "beta".to_string(),
            Version::Nightly => "nightly".to_string(),
        }
    }
}

#[derive(Setters)]
pub struct RustToolchain {
    version: Version,
    rust_fmt: bool,
    rust_clippy: bool,
    // TODO: add more rust tool chain components
}

impl RustToolchain {
    pub fn to_job(&self) -> Job {
        Job {
            name: Some("Setup Rust Toolchain".to_string()),
            runs_on: vec![Runner::default()],
            steps: vec![
                Step::default().uses("actions/checkout@v2".to_string()),
                Step::default()
                    .uses("actions-rs/toolchain@v1".to_string())
                    .with(HashMap::from([
                        ("toolchain".into(), self.version.to_string()),
                        ("components".into(), "rustfmt".into()),
                        ("override".into(), "true".into()),
                    ])),
                Step::default()
                    .uses("actions-rs/toolchain@v1".to_string())
                    .with(HashMap::from([(
                        "command".to_string(),
                        "check".to_string(),
                    )])),
            ],
            ..Default::default()
        }
    }
}
