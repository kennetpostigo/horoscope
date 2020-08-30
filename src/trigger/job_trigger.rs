use async_trait::async_trait;

use crate::trigger;
use crate::ledger::Ledger;

#[derive(Clone, Debug)]
struct Trigger {
    job: String,
    ledger: Ledger
}

#[async_trait]
impl trigger::Fire for Trigger {
    async fn should_run(&self) -> bool {
      true
    }

    async fn next(&self) -> Option<u128> {
        Some(1)
    }

    fn vclone(&self) -> Box<dyn trigger::Fire> {
      Box::new(self.clone())
    }
}