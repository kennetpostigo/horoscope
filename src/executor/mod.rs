use crate::job::{Status, Work};
use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Executor {
  pub alias: String,
}

impl Executor {
  pub fn new(alias: String) -> Executor {
    Executor { alias }
  }

  pub fn startup(&self) -> Result<(), String> {
    println!(
      "{}{}{}",
      "::::   Starting Executor ".truecolor(0,0,0).bold().on_green(),
      self.alias.truecolor(0,0,0).bold().on_green(),
      "   ::::".truecolor(0,0,0).bold().on_green()
    );
    Ok(())
  }

  pub async fn execute(&self, job: &Box<dyn Work>) -> Result<(), String> {
    match job.func().await {
      Status::Waiting => Ok(()),
      Status::Success => Ok(()),
      Status::Running => Ok(()),
      Status::Paused => Ok(()),
      Status::Failure(reason) => Err(reason),
    }
  }

  pub fn teardown(&self) -> Result<(), String> {
    println!(
      "{}{}{}",
      "::::   Tearing Down Executor ".truecolor(0,0,0).bold().on_green(),
      self.alias.truecolor(0,0,0).bold().on_green(),
      "   ::::".truecolor(0,0,0).bold().on_green()
    );
    Ok(())
  }
}
