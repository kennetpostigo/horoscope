pub mod network;
pub mod sys;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::trigger::Trigger;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Status {
  Waiting,
  Running,
  Paused,
  Success,
  Failure(String),
}

impl Status {
  pub fn to_string(&self) -> String {
    match self {
      Status::Waiting => String::from("Waiting"),
      Status::Running => String::from("Running"),
      Status::Paused => String::from("Paused"),
      Status::Success => String::from("Success"),
      Status::Failure(_) => String::from("Failure"),
    }
  }
}

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait Work: Send + Sync {
  async fn startup(&self) -> Result<(), String>;

  async fn func(&self) -> Status;

  async fn teardown(&self) -> Result<(), String>;

  fn vclone(&self) -> Box<dyn Work>;
}

#[derive(Serialize, Deserialize)]
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
  pub fn new(
    alias: String,
    executor: String,
    start_time: i64,
    end_time: Option<i64>,
    triggers: HashMap<String, Box<Trigger>>,
    job: Box<dyn Work>,
  ) -> Job {
    Job {
      state: Status::Waiting,
      alias,
      executor,
      start_time,
      end_time,
      triggers,
      job,
    }
  }

  pub async fn validate_triggers(&mut self) -> (bool, Option<i64>) {
    let mut should_run = true;
    let mut next = None;
    for (_, value) in &mut self.triggers {
      let trig = &mut value.trigger;
      if !(trig.should_run().await) {
        should_run = false;
      } else {
        match trig.next().await {
          Some(v) => match next {
            Some(curr) => {
              if v < curr {
                next = Some(v);
              }
            }
            None => next = Some(v),
          },
          None => (),
        }
      }
    }

    (should_run, next)
  }

  // TODO: Implement Modify Job
  pub fn modify_job(&mut self) -> Result<(), String> {
    Ok(())
  }

  // TODO: Implement Pause Job
  pub fn pause_job(&mut self) -> Result<(), String> {
    self.state = Status::Paused;
    Ok(())
  }

  // TODO: Implement Resume Job
  pub fn resume_job(&mut self) -> Result<(), String> {
    self.state = Status::Running;
    Ok(())
  }

  pub fn add_trigger(&mut self, trigger: Trigger) -> Result<(), String> {
    match self.triggers.entry(trigger.alias.clone()) {
      Entry::Occupied(_) => {
        Err(format!("Trigger {} already exists", trigger.alias.clone()))
      }
      Entry::Vacant(e) => {
        e.insert(Box::new(trigger));
        Ok(())
      }
    }
  }

  pub fn remove_trigger(mut self, trigger_alias: String) -> Result<(), String> {
    match self.triggers.entry(trigger_alias.clone()) {
      Entry::Occupied(e) => {
        e.remove();
        Ok(())
      }
      Entry::Vacant(_) => {
        Err(format!("Trigger {} doesn't exists", trigger_alias.clone()))
      }
    }
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
