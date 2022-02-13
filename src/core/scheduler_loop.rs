use log::info;
use thiserror::Error;
use tokio::sync::mpsc;

use crate::{base_task::_TaskId, core::_SchedulerTask, BaseTask, SchedulerTask};

const LOG_TARGET: &'static str = "scheduler_loop";

pub(crate) type _LoopResult<T> = std::result::Result<T, _LoopError>;

#[derive(Error, Debug)]
pub(crate) enum _LoopError {
    #[error("Thread panicked while lock was taken. Panic details: {0}")]
    LockPoisoning(String),
    #[error("Channel disconnected")]
    ChannelDisconnected,
    #[error("Channel is currently empty")]
    ChannelEmpty,
}

pub(crate) enum _LoopMsg {
    AddTask(BaseTask),
    RemoveTask(_TaskId),
}

pub(crate) struct _SchedulerLoop {
    heartbeat_interval: chrono::Duration,
    last_heartbeat: chrono::DateTime<chrono::Utc>,
    msg_receiver: mpsc::Receiver<_LoopMsg>,
    tasks: Vec<BaseTask>,
}

impl _SchedulerLoop {
    pub(crate) fn new(
        heartbeat_interval: chrono::Duration,
        msg_receiver: mpsc::Receiver<_LoopMsg>,
    ) -> Self {
        _SchedulerLoop {
            heartbeat_interval,
            last_heartbeat: chrono::Utc::now(),
            msg_receiver,
            tasks: Vec::default(),
        }
    }

    pub(crate) fn add_task(&mut self, task: BaseTask) {
        self.tasks.push(task);
    }

    pub(crate) fn remove_task(&mut self, id: _TaskId) {
        self.tasks.retain(|t| t.id != id);
    }

    async fn handle_channel_msgs(&mut self) -> _LoopResult<()> {
        match self.msg_receiver.try_recv().map(|msg| match msg {
            _LoopMsg::AddTask(task) => self.add_task(task),
            _LoopMsg::RemoveTask(id) => self.remove_task(id),
        }) {
            Ok(res) => Ok(res),
            Err(err) => match err {
                mpsc::error::TryRecvError::Empty => Ok(()),
                mpsc::error::TryRecvError::Disconnected => Err(_LoopError::ChannelDisconnected),
            },
        }
    }

    pub(crate) async fn start(&mut self) {
        loop {
            let time_now = chrono::Utc::now();
            let time_diff = self.last_heartbeat - time_now;
            let should_tick = time_diff > self.heartbeat_interval;
            if should_tick {
                let tasks_to_run = self.tasks.iter().filter(|x| {
                    let next_execution = x.next_execution();
                    let last_execution = x.last_execution();
                    last_execution.is_none()
                        || (next_execution.is_some() && time_now >= next_execution.unwrap())
                });

                let _result = tasks_to_run.map(|task| {
                    task.execute().ok();
                });

                {
                    self.last_heartbeat = time_now;
                }
                info!(target: LOG_TARGET, "Heartbeat tick");
            }
            tokio::time::sleep(time_diff.to_std().unwrap()).await;
        }
    }
}
