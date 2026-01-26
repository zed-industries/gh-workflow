//!
//! Job-related structures and implementations for GitHub workflow jobs.

use std::time::Duration;

use derive_setters::Setters;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::concurrency::Concurrency;
use crate::step::{Step, StepType, StepValue};
use crate::{
    Artifacts, Container, Defaults, Env, Expression, Permissions, RetryStrategy, Secret, Strategy,
};

/// Represents the environment in which a job runs.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(transparent)]
pub struct RunsOn(Value);

impl<T> From<T> for RunsOn
where
    T: Into<Value>,
{
    /// Converts a value into a `RunsOn` instance.
    fn from(value: T) -> Self {
        RunsOn(value.into())
    }
}

/// Represents a job in the workflow.
#[derive(Debug, Setters, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
#[setters(strip_option, into)]
pub struct Job {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "if")]
    pub cond: Option<Expression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runs_on: Option<RunsOn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "env")]
    pub envs: Option<Env>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<Vec<StepValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<IndexMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_minutes: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<IndexMap<String, Container>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<IndexMap<String, Secret>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaults: Option<Defaults>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_on_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
}

impl Job {
    /// Creates a new `Job` with the specified name and default settings.
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: Some(name.to_string()),
            runs_on: Some(RunsOn(Value::from("ubuntu-latest"))),
            ..Default::default()
        }
    }

    /// Sets the timeout for the job.
    pub fn timeout(self, duration: Duration) -> Self {
        self.timeout_minutes(duration.as_secs() as u32 / 60)
    }

    /// Adds a step to the job.
    pub fn add_step<S: Into<Step<T>>, T: StepType>(mut self, step: S) -> Self {
        let mut steps = self.steps.take().unwrap_or_default();
        let step: Step<T> = step.into();
        let step: StepValue = T::to_value(step);
        steps.push(step);
        self.steps = Some(steps);
        self
    }

    /// Adds an environment variable to the job.
    pub fn add_env<T: Into<Env>>(mut self, new_env: T) -> Self {
        let mut env = self.envs.take().unwrap_or_default();

        env.0.extend(new_env.into().0);
        self.envs = Some(env);
        self
    }

    /// Adds a job dependency.
    pub fn add_need<J: ToString>(mut self, job_id: J) -> Self {
        if let Some(needs) = self.needs.as_mut() {
            needs.push(job_id.to_string());
        } else {
            self.needs = Some(vec![job_id.to_string()]);
        }
        self
    }

    /// Adds an output to the job.
    pub fn add_output<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        let mut outputs = self.outputs.take().unwrap_or_default();
        outputs.insert(key.to_string(), value.to_string());
        self.outputs = Some(outputs);
        self
    }

    /// Adds a service container to the job.
    ///
    /// Services are Docker containers that provide additional functionality
    /// for the job, such as databases or caches.
    ///
    /// # Example
    /// ```ignore
    /// use gh_workflow::{Job, Container, Port};
    ///
    /// let job = Job::new("test")
    ///     .add_service(
    ///         "postgres",
    ///         Container::new("postgres:15")
    ///             .add_env(("POSTGRES_USER", "postgres"))
    ///             .add_env(("POSTGRES_PASSWORD", "postgres"))
    ///             .add_port("5432:5432")
    ///             .options("--health-cmd pg_isready --health-interval 10s --health-timeout 5s --health-retries 5"),
    ///     );
    /// ```
    pub fn add_service<K: ToString, V: Into<Container>>(mut self, name: K, service: V) -> Self {
        let mut services = self.services.take().unwrap_or_default();
        services.insert(name.to_string(), service.into());
        self.services = Some(services);
        self
    }

    /// Adds a secret to the job.
    pub fn add_secret<K: ToString, V: Into<Secret>>(mut self, key: K, secret: V) -> Self {
        let mut secrets = self.secrets.take().unwrap_or_default();
        secrets.insert(key.to_string(), secret.into());
        self.secrets = Some(secrets);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::{Container, Port};

    #[test]
    fn test_job_with_service() {
        let job = Job::new("test")
            .add_service(
                "postgres",
                Container::new("postgres:15")
                    .add_env(("POSTGRES_USER", "postgres"))
                    .add_env(("POSTGRES_PASSWORD", "postgres"))
                    .add_port(Port::Mapping("5432:5432".to_string()))
                    .options("--health-cmd pg_isready --health-interval 10s --health-timeout 5s --health-retries 5"),
            );

        let yaml = serde_yaml::to_string(&job).expect("serialize job");
        println!("Job YAML:\n{}", yaml);

        assert!(yaml.contains("services:"));
        assert!(yaml.contains("postgres:"));
        assert!(yaml.contains("image: postgres:15"));
        assert!(yaml.contains("POSTGRES_USER: postgres"));
        assert!(yaml.contains("5432:5432"));
        assert!(yaml.contains("--health-cmd pg_isready"));
    }

    #[test]
    fn test_job_with_multiple_services() {
        let job = Job::new("integration-test")
            .add_service(
                "postgres",
                Container::new("postgres:15")
                    .add_env(("POSTGRES_PASSWORD", "postgres"))
                    .add_port("5432:5432"),
            )
            .add_service(
                "redis",
                Container::new("redis:7")
                    .add_port("6379:6379"),
            );

        let yaml = serde_yaml::to_string(&job).expect("serialize job");
        println!("Job with multiple services YAML:\n{}", yaml);

        assert!(yaml.contains("postgres:"));
        assert!(yaml.contains("redis:"));
        assert!(yaml.contains("postgres:15"));
        assert!(yaml.contains("redis:7"));
    }
}
