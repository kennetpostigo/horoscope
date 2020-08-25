use crate::job::{Job, Work};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct JobStore {
    pub alias: String,
    pub jobs: HashMap<String, Job>,
    // logger
}
impl JobStore {
    pub fn new(alias: String) -> JobStore {
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
        job: Box<dyn Work>,
        alias: String,
        executor: String,
        recurring: u128,
        until_success: i32,
        start_time: u128,
    ) {
        let cpy = alias.clone();
        self.jobs.entry(alias).or_insert(Job::new(
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

    pub fn get_due_jobs(&mut self) -> Vec<&Job> {
        let mut ready = Vec::new();
        for (_key, value) in &self.jobs {
            let start = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");
            let now = start.as_millis();

            if value.start_time <= now {
                ready.push(value);
            }
        }

        ready
    }
}