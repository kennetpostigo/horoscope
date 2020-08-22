use crate::scheduler::blocking_scheduler::Scheduler;

pub struct Executor {
    scheduler: Scheduler,
    alias: String,
}

impl Executor {
    pub fn start(&self, scheduler: &Scheduler) {
        print!("Starting Executor {}", self.alias)
    }
    pub fn shutdown(&self) {
        print!("Shutting down Executor {}", self.alias)
    }
}
