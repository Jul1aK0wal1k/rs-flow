use crate::{core::_SchedulerTask, SchedulerTask};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;
use thiserror::Error;

const LOG_TARGET: &'static str = "base_task";

pub type TaskId = String;

pub type BaseTaskResult<T> = std::result::Result<T, BaseTaskError>;

#[derive(Error, Debug)]
pub enum BaseTaskError {
    #[error("Datetime parse error, reason {0}")]
    FailedDateTimeParse(String),
}

#[derive(Clone)]
pub struct BaseTask {
    pub id: TaskId,
    pub name: String,
    task: Arc<Box<(dyn SchedulerTask + Send + Sync)>>,
    every: Duration,
    start_from: Option<DateTime<Utc>>,
    last_execution: Option<DateTime<Utc>>,
}

impl BaseTask {
    pub fn new(
        id: String,
        name: String,
        task: Box<(dyn SchedulerTask + Send + Sync)>,
        every: Duration,
        start_from: Option<DateTime<Utc>>,
    ) -> BaseTaskResult<Self> {
        Ok(BaseTask {
            id,
            name,
            task: Arc::new(task),
            every,
            start_from,
            last_execution: None,
        })
    }
}

#[async_trait]
impl SchedulerTask for BaseTask {
    async fn execute(&self) -> std::result::Result<(), std::io::Error> {
        if let Err(err) = self.task.execute().await {
            log::warn!(target: LOG_TARGET, "In: {} {:?}", self.name, err);
        }
        Ok(())
    }
}

impl _SchedulerTask for BaseTask {
    type TimeZone = Utc;

    fn last_execution(&self) -> Option<DateTime<Self::TimeZone>> {
        self.last_execution
    }

    fn next_execution(&self) -> Option<DateTime<Self::TimeZone>> {
        match self.last_execution {
            Some(time) => time.checked_add_signed(self.every),
            None => match self.start_from {
                Some(time) => Some(time),
                None => Some(Self::TimeZone::now()),
            },
        }
    }
}
