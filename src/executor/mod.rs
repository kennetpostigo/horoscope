use crate::job::{Status, Work};

#[derive(Clone, Debug)]
pub struct Executor {
    alias: String,
}

impl Executor {
    pub fn new(alias: String) -> Executor {
        Executor { alias }
    }

    pub fn start(&self) {
        println!(":: Starting Executor {}::", self.alias)
    }

    pub async fn execute(&self, job: &impl Work) -> Status {
        job.func().await
    }

    pub fn shutdown(&self) {
        println!(":: Shutting down Executor {} ::", self.alias)
    }
}
