use std::sync::Arc;

use crate::{SchedulerTask, core::_SchedulerTask};

const LOG_TARGET: &'static str = "base_task";

pub(crate) type _TaskId = String;

#[derive(Clone)]
pub struct BaseTask {
    pub id: _TaskId,
    pub name: String,
    task: Arc<Box<(dyn SchedulerTask + Send + Sync)>>,
    pub every: chrono::Duration,
    pub start_from: Option<chrono::DateTime<chrono::Utc>>,
    pub last_execution: Option<chrono::DateTime<chrono::Utc>>,
}

impl BaseTask {
    pub fn new(
        name: String,
        task: Box<(dyn SchedulerTask + Send + Sync)>,
        every: chrono::Duration,
        start_from: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        BaseTask {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            task: Arc::new(task),
            every,
            start_from,
            last_execution: None,
        }
    }
}

impl SchedulerTask for BaseTask {
    fn execute(&self) -> std::result::Result<(), std::io::Error> {
        if let Err(err) = self.task.execute() {
            log::warn!(target: LOG_TARGET, "In: {} {:?}", self.name, err);
        }
        Ok(())
    }
}

impl _SchedulerTask for BaseTask {
    type TimeZone = chrono::Utc;

    fn last_execution(&self) -> Option<chrono::DateTime<Self::TimeZone>> {
        self.last_execution
    }

    fn next_execution(&self) -> Option<chrono::DateTime<Self::TimeZone>> {
        match self.last_execution {
            Some(time) => time.checked_add_signed(self.every),
            None => match self.start_from {
                Some(time) => Some(time),
                None => Some(Self::TimeZone::now()),
            },
        }
    }
}


