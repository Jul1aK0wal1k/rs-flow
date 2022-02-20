use async_trait::async_trait;
use std::io::Result;

#[async_trait]
pub trait SchedulerTask {
    async fn execute(&self) -> Result<()>;
}
