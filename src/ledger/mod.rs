use std::fmt::Debug;

pub struct Ledger {
  alias: String,
  ledger: Box<dyn History>,
}

pub trait History where Self: Send + Sync {
  fn insert(&self, entry: (String, String)) -> Result<(), String>;
  fn remove(&self, entry: (String, String)) -> Result<(), String>;
  fn entry(&self, entry: (String, String)) -> Result<Option<String>, String>;
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
