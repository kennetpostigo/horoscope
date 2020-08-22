use crate::job::Job;
use crate::scheduler::blocking_scheduler::Scheduler;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

pub struct MemoryJob {
    job: Arc<Job>,
    start_time: SystemTime,
}

impl MemoryJob {
    pub fn new(job: Job) -> MemoryJob {
        MemoryJob {
            job: Arc::new(job),
            start_time: SystemTime::now(),
        }
    }
}

pub struct JobStore {
    scheduler: Scheduler,
    alias: Option<String>,
    jobs: HashMap<String, MemoryJob>,
    // logger
}
impl JobStore {
    pub fn start(&self, scheduler: &Scheduler) {}

    pub fn shutdown(&self) {}

    pub fn add_job(&mut self, job: Job, alias: String) {
        self.jobs.entry(alias).or_insert(MemoryJob::new(job));
    }

    pub fn remove_job(&mut self, alias: &String) {
        self.jobs.remove(alias);
    }
}

pub enum JobState {
    Success,
    Failure,
}
