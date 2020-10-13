use crate::trigger;
use async_trait::async_trait;
use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize,Clone, Debug)]
struct Trigger {
  left: trigger::Trigger,
  right: trigger::Trigger,
}

#[async_trait]
#[typetag::serde]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    self.left.trigger.should_run().await
      || self.right.trigger.should_run().await
  }

  async fn next(&mut self) -> Option<i64> {
    None
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
