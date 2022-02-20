use async_trait::async_trait;

use crate::SchedulerTask;

pub struct FunctionTask<Func: (Fn() -> ()) + Sync + Send> {
    func: Func,
}

impl<Func: Fn() -> () + Sync + Send> FunctionTask<Func> {
    pub fn new(func: Func) -> Self {
        FunctionTask { func }
    }
}

#[async_trait]
impl<Func: Fn() -> () + Sync + Send> SchedulerTask for FunctionTask<Func> {
    async fn execute(&self) -> std::io::Result<()> {
        (self.func)();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{tasks::FunctionTask, SchedulerTask};

    #[tokio::test]
    async fn function_test() {
        let add_one_task = FunctionTask::new(|| {
            let mut s = String::from("foo");
            let t = String::from("bar");
            s += &t;
            assert!(s == String::from("foobar"))
        });
        assert!(add_one_task.execute().await.is_ok())
    }

    #[tokio::test]
    async fn function_with_move_test() {
        let x = String::from("foo");
        let add_one_task = FunctionTask::new(move || {
            let mut y = x.clone();
            let t = String::from("bar");
            y += &t;
            assert!(*y == String::from("foobar"))
        });
        assert!(add_one_task.execute().await.is_ok())
    }
}
