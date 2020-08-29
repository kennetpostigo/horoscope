pub mod cron;
pub mod network;

use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum Status {
    Uninitialized,
    Running,
    Paused,
    Retry,
    Success,
    Failure(String),
}

// Time Trigger
// Ledger Trigger
// User-defined Trigger

#[async_trait]
pub trait Work
where
    Self: Send + Sync,
{
    async fn startup(&self) -> Result<(), String>;

    async fn func(&self) -> Status;

    async fn teardown(&self) -> Result<String, String>;

    fn vclone(&self) -> Box<dyn Work>;
}

#[async_trait]
pub trait Trigger
where
    Self: Send + Sync,
{
    async fn should_run(&self) -> bool;

    async fn next(&self) -> u128;

    fn vclone(&self) -> Box<dyn Trigger>;
}

pub struct Job {
    pub state: Status,
    pub alias: String,
    pub executor: String,
    pub start_time: u128,
    pub end_time: Option<u128>,
    // TODO: pub trigger: Box<dyn Trigger>,
    pub job: Box<dyn Work>,
}

impl Job {
    // TODO: figure out how to default recurring to 0
    pub fn new(
        alias: String,
        executor: String,
        start_time: u128,
        end_time: Option<u128>,
        job: Box<dyn Work>,
    ) -> Job {
        Job {
            state: Status::Uninitialized,
            alias,
            executor,
            start_time,
            end_time,
            //TODO: trigger: Box::new(Trigger)
            job,
        }
    }

    pub fn modify_job(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn pause_job(&mut self) -> Result<(), String> {
        self.state = Status::Paused;
        Ok(())
    }

    pub fn resume_job(&mut self) -> Result<(), String> {
        self.state = Status::Running;
        Ok(())
    }
}

impl Clone for Job {
    fn clone(&self) -> Self {
        Job {
            state: self.state.clone(),
            alias: self.alias.clone(),
            executor: self.executor.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            job: self.job.vclone(),
        }
    }
}

impl Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("state", &self.state)
            .field("alias", &self.alias)
            .field("executor", &self.executor)
            .field("start_time", &self.start_time)
            .field("end_success", &self.end_time)
            .field("job", &"<job>")
            .finish()
    }
}
