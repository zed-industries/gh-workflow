use std::collections::HashMap;

use derive_setters::Setters;

use crate::workflow::*;

///
/// A type-safe representation of the Rust toolchain.
/// Instead of writing the github action for Rust by hand, we can use this struct to generate the github action.
#[derive(Default)]
pub enum Version {
    #[default]
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

#[derive(Setters, Default)]
pub struct RustToolchain {
    version: Version,
    fmt: bool,
    clippy: bool,
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
                        (
                            "components".into(),
                            vec!["rustfmt", "clippy"].join(", ").into(),
                        ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_to_job() {
        let toolchain = RustToolchain::default();
        let job = toolchain.to_job();
        let workflow = Workflow::default()
            .add_job("build".to_string(), job)
            .unwrap();
        let yml = serde_yaml::to_string(&workflow).unwrap();
        assert_snapshot!(yml);
    }
}
