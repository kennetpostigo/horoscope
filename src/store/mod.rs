pub mod memory;

use async_trait::async_trait;
use colored::*;
use std::fmt::Debug;

use crate::job::{Job, Work};

#[derive(Clone, Debug)]
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

  fn add_job(
    &mut self,
    alias: String,
    executor: String,
    start_time: i64,
    end_time: Option<i64>,
    job: Box<dyn Work>,
  ) -> Result<(), String>;

  fn modify_job(&mut self, alias: &String) -> Result<(), String>;

  fn pause_job(&mut self, alias: String) -> Result<(), String>;

  fn resume_job(&mut self, alias: String) -> Result<(), String>;

  fn remove_job(&mut self, alias: &String) -> Result<(), String>;

  fn get_due_jobs(&mut self) -> Result<Vec<&mut Job>, String>;

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

pub struct Store {
  pub alias: String,
  pub store: Box<dyn Silo>,
}

impl Store {
  pub fn new(alias: String, store: Box<dyn Silo>) -> Store {
    Store { store, alias }
  }
}

impl Clone for Store {
  fn clone(&self) -> Store {
    Store {
      alias: self.alias.clone(),
      store: self.store.vclone(),
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
