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
  async fn should_run(&self) -> bool {
    match self.ledger.ledger.entry(
      &self.store,
      &self.job,
      &self.status,
      &self.time,
    ) {
      Ok(Some(_)) => true,
      Ok(None) => false,
      Err(_e) => false,
    }
  }

  async fn next(&self) -> Option<u128> {
    None
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
