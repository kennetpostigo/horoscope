use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::job::{Job, Work};
use crate::store::Silo;

#[derive(Clone, Debug)]
pub struct Store {
  pub alias: String,
  pub jobs: HashMap<String, Job>,
  // logger
}

impl Store {
  pub fn new(alias: String) -> Self {
    Store {
      alias: alias.clone(),
      jobs: HashMap::new(),
    }
  }
}

impl Silo for Store {
  fn start(&mut self) -> Result<(), String> {
    println!(":: Starting JobStore {} ::", self.alias);
    Ok(())
  }

  fn teardown(&self) -> Result<(), String> {
    println!(":: Shutting down JobStore {} :: ", self.alias);
    Ok(())
  }

  fn add_job(
    &mut self,
    alias: String,
    executor: String,
    start_time: u128,
    end_time: Option<u128>,
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
      let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");
      let now = start.as_millis();

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
