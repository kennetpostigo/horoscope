use async_trait::async_trait;
use chrono::Utc;
use serde::{Serialize, Deserialize}; 

use crate::trigger;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trigger {
  alias: String,
  attempts: i32,
  run: i32,
}

#[async_trait]
#[typetag::serde]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    if (&self.attempts < &self.run) {
      self.attempts = self.attempts + 1;
      true
    } else {
      false
    }
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
