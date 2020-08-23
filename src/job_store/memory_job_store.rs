use crate::job::{Job, Work};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MemoryJob<T: Work> {
    pub job: Job<T>,
    pub start_time: u128,
}

impl<T: Work> MemoryJob<T> {
    pub fn new(job: T, alias: String) -> MemoryJob<T> {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");
        let start_time = start.as_millis();

        MemoryJob {
            job: Job::new(job, alias),
            start_time,
        }
    }
}

pub struct JobStore<T: Work> {
    pub alias: String,
    pub jobs: HashMap<String, MemoryJob<T>>,
    // logger
}
impl<T: Work> JobStore<T> {
    pub fn new(alias: String) -> JobStore<T> {
        JobStore {
            alias: alias.clone(),
            jobs: HashMap::new(),
        }
    }
    pub fn start(&mut self) {
        println!(":: Starting JobStore {} ::", self.alias)
    }

    pub fn shutdown(&self) {
        println!(":: Shutting down JobStore {} :: ", self.alias)
    }

    pub fn add_job(&mut self, job: T, alias: String, executor: String) {
        self.jobs
            .entry(alias)
            .or_insert(MemoryJob::new(job, executor));
    }

    pub fn remove_job(&mut self, alias: &String) {
        self.jobs.remove(alias);
    }

    pub fn get_due_jobs(&mut self) -> Vec<&Job<T>> {
        let mut ready = Vec::new();
        for (key, value) in &self.jobs {
            let start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");
            let now = start.as_millis();

            if value.start_time <= now {
                ready.push(&value.job);
            }
        }

        ready
    }
}

pub enum JobState {
    Success,
    Failure,
}
