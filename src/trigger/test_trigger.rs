use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::ledger::Ledger;
use crate::trigger;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trigger {
  alias: String,
  should: bool,
  next: Option<i64>,
}

impl Trigger {
  pub fn new(alias: String, should: bool, next: Option<i64>) -> Self {
    Trigger {
      alias,
      should,
      next,
    }
  }
}

#[async_trait]
#[typetag::serde(name = "TestTrigger")]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    self.should
  }

  async fn should_run_with_ledger(&mut self, _ledger: &mut Ledger) -> bool {
    panic!("trigger::test_trigger - DOES NOT REQUIRE LEDGER")
  }

  async fn next(&mut self) -> Option<i64> {
    self.next
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
