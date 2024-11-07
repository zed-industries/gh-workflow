use derive_setters::Setters;
use serde::Serialize;

use crate::SetEvent;

#[derive(Serialize, Setters, Clone)]
#[serde(rename_all = "snake_case")]
#[setters(strip_option)]
pub struct Event {
    pub push: Option<Push>,
    pub pull_request: Option<PullRequest>,
    // TODO: add all more events
}

impl Default for Event {
    fn default() -> Self {
        Event { push: Some(Push::default()), pull_request: None }
    }
}

impl SetEvent for Event {
    fn apply(self, mut workflow: crate::Workflow) -> crate::Workflow {
        workflow.on = serde_json::to_value(self).ok();
        workflow
    }
}

#[derive(Default, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Push {
    branches: Vec<String>,
}

impl Push {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        self.branches.push(branch.to_string());
        self
    }
}

#[derive(Default, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct PullRequest {
    types: Vec<String>,
    branches: Vec<String>,
}

impl PullRequest {
    pub fn branch<S: ToString>(mut self, branch: S) -> Self {
        self.branches.push(branch.to_string());
        self
    }

    pub fn open(mut self) -> Self {
        self.types.push("opened".to_string());
        self
    }

    pub fn synchronize(mut self) -> Self {
        self.types.push("synchronize".to_string());
        self
    }

    pub fn reopen(mut self) -> Self {
        self.types.push("reopened".to_string());
        self
    }
}
