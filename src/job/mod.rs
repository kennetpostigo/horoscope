pub mod cron;
pub mod network;

use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Status {
    Success,
    Retry,
    Failure,
}

#[async_trait]
pub trait Work
where
    Self: Send + Sync,
{
    async fn startup(&self);
    async fn func(&self) -> Status;
    async fn teardown(&self);
    fn vclone(&self) -> Box<dyn Work>;
}

pub struct Job {
    pub alias: String,
    pub executor: String,
    pub recurring: u128,
    pub until_success: i32,
    pub start_time: u128,
    pub job: Box<dyn Work>,
}

impl Job {
    // TODO: figure out how to default recurring to 0
    pub fn new(
        job: Box<dyn Work>,
        alias: String,
        executor: String,
        recurring: u128,
        until_success: i32,
        start_time: u128,
    ) -> Job {
        Job {
            job,
            alias,
            recurring,
            until_success,
            executor,
            start_time,
        }
    }
}


impl Clone for Job {
    fn clone(&self) -> Self {
        Job {
            alias: self.alias.clone(),
            executor: self.executor.clone(),
            recurring: self.recurring,
            until_success: self.until_success,
            start_time: self.start_time,
            job: self.job.vclone(),
        }
    }
}

impl Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("alias", &self.alias)
            .field("executor", &self.executor)
            .field("recurring", &self.recurring)
            .field("until_success", &self.until_success)
            .field("start_time", &self.start_time)
            .field("job", &"<job>")
            .finish()
    }
}