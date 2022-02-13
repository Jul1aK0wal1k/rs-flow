#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]

mod scheduler;
mod scheduler_task;
mod result;
mod tasks;
mod core;
mod base_task;

pub use scheduler::Scheduler;
pub use base_task::BaseTask;
pub use result::Result;
pub use scheduler_task::SchedulerTask;