pub mod memory;

use std::fmt::Debug;

use crate::job::Status;

pub struct Ledger {
  pub alias: String,
  pub ledger: Box<dyn History>,
}

pub trait History
where
  Self: Send + Sync, {
  fn insert(
    &self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> Result<(), String>;

  fn remove(
    &self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> Result<(), String>;

  fn entry(
    &self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> Result<Option<(String, i64)>, String>;

  fn vclone(&self) -> Box<dyn History>;
}

impl Clone for Ledger {
  fn clone(&self) -> Self {
    Ledger {
      alias: self.alias.clone(),
      ledger: self.ledger.vclone(),
    }
  }
}

impl Debug for Ledger {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Ledger")
      .field("alias", &self.alias)
      .field("history", &"<history>")
      .finish()
  }
}
