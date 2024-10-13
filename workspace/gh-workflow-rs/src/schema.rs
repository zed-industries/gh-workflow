use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum GithubEvent {
    BranchProtectionRule,
    CheckRun,
    CheckSuite,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Discussion,
    DiscussionComment,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    MergeGroup,
    Milestone,
    PageBuild,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequest,
    PullRequestReview,
    PullRequestReviewComment,
    PullRequestTarget,
    Push,
    RegistryPackage,
    Release,
    Status,
    Watch,
    WorkflowCall,
    WorkflowDispatch,
    WorkflowRun,
    RepositoryDispatch,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    pub name: Option<String>,
    pub on: Vec<EventConfig>,
    pub jobs: HashMap<String, Job>,
    pub concurrency: Option<Concurrency>,
    pub defaults: Option<Defaults>,
    pub secrets: Option<HashMap<String, Secret>>, // Added secrets field
    pub env: Option<HashMap<String, String>>,     // Added workflow-level env
    pub timeout_minutes: Option<u32>,             // Added workflow-level timeout
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct EventConfig {
    pub event: GithubEvent,
    pub types: Option<Vec<String>>,
    pub branches: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub paths: Option<Vec<String>>,
    pub branches_ignore: Option<Vec<String>>, // Added branches-ignore filter
    pub tags_ignore: Option<Vec<String>>,     // Added tags-ignore filter
    pub paths_ignore: Option<Vec<String>>,    // Added paths-ignore filter
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Job {
    pub name: Option<String>,
    pub runs_on: Vec<Runner>,
    pub steps: Vec<Step>,
    pub container: Option<Container>,
    pub needs: Option<Vec<String>>,
    pub permissions: Option<Permissions>,
    pub strategy: Option<Strategy>,
    pub environment: Option<Environment>,
    pub outputs: Option<HashMap<String, String>>,
    pub concurrency: Option<Concurrency>,
    pub timeout_minutes: Option<u32>,
    pub if_condition: Option<Expression>,
    pub services: Option<HashMap<String, Container>>,
    pub secrets: Option<HashMap<String, Secret>>, // Added secrets field for reusable workflows
    pub defaults: Option<Defaults>,               // Added job-level defaults
    pub env: Option<HashMap<String, String>>,     // Added job-level env
    pub continue_on_error: Option<bool>,          // Added continue-on-error field for jobs
    pub retry: Option<RetryStrategy>,             // Added retry strategy for jobs
    pub artifacts: Option<Artifacts>,             // Added artifacts support for jobs
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    pub id: Option<String>,
    pub name: Option<String>,
    pub uses: Option<String>,
    pub run: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub if_condition: Option<Expression>,
    pub timeout_minutes: Option<u32>,
    pub continue_on_error: Option<bool>,
    pub working_directory: Option<String>, // Added working directory for steps
    pub retry: Option<RetryStrategy>,      // Added retry strategy for steps
    pub artifacts: Option<Artifacts>,      // Added artifacts support for steps
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    Linux,
    MacOS,
    Windows,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Container {
    pub image: String,
    pub credentials: Option<Credentials>,
    pub env: Option<HashMap<String, String>>,
    pub ports: Option<Vec<Port>>,
    pub volumes: Option<Vec<Volume>>,
    pub options: Option<String>,
    pub hostname: Option<String>, // Added hostname field specific for services
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Port {    
    Number(u16),
    Name(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Volume {
    pub source: String,
    pub destination: String,
}

impl Volume {
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Concurrency {
    pub group: String,
    pub cancel_in_progress: Option<bool>,
    pub limit: Option<u32>, // Added concurrency limit for finer control
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Permissions {
    pub actions: Option<PermissionLevel>,
    pub contents: Option<PermissionLevel>,
    pub issues: Option<PermissionLevel>,
    pub pull_requests: Option<PermissionLevel>,
    pub deployments: Option<PermissionLevel>,
    pub checks: Option<PermissionLevel>,
    pub statuses: Option<PermissionLevel>,
    pub packages: Option<PermissionLevel>,
    pub pages: Option<PermissionLevel>,
    pub id_token: Option<PermissionLevel>,
    pub event_specific: Option<HashMap<GithubEvent, PermissionLevel>>, // Added event-specific permissions
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionLevel {
    #[default]
    Read,
    Write,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Strategy {
    pub matrix: Matrix,
    pub fail_fast: Option<bool>,
    pub max_parallel: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Matrix {
    pub include: Option<Vec<HashMap<String, String>>>,
    pub exclude: Option<Vec<HashMap<String, String>>>,
    pub dynamic: Option<HashMap<String, Vec<String>>>, // Added dynamic matrix support
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Environment {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Defaults {
    pub run: Option<RunDefaults>,
    pub retry: Option<RetryDefaults>,     // Added defaults for retry
    pub concurrency: Option<Concurrency>, // Added default concurrency settings
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RunDefaults {
    pub shell: Option<String>,
    pub working_directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RetryDefaults {
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Expression {
    pub value: String,
    pub parsed: Option<ParsedExpression>, // Added parsed representation of expressions
    pub evaluation_result: Option<bool>,  // Added evaluation result for expressions
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ParsedExpression {
    pub variables: Vec<String>, // Represents variables used within the expression
    pub functions: Vec<String>, // Represents functions or operators used within the expression
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Secret {
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RetryStrategy {
    pub max_attempts: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Artifacts {
    pub upload: Option<Vec<Artifact>>,   // Added artifacts for upload
    pub download: Option<Vec<Artifact>>, // Added artifacts for download
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Artifact {
    pub name: String,
    pub path: String,
    pub retention_days: Option<u32>,
}
