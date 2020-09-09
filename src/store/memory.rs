use chrono::prelude::*;
use colored::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use async_trait::async_trait;

use crate::job::{Job, Work};
use crate::store::Silo;

#[derive(Clone, Debug)]
pub struct Store {
  pub alias: String,
  pub jobs: HashMap<String, Job>,
  // logger
}

impl Store {
  pub fn new(alias: String) -> Store {
    Store {
      alias: alias.clone(),
      jobs: HashMap::new(),
    }
  }
}

#[async_trait]
impl Silo for Store {
  async fn startup(&mut self) -> Result<(), String> {
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

  fn teardown(&self) -> Result<(), String> {
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

  fn add_job(
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
  fn modify_job(&mut self, _alias: &String) -> Result<(), String> {
    Ok(())
  }

  fn pause_job(&mut self, alias: String) -> Result<(), String> {
    match self.jobs.entry(alias.clone()) {
      Entry::Occupied(mut entry) => {
        let j = entry.get_mut();
        j.pause_job()
      }
      Entry::Vacant(_entry) => Err(format!(
        "Failed to Pause Job {}, it not found in Store {}",
        &alias, &self.alias
      )),
    }
  }

  fn resume_job(&mut self, alias: String) -> Result<(), String> {
    match self.jobs.entry(alias.clone()) {
      Entry::Occupied(mut entry) => {
        let j = entry.get_mut();
        j.resume_job()
      }
      Entry::Vacant(_entry) => Err(format!(
        "Failed to Resume Job {}, it not found in Store {}",
        &alias, &self.alias
      )),
    }
  }

  fn remove_job(&mut self, alias: &String) -> Result<(), String> {
    match self.jobs.remove(alias) {
      Some(_) => Ok(()),
      None => Err(format!(
        "Job {} was not found in the Store {}",
        alias, &self.alias
      )),
    }
  }

  fn get_due_jobs(&mut self) -> Result<Vec<&Job>, String> {
    let mut ready = Vec::new();
    for (_key, value) in &self.jobs {
      let now = Utc::now().timestamp_nanos();

      if value.start_time <= now {
        ready.push(value);
      }
    }

    Ok(ready)
  }

  fn vclone(&self) -> Box<dyn Silo> {
    Box::new(self.clone())
  }
}
