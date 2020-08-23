use crate::job::{Job, Work};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct MemoryJob<T: Work + Clone> {
    pub job: Job<T>,
    pub start_time: u128,
}

impl<T: Work + Clone> MemoryJob<T> {
    pub fn new(
        job: T,
        alias: String,
        executor: String,
        recurring: i128,
        until_success: i32,
        start_time: u128,
    ) -> MemoryJob<T> {
        MemoryJob {
            job: Job::new(job, alias, executor, recurring, until_success, start_time),
            start_time,
        }
    }
}

#[derive(Clone, Debug)]
pub struct JobStore<T: Work + Clone> {
    pub alias: String,
    pub jobs: HashMap<String, MemoryJob<T>>,
    // logger
}
impl<T: Work + Clone> JobStore<T> {
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

    pub fn add_job(
        &mut self,
        job: T,
        alias: String,
        executor: String,
        recurring: i128,
        until_success: i32,
        start_time: u128,
    ) {
        let cpy = alias.clone();
        self.jobs.entry(alias).or_insert(MemoryJob::new(
            job,
            cpy,
            executor,
            recurring,
            until_success,
            start_time,
        ));
    }

    pub fn remove_job(&mut self, alias: &String) {
        self.jobs.remove(alias);
    }

    pub fn get_due_jobs(&mut self) -> Vec<&Job<T>> {
        let mut ready = Vec::new();
        for (_key, value) in &self.jobs {
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

#[derive(Clone, Debug)]
pub enum JobState {
    Success,
    Failure,
}
