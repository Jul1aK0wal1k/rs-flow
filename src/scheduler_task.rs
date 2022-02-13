use std::io::Result;

pub trait SchedulerTask {
    fn execute(&self) -> Result<()>;
}
