use derive_setters::Setters;

use crate::error::{Error, Result};
use crate::runtime::Runtime;
use crate::Workflow;

#[derive(Setters)]
pub struct Agent {
    workflow: Workflow,
    file: String,
    rtm: Box<dyn Runtime>,
}

impl Agent {
    pub fn new(workflow: Workflow, rtm: Box<dyn Runtime>) -> Self {
        Self {
            workflow,
            rtm,
            file: "./.github/workflows/ci.yml".to_string(),
        }
    }

    fn modify(&self, workflow: Workflow) -> Workflow {
        workflow
    }

    fn generate(&self) -> Result<String> {
        let workflow = self.modify(self.workflow.clone());
        Ok(serde_yaml::to_string(&self.workflow)?)
    }

    pub async fn write(&self) -> Result<()> {
        let content = self.generate()?;
        Ok(self.rtm.write(self.file.clone(), content).await?)
    }

    pub async fn check(&self, workflow: Workflow) -> Result<()> {
        let actual = self.rtm.read(self.file.clone()).await?;
        let expected = self.generate()?;

        if actual != expected {
            Err(Error::WorkflowMismatch)
        } else {
            Ok(())
        }
    }
}
