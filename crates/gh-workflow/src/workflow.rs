//!
//! The serde representation of Github Actions Workflow.

use std::fmt::Display;

use derive_setters::Setters;
use indexmap::IndexMap;
use merge::Merge;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::concurrency::Concurrency;
use crate::error::Result;
use crate::generate::Generate;
use crate::job::Job;
use crate::permissions::Permissions;
use crate::{Event};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Jobs(pub(crate) IndexMap<String, Job>);
impl Jobs {
    pub fn add(mut self, key: String, value: Job) -> Self {
        self.0.insert(key, value);
        self
    }

    /// Gets a reference to a job by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the job to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(&Job)` if the job exists, `None` otherwise.
    pub fn get(&self, key: &str) -> Option<&Job> {
        self.0.get(key)
    }
}

/// Represents the configuration for a GitHub workflow.
///
/// A workflow is a configurable automated process made up of one or more jobs.
/// This struct defines the properties that can be set in a workflow YAML file
/// for GitHub Actions, including the name, environment variables, permissions,
/// jobs, concurrency settings, and more.
#[derive(Debug, Default, Setters, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Workflow {
    /// The name of the workflow. GitHub displays the names of your workflows
    /// under your repository's "Actions" tab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Environment variables that can be used in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    /// The name for workflow runs generated from the workflow.
    /// GitHub displays the workflow run name in the list of workflow runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,

    /// The event that triggers the workflow. This can include events like
    /// `push`, `pull_request`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<Event>,

    /// Permissions granted to the `GITHUB_TOKEN` for the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,

    /// The jobs that are defined in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Jobs>,

    /// Concurrency settings for the workflow, allowing control over
    /// how jobs are executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,

    /// Default settings for jobs in the workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,

    /// Secrets that can be used in the workflow, such as tokens or passwords.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,

    /// The maximum number of minutes a job can run before it is canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
}

/// Represents an action that can be triggered by an event in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct EventAction {
    /// A list of branches that trigger the action.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches: Vec<String>,

    /// A list of branches that are ignored for the action.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    branches_ignore: Vec<String>,
}

impl Workflow {
    /// Creates a new `Workflow` with the specified name.
    pub fn new<T: ToString>(name: T) -> Self {
        Self { name: Some(name.to_string()), ..Default::default() }
    }

    /// Converts the `Workflow` to a YAML string representation.
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    /// Adds a job to the workflow with the specified ID and job configuration.
    pub fn add_job<T: ToString, J: Into<Job>>(mut self, id: T, job: J) -> Self {
        let key = id.to_string();
        let jobs = self.jobs.take().unwrap_or_default().add(key, job.into());

        self.jobs = Some(jobs);
        self
    }

    /// Parses a YAML string into a `Workflow`.
    pub fn parse(yml: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(yml)?)
    }

    /// Generates the workflow using the `Generate` struct.
    pub fn generate(self) -> Result<()> {
        Generate::new(self).generate()
    }

    /// Adds an event to the workflow.
    pub fn add_event<T: Into<Event>>(mut self, that: T) -> Self {
        if let Some(mut this) = self.on.take() {
            this.merge(that.into());
            self.on = Some(this);
        } else {
            self.on = Some(that.into());
        }
        self
    }

    /// Adds an environment variable to the workflow.
    pub fn add_env<T: Into<Env>>(mut self, new_env: T) -> Self {
        let mut env = self.env.take().unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.env = Some(env);
        self
    }
}

/// Represents the type of activity in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ActivityType {
    Created,
    Edited,
    Deleted,
}

/// Represents environment variables in the workflow.
#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct Env(pub IndexMap<String, Value>);

impl From<IndexMap<String, Value>> for Env {
    /// Converts an `IndexMap` into an `Env`.
    fn from(value: IndexMap<String, Value>) -> Self {
        Env(value)
    }
}

impl Env {
    /// Sets the `GITHUB_TOKEN` environment variable.
    pub fn github() -> Self {
        let mut map = IndexMap::new();
        map.insert(
            "GITHUB_TOKEN".to_string(),
            Value::from("${{ secrets.GITHUB_TOKEN }}"),
        );
        Env(map)
    }

    /// Creates a new `Env` with a specified key-value pair.
    pub fn new<K: ToString, V: Into<Value>>(key: K, value: V) -> Self {
        Env::default().add(key, value)
    }

    /// Adds an environment variable to the `Env`.
    pub fn add<T1: ToString, T2: Into<Value>>(mut self, key: T1, value: T2) -> Self {
        self.0.insert(key.to_string(), value.into());
        self
    }
}

/// Represents environment variables as key-value pairs.
impl<S1: Display, S2: Display> From<(S1, S2)> for Env {
    /// Converts a tuple into an `Env`.
    fn from(value: (S1, S2)) -> Self {
        let mut index_map: IndexMap<String, Value> = IndexMap::new();
        index_map.insert(value.0.to_string(), Value::String(value.1.to_string()));
        Env(index_map)
    }
}

/// Represents the runner environment for jobs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    #[default]
    Linux,
    MacOS,
    Windows,
    Custom(String),
}

/// Represents a container configuration for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Container {
    /// The image to use for the container.
    pub image: String,

    /// Credentials for accessing the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,

    /// Environment variables for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Env>,

    /// Ports to expose from the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,

    /// Volumes to mount in the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,

    /// Additional options for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,

    /// Hostname for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

/// Represents credentials for accessing a container.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Credentials {
    /// The username for authentication.
    pub username: String,

    /// The password for authentication.
    pub password: String,
}

/// Represents a network port.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Port {
    /// A port specified by its number.
    Number(u16),

    /// A port specified by its name.
    Name(String),
}

/// Represents a volume configuration for containers.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Volume {
    /// The source path of the volume.
    pub source: String,

    /// The destination path of the volume.
    pub destination: String,
}

impl Volume {
    /// Creates a new `Volume` from a string representation.
    pub fn new(volume_str: &str) -> Option<Self> {
        let parts: Vec<&str> = volume_str.split(':').collect();
        if parts.len() == 2 {
            Some(Volume {
                source: parts[0].to_string(),
                destination: parts[1].to_string(),
            })
        } else {
            None
        }
    }
}





/// Represents the strategy for running jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Strategy {
    /// The matrix for job execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Value>,

    /// Whether to fail fast on errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_fast: Option<bool>,

    /// The maximum number of jobs to run in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel: Option<u32>,
}

/// Represents an environment for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Environment {
    /// The name of the environment.
    pub name: String,

    /// The URL associated with the environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Represents default settings for jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Defaults {
    /// Default settings for running jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<RunDefaults>,

    /// Default retry settings for jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryDefaults>,

    /// Default concurrency settings for jobs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
}

/// Represents default settings for running commands.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct RunDefaults {
    /// The shell to use for running commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,

    /// The working directory for running commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// Represents default settings for retries.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RetryDefaults {
    /// The maximum number of retry attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

/// Represents an expression used in conditions.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct Expression(pub String);

impl Expression {
    /// Creates a new `Expression` from a string.
    pub fn new<T: ToString>(expr: T) -> Self {
        Self(expr.to_string())
    }
}

/// Represents a secret required for the workflow.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Secret {
    /// Indicates if the secret is required.
    pub required: bool,

    /// A description of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Represents a strategy for retrying jobs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RetryStrategy {
    /// The maximum number of retry attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<u32>,
}

/// Represents artifacts produced by jobs.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Artifacts {
    /// Artifacts to upload after the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload: Option<Vec<Artifact>>,

    /// Artifacts to download before the job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<Vec<Artifact>>,
}

/// Represents an artifact produced by a job.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Artifact {
    /// The name of the artifact.
    pub name: String,

    /// The path to the artifact.
    pub path: String,

    /// The number of days to retain the artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<u32>,
}