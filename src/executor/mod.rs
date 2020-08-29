use crate::job::{Status, Work};

#[derive(Clone, Debug)]
pub struct Executor {
    alias: String,
}

impl Executor {
    pub fn new(alias: String) -> Executor {
        Executor { alias }
    }

    pub fn startup(&self) -> Result<(), String> {
        println!(":: Starting Executor {}::", self.alias);
        Ok(())
    }

    pub async fn execute(&self, job: &Box<dyn Work>) -> Result<(), String> {
        match job.func().await {
            Status::Success => Ok(()),
            Status::Running => Ok(()),
            Status::Retry => Ok(()),
            Status::Paused => Ok(()),
            Status::Uninitialized => Ok(()),
            Status::Failure(reason) => Err(reason),
        }
    }

    pub fn teardown(&self) -> Result<(), String> {
        println!(":: Shutting down Executor {} ::", self.alias);
        Ok(())
    }
}
