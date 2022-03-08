use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Failed Task execution, reason {0}")]
    FailedTaskExecution(String),
}

pub type TaskResult = std::result::Result<(), TaskError>;

#[async_trait]
pub trait SchedulerTask {
    async fn execute(&self) -> TaskResult;
}
