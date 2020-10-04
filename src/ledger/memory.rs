use crate::job::Status;
use crate::ledger::History;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ledger {
  data: HashMap<
    String,
    HashMap<String, HashMap<String, Vec<(String, String, Status, i64)>>>,
  >,
  ts: Vec<(String, String, Status, i64)>,
}

impl Ledger {
  pub fn new() -> Self {
    Ledger {
      data: HashMap::new(),
      ts: vec![],
    }
  }
}

impl History for Ledger {
  fn insert(
    &mut self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) {
    self
      .ts
      .push((store.clone(), job.clone(), status.clone(), time.clone()));
    match self.data.entry(store.clone()) {
      Entry::Occupied(mut sentry) => {
        let store_map = sentry.get_mut();
        match store_map.entry(job.clone()) {
          Entry::Occupied(mut jentry) => {
            let job_map = jentry.get_mut();
            match job_map.entry(status.to_string()) {
              Entry::Occupied(mut entry) => {
                let entries = entry.get_mut();
                entries.push((
                  store.clone(),
                  job.clone(),
                  status.clone(),
                  time.clone(),
                ));
              }
              Entry::Vacant(entry) => {
                entry.insert(vec![(
                  store.clone(),
                  job.clone(),
                  status.clone(),
                  time.clone(),
                )]);
              }
            }
          }
          Entry::Vacant(entry) => {
            let mut status_map = HashMap::new();
            status_map.insert(
              status.to_string(),
              vec![(store.clone(), job.clone(), status.clone(), time.clone())],
            );
            entry.insert(status_map);
          }
        }
      }
      Entry::Vacant(entry) => {
        let mut job_map = HashMap::new();
        let mut status_map = HashMap::new();
        status_map.insert(
          status.to_string(),
          vec![(store.clone(), job.clone(), status.clone(), time.clone())],
        );
        job_map.insert(job.clone(), status_map);
        entry.insert(job_map);
      }
    }
  }

  fn entry(
    &mut self,
    store: &String,
    job: &String,
    status: &Status,
    time: &i64,
  ) -> bool {
    let now = Utc::now().timestamp_nanos();
    match self.data.entry(store.clone()) {
      Entry::Occupied(mut sentry) => {
        let store_map = sentry.get_mut();
        match store_map.entry(job.clone()) {
          Entry::Occupied(mut jentry) => {
            let job_map = jentry.get_mut();
            match job_map.entry(status.to_string()) {
              Entry::Occupied(entry) => {
                let entries = entry.get();
                let mut contains = false;
                for (_s, _j, _sus, t) in entries {
                  if (now - t) <= time.clone() {
                    contains = true;
                    break;
                  }
                }
                contains
              }
              Entry::Vacant(_entry) => false,
            }
          }
          Entry::Vacant(_entry) => false,
        }
      }
      Entry::Vacant(_entry) => false,
    }
  }

  fn vclone(&self) -> Box<dyn History> {
    Box::new(self.clone())
  }
}
