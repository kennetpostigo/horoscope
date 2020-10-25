use crate::trigger;
use async_trait::async_trait;
use serde::{Serialize, Deserialize}; 
use chrono::prelude::*;

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct Trigger {
  alias: String,
  should: bool,
  next: Option<i64>
}

impl Trigger {
  pub fn new(alias: String, should: bool, next: Option<i64>) -> Self {
    Trigger { alias, should, next }
  }
}

#[async_trait]
#[typetag::serde]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    self.should
  }

  async fn next(&mut self) -> Option<i64> {
    self.next
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
