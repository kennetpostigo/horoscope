pub mod cron;
pub mod network;

use crate::trigger::Trigger;
use async_trait::async_trait;
use std::collections::HashMap;
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

impl Status {
  pub fn to_string(&self) -> String {
    match self {
      Status::Uninitialized => String::from("Uninitialized"),
      Status::Running => String::from("Running"),
      Status::Paused => String::from("Paused"),
      Status::Retry => String::from("Retry"),
      Status::Success => String::from("Success"),
      Status::Failure(_) => String::from("Failure"),
    }
  }
}

// Time Trigger
// Ledger Trigger
// User-defined Trigger

// pub async fn deserialize(job_str: String) -> Box<dyn Work>  {
  
// }

// pub async fn serialize(job_str: Box<dyn Work>) -> String {
//   for 
// }

#[async_trait]
pub trait Work
where
  Self: Send + Sync, {
  async fn startup(&self) -> Result<(), String>;

  async fn func(&self) -> Status;

  async fn teardown(&self) -> Result<String, String>;

  fn vclone(&self) -> Box<dyn Work>;
}

pub struct Job {
  pub state: Status,
  pub alias: String,
  pub executor: String,
  pub start_time: i64,
  pub end_time: Option<i64>,
  pub triggers: HashMap<String, Box<Trigger>>,
  pub job: Box<dyn Work>,
}

impl Job {
  // TODO: figure out how to default recurring to 0
  pub fn new(
    alias: String,
    executor: String,
    start_time: i64,
    end_time: Option<i64>,
    triggers: HashMap<String, Box<Trigger>>,
    job: Box<dyn Work>,
  ) -> Job {
    Job {
      state: Status::Uninitialized,
      alias,
      executor,
      start_time,
      end_time,
      triggers,
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
      triggers: self.triggers.clone(),
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
      .field("triggers", &self.triggers)
      .field("job", &"<job>")
      .finish()
  }
}
