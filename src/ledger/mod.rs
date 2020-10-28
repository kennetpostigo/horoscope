pub mod memory;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::job::Status;

#[derive(Serialize, Deserialize)]
pub struct Ledger {
  pub alias: String,
  pub ledger: Box<dyn History>,
}

impl Ledger {
  pub fn new(alias: String, ledger: Box<dyn History>) -> Self {
    Ledger { alias, ledger }
  }
}

#[typetag::serde(tag = "type")]
pub trait History: Send + Sync {
  fn insert(
    &mut self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  );

  fn entry(
    &mut self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> bool;

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

// Bincode/Sled scraps
// use serde::{Deserialize, Serialize};
// use sled::Db;

// let db = sled::open("/quick_scope");
// pub fn serialize<T: Serialize>(value: &T, db = &mut sled::Db) -> Result<Vec<u8>> {
//    let val = bincode::options()
//         .big_endian()
//         .serialize(&value)
//         .map_err(|e| e.into());

//   db.insert(key, value);
// }

// pub fn deserialize<'a, T: Deserialize<'a>>(value: &'a [u8]) -> Result<T> {
//     bincode::options()
//         .big_endian()
//         .deserialize(&value)
//         .map_err(|e| e.into())
// }
