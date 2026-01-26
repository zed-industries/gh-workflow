//! Container configuration types for GitHub workflow jobs.

use derive_setters::Setters;
use serde::{Deserialize, Serialize};

use crate::env::Env;

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
    pub volumes: Option<Vec<String>>,

    /// Additional options for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,

    /// Hostname for the container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

impl Container {
    /// Creates a new `Container` with the given image.
    pub fn new<S: Into<String>>(image: S) -> Self {
        Self {
            image: image.into(),
            ..Default::default()
        }
    }

    /// Adds a port to the container.
    pub fn add_port<P: Into<Port>>(mut self, port: P) -> Self {
        let mut ports = self.ports.take().unwrap_or_default();
        ports.push(port.into());
        self.ports = Some(ports);
        self
    }

    /// Adds a volume to the container.
    pub fn add_volume<S: Into<String>>(mut self, volume: S) -> Self {
        let mut volumes = self.volumes.take().unwrap_or_default();
        volumes.push(volume.into());
        self.volumes = Some(volumes);
        self
    }

    /// Adds an environment variable to the container.
    pub fn add_env<E: Into<Env>>(mut self, new_env: E) -> Self {
        let mut env = self.env.take().unwrap_or_default();
        env.0.extend(new_env.into().0);
        self.env = Some(env);
        self
    }
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

impl Credentials {
    /// Creates new credentials with the given username and password.
    pub fn new<U: Into<String>, P: Into<String>>(username: U, password: P) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

/// Represents a network port.
///
/// Ports can be specified as either a number (for same host/container port)
/// or a string in "host:container" format.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Port {
    /// A port specified by its number.
    Number(u16),

    /// A port mapping specified as a string (e.g., "5432:5432" or "8080:80").
    Mapping(String),
}

impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Port::Number(port)
    }
}

impl From<String> for Port {
    fn from(mapping: String) -> Self {
        Port::Mapping(mapping)
    }
}

impl From<&str> for Port {
    fn from(mapping: &str) -> Self {
        Port::Mapping(mapping.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_serialization() {
        // Test number port
        let port = Port::Number(5432);
        let yaml = serde_yaml::to_string(&port).expect("serialize port number");
        assert_eq!(yaml.trim(), "5432");

        // Test mapping port - YAML may or may not quote based on content
        let port = Port::Mapping("5432:5432".to_string());
        let yaml = serde_yaml::to_string(&port).expect("serialize port mapping");
        assert!(yaml.contains("5432:5432"));
    }

    #[test]
    fn test_container_serialization() {
        let container = Container::new("postgres:15")
            .add_env(("POSTGRES_USER", "postgres"))
            .add_env(("POSTGRES_PASSWORD", "postgres"))
            .add_port("5432:5432")
            .options("--health-cmd pg_isready --health-interval 10s");

        let yaml = serde_yaml::to_string(&container).expect("serialize container");
        println!("Container YAML:\n{}", yaml);

        assert!(yaml.contains("image: postgres:15"));
        assert!(yaml.contains("POSTGRES_USER: postgres"));
        assert!(yaml.contains("5432:5432"));
        assert!(yaml.contains("--health-cmd pg_isready"));
    }

    #[test]
    fn test_container_with_credentials() {
        let container = Container::new("ghcr.io/myorg/myimage:latest")
            .credentials(Credentials::new("${{ github.actor }}", "${{ secrets.GHCR_TOKEN }}"));

        let yaml = serde_yaml::to_string(&container).expect("serialize container");
        assert!(yaml.contains("username:"));
        assert!(yaml.contains("password:"));
    }
}
