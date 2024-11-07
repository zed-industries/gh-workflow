use std::path::Path;

use crate::error::{Error, Result};
use crate::Workflow;

pub struct Generate {
    workflow: Workflow,
    path: String,
}

impl Generate {
    pub fn new<P: ToString>(workflow: Workflow, path: P) -> Self {
        Self { workflow, path: path.to_string() }
    }

    pub fn generate(&self) -> Result<()> {
        let comment = include_str!("./comment.yml");
        let path = Path::new(self.path.as_str());

        path.parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .map_err(Error::IO)?;

        let content = format!("{}\n{}", comment, self.workflow.to_string()?);

        std::fs::write(path, content).map_err(Error::IO)?;

        println!(
            "Generated workflow file: {}",
            path.canonicalize()?.display()
        );
        Ok(())
    }
}
