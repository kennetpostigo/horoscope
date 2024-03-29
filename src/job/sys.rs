use async_process::Command;
use async_trait::async_trait;
use colored::*;
use serde::{Deserialize, Serialize};

use crate::job::{Status, Work};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Job {
  pub alias: String,
  pub script: String,
  pub args: Vec<String>,
}

impl Job {
  pub fn new(alias: String, script: String, args: Vec<String>) -> Self {
    Job {
      alias,
      script,
      args,
    }
  }
}

#[async_trait]
#[typetag::serde(name = "SystemJob")]
impl Work for Job {
  async fn startup(&self) -> Result<(), String> {
    println!(
      "{}{}{}",
      "::::   Starting Sys Job "
        .truecolor(0, 0, 0)
        .bold()
        .on_green(),
      self.alias.truecolor(0, 0, 0).bold().on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(())
  }

  async fn func(&self) -> Status {
    match Command::new(&self.script.clone())
      .args(&self.args)
      .output()
      .await
    {
      Ok(_) => Status::Success,
      Err(_e) => Status::Failure(format!(
        "Failed to successfully run {} with {:?}",
        &self.script, &self.args
      )),
    }
  }

  async fn teardown(&self) -> Result<(), String> {
    println!(
      "{}{}{}",
      "::::   Tearing Down Sys Job "
        .truecolor(0, 0, 0)
        .bold()
        .on_green(),
      self.alias.truecolor(0, 0, 0).bold().on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(())
  }

  fn vclone(&self) -> Box<dyn Work> {
    Box::new(self.clone())
  }
}
