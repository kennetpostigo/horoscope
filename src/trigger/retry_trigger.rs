use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::ledger::Ledger;
use crate::trigger;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trigger {
  alias: String,
  attempts: i32,
  run: i32,
}

impl Trigger {
  pub fn new(alias: String, run: i32) -> Self {
    Trigger {
      alias,
      attempts: 0,
      run,
    }
  }
}

#[async_trait]
#[typetag::serde(name = "RetryTrigger")]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    if (&self.attempts < &self.run) {
      self.attempts = self.attempts + 1;
      true
    } else {
      false
    }
  }

  async fn should_run_with_ledger(&mut self, _ledger: &mut Ledger) -> bool {
    panic!("trigger::retry_trigger - DOES NOT REQUIRE LEDGER")
  }

  async fn next(&mut self) -> Option<i64> {
    if (&self.attempts < &self.run) {
      Some(Utc::now().timestamp_nanos())
    } else {
      None
    }
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
