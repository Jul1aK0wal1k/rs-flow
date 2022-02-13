use crate::SchedulerTask;

pub struct FunctionTask<Func: Fn() -> ()> {
    func: Func,
}

impl<Func: Fn() -> ()> FunctionTask<Func> {
    pub fn new(func: Func) -> Self {
        FunctionTask { func }
    }
}

impl<Func: Fn() -> ()> SchedulerTask for FunctionTask<Func> {
    fn execute(&self) -> std::io::Result<()> {
        (self.func)();
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crate::{SchedulerTask, tasks::FunctionTask};

    #[test]
    fn function_test() {
        let add_one_task = FunctionTask::new(|| {
            let mut s = String::from("foo");
            let t = String::from("bar");
            s += &t;
            assert!(s == String::from("foobar"))
        });
        assert!(add_one_task.execute().is_ok()) 
    }

    
    #[test]
    fn function_with_move_test() {
        let x  = String::from("foo");
        let add_one_task = FunctionTask::new( move || {
            let mut y = x.clone();
            let t = String::from("bar");
            y += &t;
            assert!(*y == String::from("foobar"))
        });
        assert!(add_one_task.execute().is_ok()) 
    }
}