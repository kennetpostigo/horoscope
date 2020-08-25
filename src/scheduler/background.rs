use async_std::task;
use async_trait::async_trait;
use std::fmt::Debug;

// use crate::event::Event;
use crate::executor::Executor;
use crate::job::Work;
use crate::scheduler::{Schedule, SchedulerState};
use crate::store::Ledger;

pub struct Scheduler {
    pub scheduler: Box<dyn Schedule>,
    pub state: SchedulerState,
    // pub listeners: Vec<Box<Fn(Event) -> ()>>,
}

impl Scheduler {
    pub fn new(scheduler: Box<dyn Schedule>) -> Self {
        Scheduler {
            scheduler,
            state: SchedulerState::Uninitialized,
            // listeners: vec![],
        }
    }
}

#[async_trait]
impl Schedule for Scheduler {
    fn startup(&mut self) {
        task::spawn(self.scheduler.async_startup());
    }

    async fn async_startup(&mut self) {
        println!("Background Scheduler doesn't utilize a async startup");
    }

    fn add_store(&mut self, job_store: Box<dyn Ledger>, alias: String) {
        self.scheduler.add_store(job_store, alias)
    }

    fn add_job(
        &mut self,
        store_alias: String,
        alias: String,
        job: Box<dyn Work>,
        executor: String,
        recurring: u128,
        until_success: i32,
        start_time: u128,
    ) {
        self.scheduler.add_job(
            store_alias,
            alias,
            job,
            executor,
            recurring,
            until_success,
            start_time,
        )
    }

    fn add_executor(&mut self, executor: Executor, alias: String) {
        self.scheduler.add_executor(executor, alias)
    }

    fn remove_store(&mut self, alias: &String) {
        self.scheduler.remove_store(alias)
    }

    fn remove_job(&mut self, alias: String, job_alias: String) {
        self.scheduler.remove_job(alias, job_alias)
    }

    fn remove_executor(&mut self, alias: &String) {
        self.scheduler.remove_executor(alias)
    }

    fn vclone(&self) -> Box<dyn Schedule> {
        Box::new(self.clone())
    }
}

impl Clone for Scheduler {
    fn clone(&self) -> Self {
        Scheduler {
            state: self.state.clone(),
            scheduler: self.scheduler.vclone(),
        }
    }
}

impl Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("state", &self.state)
            .field("job", &"<scheduler>")
            .finish()
    }
}
