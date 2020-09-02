use chrono::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::job::Status;
use crate::ledger::History;

#[derive(Clone, Debug)]
struct Ledger {
  data: HashMap<
    String,
    HashMap<String, HashMap<String, Vec<(String, String, Status, i64)>>>,
  >,
}

impl History for Ledger {
  fn insert(
    &self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> Result<(), String> {
    match self.data.entry(store.clone()) {
      Entry::Occupied(sentry) => {
        let store_map = sentry.get();
        match store_map.entry(job.clone()) {
          Entry::Occupied(jentry) => {
            let job_map = jentry.get();
            match job_map.entry(status.to_string()) {
              Entry::Occupied(entry) => {
                let entries = entry.get();
                entries.push((
                  store.clone(),
                  job.clone(),
                  status.clone(),
                  time.clone(),
                ));
                Ok(())
              }
              Entry::Vacant(_e) => Ok(()),
            }
          }
          Entry::Vacant(_) => Ok(()),
        }
      }
      Entry::Vacant(_entry) => Ok(()),
    }
  }

  fn remove(
    &self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> Result<(), String> {
  }

  fn entry(
    &self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> Result<Option<(String, i64)>, String> {
  }

  fn vclone(&self) -> Box<dyn History> {
    Box::new(self.clone())
  }
}
