use crate::event::Event;
use crate::executor::Executor;
use crate::job::Job;
use crate::job_store::memory_job_store::JobStore;
use std::collections::HashMap;
use std::sync::Arc;

enum SchedulerState {
    Uninitialized,
    Running,
    Stopped,
}

pub struct Scheduler {
    job_stores: HashMap<String, Arc<JobStore>>,
    executors: HashMap<String, Executor>,
    listeners: Vec<Box<Fn(Event) -> ()>>,
    state: SchedulerState,
}
impl Scheduler {
    pub fn new() -> Scheduler {
        println!(":: Scheduler starting up ::");
        Scheduler {
            executors: HashMap::new(),
            job_stores: HashMap::new(),
            listeners: vec![],
            state: SchedulerState::Uninitialized,
        }
    }

    pub fn add_job_store(&mut self, job_store: JobStore, alias: String) {
        job_store.start(&self);
        self.job_stores.entry(alias).or_insert(Arc::new(job_store));
    }

    pub fn add_job(&mut self, alias: &String, job: Job, job_alias: String) {
        let store = self.job_stores.get(alias);
        let res = store.unwrap();

        match Arc::try_unwrap(res.clone()) {
            Ok(ref mut m) => {
                m.add_job(job, job_alias);
                return;
            }
            _ => return,
        }
    }

    pub fn add_executor(&mut self, executor: Executor, alias: String) {
        executor.start(self);
        self.executors.entry(alias).or_insert(executor);
    }

    pub fn remove_job_store(&mut self, alias: &String) {
        self.job_stores.remove(alias);
        return;
    }

    pub fn remove_executor(&mut self, alias: &String) {
        self.executors.remove(alias);
        return;
    }
}
