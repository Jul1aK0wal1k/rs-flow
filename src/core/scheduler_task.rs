use crate::SchedulerTask;

pub(crate) trait _SchedulerTask : SchedulerTask {
    type TimeZone: chrono::offset::TimeZone;
    fn last_execution(&self) -> Option<chrono::DateTime<Self::TimeZone>>;
    fn next_execution(&self) -> Option<chrono::DateTime<Self::TimeZone>>;
}