use crate::job::Status;
use crate::ledger::Ledger;
use crate::trigger;
use async_trait::async_trait;

#[derive(Clone, Debug)]
struct Trigger {
  job: String,
  store: String,
  status: Status,
  time: i64,
  ledger: Ledger,
}

#[async_trait]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    match self.ledger.ledger.entry(
      &self.store,
      &self.job,
      &self.status,
      &self.time,
    ) {
      true => true,
      false => false,
    }
  }

  async fn next(&mut self) -> Option<i64> {
    None
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
