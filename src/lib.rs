#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

mod base_task;
mod core;
mod result;
mod scheduler;
mod scheduler_task;
mod tasks;
mod time;

#[cfg(feature = "custom_tasks")]
pub use core::_SchedulerTask;

pub use base_task::{BaseTask, BaseTaskError, BaseTaskResult, TaskId};
pub use result::Result;
pub use scheduler::{Scheduler, SchedulerError, SchedulerResult};
pub use scheduler_task::SchedulerTask;
pub use tasks::FunctionTask;
pub use time::StartFrom;
