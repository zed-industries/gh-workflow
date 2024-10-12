use crate::error::Result;

#[async_trait::async_trait]
pub trait Runtime {
    async fn write(&self, path: String, content: String) -> Result<()>;
    async fn read(&self, path: String) -> Result<String>;
}
