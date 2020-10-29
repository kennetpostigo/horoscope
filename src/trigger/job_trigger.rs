use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::job::Status;
use crate::ledger::Ledger;
use crate::trigger;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trigger {
  alias: String,
  job: String,
  store: String,
  status: Status,
  time: i64,
}

impl Trigger {
  pub fn new(
    alias: String,
    job: String,
    store: String,
    status: Status,
    time: i64,
  ) -> Self {
    Trigger {
      alias,
      job,
      store,
      status,
      time,
    }
  }
}

#[async_trait]
#[typetag::serde(name = "JobTrigger")]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    panic!("trigger::job_trigger - REQUIRES SHOULD_RUN")
  }

  async fn should_run_with_ledger(&mut self, ledger: &mut Ledger) -> bool {
    match ledger
      .ledger
      .entry(&self.store, &self.job, &self.status, &self.time)
    {
      true => true,
      false => false,
    }
  }

  async fn next(&mut self) -> Option<i64> {
    None
  }

  fn needs_ledger(&self) -> bool {
      true
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
