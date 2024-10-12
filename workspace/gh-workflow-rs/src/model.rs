use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Workflow {
    name: String,
    on: On,
    jobs: Jobs,
}

#[derive(Serialize, Debug, Clone)]
pub struct On {
    push: Branches,
    pull_request: Branches,
}

#[derive(Serialize, Debug, Clone)]
pub struct Branches {
    branches: Vec<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Jobs {
    build: Job,
}

#[derive(Serialize, Debug, Clone)]
pub struct Job {
    #[serde(rename = "runs-on")]
    runs_on: String,
    steps: Vec<Step>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Step {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run: Option<String>,
}
