use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::ledger::Ledger;
use crate::trigger;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trigger {
  alias: String,
  left: trigger::Trigger,
  right: trigger::Trigger,
}

impl Trigger {
  pub fn new(
    alias: String,
    left: trigger::Trigger,
    right: trigger::Trigger,
  ) -> Self {
    Trigger { alias, left, right }
  }
}

#[async_trait]
#[typetag::serde(name = "OrTrigger")]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    self.left.trigger.should_run().await
      || self.right.trigger.should_run().await
  }

  async fn should_run_with_ledger(&mut self, _ledger: &mut Ledger) -> bool {
    panic!("trigger::or_trigger - DOES NOT REQUIRE LEDGER")
  }

  async fn next(&mut self) -> Option<i64> {
    None
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
