use crate::{
    base_task::_TaskId,
    core::{_LoopMsg, _SchedulerLoop},
    BaseTask, SchedulerTask,
};

use std::panic;
use thiserror::Error;
use tokio::{sync::mpsc, task};

const LOG_TARGET: &'static str = "scheduler";
const MAX_CHANNEL_SIZE: usize = u16::MAX as usize;

pub type SchedulerResult<T> = std::result::Result<T, SchedulerError>;

#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("Loop has not been started, please call start/1 before calling stop/0")]
    LoopNotStarted,
    #[error("A task inside the loop has paniced! Details: {0}")]
    PanicInsideLoop(String),
    #[error("Receiver was closed")]
    ChannelReceiverClosed,
}

pub struct Scheduler {
    loop_handle: Option<task::JoinHandle<()>>,
    channel_sender: Option<mpsc::Sender<_LoopMsg>>,
    heartbeat_interval: std::time::Duration,
}

impl Scheduler {
    pub fn new(heartbeat_interval: std::time::Duration) -> Self {
        Scheduler {
            loop_handle: None,
            channel_sender: None,
            heartbeat_interval,
        }
    }

    pub async fn add_task(
        &self,
        task: Box<(dyn SchedulerTask + panic::RefUnwindSafe + panic::UnwindSafe + Sync + Send)>,
        name: String,
        every: chrono::Duration,
        start_from: Option<chrono::DateTime<chrono::Utc>>,
    ) -> SchedulerResult<()> {
        let task = BaseTask::new(name, task, every, start_from);
        self._send_msg(_LoopMsg::AddTask(task)).await
    }

    pub async fn remove_task(&self, id: _TaskId) -> SchedulerResult<()> {
        self._send_msg(_LoopMsg::RemoveTask(id)).await
    }

    async fn _send_msg(&self, msg: _LoopMsg) -> SchedulerResult<()> {
        if self.channel_sender.is_some() {
            self.channel_sender
                .as_ref()
                .unwrap()
                .send(msg)
                .await
                .map_err(|_| SchedulerError::ChannelReceiverClosed)
        } else {
            Err(SchedulerError::LoopNotStarted)
        }
    }

    pub fn start(&mut self) -> SchedulerResult<()> {
        let heartbeat_interval = chrono::Duration::from_std(self.heartbeat_interval).unwrap();
        let (channel_sender, channel_receiver): (mpsc::Sender<_LoopMsg>, mpsc::Receiver<_LoopMsg>) =
            mpsc::channel(MAX_CHANNEL_SIZE);

        let loop_handle = tokio::spawn(async move {
            let mut task_loop = _SchedulerLoop::new(heartbeat_interval, channel_receiver);
            task_loop.start().await;
        });

        self.loop_handle = Some(loop_handle);
        self.channel_sender = Some(channel_sender);
        Ok(())
    }

    pub fn stop(self) -> SchedulerResult<()> {
        match self.loop_handle {
            Some(handle) => {
                handle.abort();
                Ok(())
            }
            None => Err(SchedulerError::LoopNotStarted),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::FunctionTask;
    use crate::Scheduler;
    #[tokio::test]
    async fn scheduler_start_test() {
        let mut scheduler = Scheduler::new(std::time::Duration::new(1, 0));
        assert!(scheduler.start().is_ok())
    }

    #[tokio::test]
    async fn scheduler_stop_test() {
        let mut scheduler = Scheduler::new(std::time::Duration::new(1, 0));
        let start_result = scheduler.start();
        let stop_result = scheduler.stop();
        assert!(start_result.is_ok());
        assert!(stop_result.is_ok())
    }

    #[tokio::test]
    async fn scheduler_add_task_test() {
        let scheduler = Scheduler::new(std::time::Duration::new(1, 0));
        let function_task = FunctionTask::new(move || {
            let mut y = String::from("foo");
            let t = String::from("bar");
            y += &t;
            assert!(*y == String::from("foobar"))
        });
        let name = String::from("task_name");
        let every = chrono::Duration::seconds(1);
        let result = scheduler.add_task(Box::new(function_task), name, every, None).await;
        assert!(result.is_ok());
    }
}
