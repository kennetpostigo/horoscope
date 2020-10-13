use async_trait::async_trait;
use chrono::prelude::*;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::job::{Job, Work};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum JobState {
  Success,
  Failure,
}

#[async_trait]
pub trait Silo: Send + Sync {
  async fn startup(&mut self) -> Result<(), String> {
    println!(
      "{}{}",
      "::::   Starting JobStore "
        .truecolor(0, 0, 0)
        .bold()
        .on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(())
  }

  fn teardown(&self) -> Result<(), String> {
    println!(
      "{}{}",
      "::::   Tearing Down JobStore "
        .truecolor(0, 0, 0)
        .bold()
        .on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(())
  }

  fn vclone(&self) -> Box<dyn Silo>;
}

#[derive(Serialize, Deserialize)]
pub struct Store {
  pub alias: String,
  pub jobs: HashMap<String, Job>,
}

impl Store {
  pub fn new(alias: String) -> Store {
    Store {
      alias,
      jobs: HashMap::new(),
    }
  }

  pub async fn startup(&mut self) -> Result<(), String> {
    println!(
      "{}{}{}",
      "::::   Starting Memory JobStore "
        .truecolor(0, 0, 0)
        .bold()
        .on_green(),
      self.alias.truecolor(0, 0, 0).bold().on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(())
  }

  pub fn teardown(&self) -> Result<(), String> {
    println!(
      "{}{}{}",
      "::::   Tearing Down Memory JobStore "
        .truecolor(0, 0, 0)
        .bold()
        .on_green(),
      self.alias.truecolor(0, 0, 0).bold().on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(())
  }

  pub fn add_job(
    &mut self,
    alias: String,
    executor: String,
    start_time: i64,
    end_time: Option<i64>,
    job: Box<dyn Work>,
  ) -> Result<(), String> {
    self.jobs.entry(alias.clone()).or_insert(Job::new(
      alias.clone(),
      executor,
      start_time,
      end_time,
      HashMap::new(),
      job,
    ));
    Ok(())
  }

  // TODO: Implement this
  pub fn modify_job(&mut self, _alias: &String) -> Result<(), String> {
    Ok(())
  }

  pub fn pause_job(&mut self, alias: String) -> Result<(), String> {
    match self.jobs.entry(alias.clone()) {
      Entry::Occupied(mut entry) => {
        let j = entry.get_mut();
        j.pause_job()
      }
      Entry::Vacant(_entry) => Err(format!(
        "Failed to Pause Job {}, it's not found in Store {}",
        &alias, &self.alias
      )),
    }
  }

  pub fn resume_job(&mut self, alias: String) -> Result<(), String> {
    match self.jobs.entry(alias.clone()) {
      Entry::Occupied(mut entry) => {
        let j = entry.get_mut();
        j.resume_job()
      }
      Entry::Vacant(_entry) => Err(format!(
        "Failed to Resume Job {}, it's not found in Store {}",
        &alias, &self.alias
      )),
    }
  }

  pub fn remove_job(&mut self, alias: &String) -> Result<(), String> {
    match self.jobs.remove(alias) {
      Some(_) => Ok(()),
      None => Err(format!(
        "Job {} was not found in the Store {}",
        alias, &self.alias
      )),
    }
  }

  pub fn get_due_jobs(&mut self) -> Result<Vec<&mut Job>, String> {
    let mut ready = Vec::new();
    for (_key, value) in &mut self.jobs {
      let now = Utc::now().timestamp_nanos();

      if value.start_time <= now {
        ready.push(value);
      }
    }

    Ok(ready)
  }
}

impl Clone for Store {
  fn clone(&self) -> Store {
    Store {
      alias: self.alias.clone(),
      jobs: self.jobs.clone(),
    }
  }
}

impl Debug for Store {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Job")
      .field("alias", &self.alias)
      .field("job", &"<store>")
      .finish()
  }
}
