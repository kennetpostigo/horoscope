use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

// use crate::event::Event;
use crate::executor::Executor;
use crate::job::Work;
use crate::ledger::{memory, Ledger};
use crate::logger::Logger;
use crate::scheduler::{Msg, Schedule, SchedulerState};
use crate::store::Store;
// type Listener = Box<dyn Fn(Event) -> ()>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scheduler {
  pub state: SchedulerState,
  pub ledger: Ledger,
  pub stores: HashMap<String, Store>,
  pub executors: HashMap<String, Executor>,
  pub logger: Option<Logger>,
  pub dirty: bool,
}

impl Scheduler {
  pub fn new(id: String, logger: Option<Logger>) -> Self {
    Scheduler {
      state: SchedulerState::Uninitialized,
      ledger: Ledger::new(id, Box::new(memory::Ledger::new())),
      stores: HashMap::new(),
      executors: HashMap::new(),
      logger,
      dirty: false,
    }
  }
}

#[async_trait]
#[typetag::serde]
impl Schedule for Scheduler {
  fn startup(&mut self) {
    println!(
      "{}",
      "::::   Scheduler Starting Up   ::::"
        .truecolor(0, 0, 0)
        .bold()
        .on_green()
    );
    self.state = SchedulerState::Running;
  }

  async fn proxy(
    &mut self,
    msg: Msg,
    _sender: &Sender<Msg>,
    _reader: &Receiver<Msg>,
  ) {
    match self.logger.clone() {
      Some(logger) => match msg {
        Msg::LoadFromDisk => self.load_snapshot_from_disk(),
        Msg::LoadFromSnapshot(snap) => self.load_snapshot_from_mem(snap),
        Msg::Snapshot => self.save_snapshot(),
        Msg::AddExecutor(alias, exctr) => match self
          .add_executor(alias.clone(), exctr)
        {
          Ok(_) => logger.info(format!("ADDING EXECUTER {} SUCCEEDED", &alias)),
          Err(e) => logger.err(format!("{}", e)),
        },
        Msg::RemoveExecutor(alias) => match self.remove_executor(&alias) {
          Ok(_) => {
            logger.info(format!("REMOVING EXECUTOR {} SUCCEEDED", &alias))
          }
          Err(e) => logger.err(format!("{}", e)),
        },
        Msg::AddStore(alias, store) => {
          match self.add_store(alias.clone(), store).await {
            Ok(_) => logger.info(format!("ADDING STORE {} SUCCEEDED", &alias)),
            Err(e) => logger.err(format!("{}", e)),
          }
        }
        // TODO: Implement Modify Store
        // Msg::ModifyStore(alias, properties) => scheduler.modify_store(alias, properties),
        Msg::RemoveStore(alias) => match self.remove_store(&alias) {
          Ok(_) => logger.info(format!("REMOVING STORE {} SUCCEEDED", &alias)),
          Err(e) => logger.err(format!("{}", e)),
        },
        Msg::AddJob(
          alias,
          store_alias,
          executor,
          start_time,
          end_time,
          job,
        ) => {
          match self.add_job(
            alias.clone(),
            store_alias.clone(),
            executor,
            start_time,
            end_time,
            job,
          ) {
            Ok(_) => logger.info(format!(
              "ADDING JOB {} TO STORE {} SUCCEEDED",
              &alias, &store_alias
            )),
            Err(e) => logger.err(format!("{}", e)),
          }
        }
        // TODO: Implement ModifyJob
        Msg::ModifyJob(alias, store_alias) => {
          match self.modify_job(alias.clone(), store_alias.clone()) {
            Ok(_) => logger.info(format!(
              "MODIFYING JOB {} IN STORE {} SUCCEEDED",
              &alias, &store_alias
            )),
            Err(e) => logger.err(format!("{}", e)),
          }
        }
        Msg::RemoveJob(alias, store_alias) => {
          match self.remove_job(alias.clone(), store_alias.clone()) {
            Ok(_) => logger.info(format!(
              "REMOVING JOB {} FROM STORE {} SUCCEEDED",
              &alias, &store_alias
            )),
            Err(e) => logger.err(format!("{}", e)),
          }
        }
        // TODO: Implement Pause Job
        Msg::PauseJob(alias, store_alias) => {
          match self.pause_job(alias.clone(), store_alias.clone()) {
            Ok(_) => logger.info(format!(
              "PAUSING JOB {} IN STORE {} SUCCEEDED",
              &alias, &store_alias
            )),
            Err(e) => logger.err(format!("{}", e)),
          }
        }
        // TODO: Implement Resume Job
        Msg::ResumeJob(alias, store_alias) => {
          match self.resume_job(alias.clone(), store_alias.clone()) {
            Ok(_) => logger.info(format!(
              "RESUMING JOB {} IN STORE {} SUCCEEDED",
              &alias, &store_alias,
            )),
            Err(e) => logger.err(e),
          }
        }
        Msg::Log(id, _status, _result) => logger.info(format!("LOG {}", id)),
      },
      None => (),
    };
  }

  async fn check_jobs(&mut self) {
    for (_key, value) in &mut self.stores {
      let cpy = &mut value.clone();
      match cpy.get_due_jobs() {
        Ok(ready) => {
          for to_execute in ready {
            let executioner = self.executors.get(&to_execute.executor);
            match executioner {
              None => (),
              Some(e) => {
                let (should_run, next) = to_execute.validate_triggers().await;
                if (should_run) {
                  match (e.execute(&to_execute.job).await) {
                    Ok(_v) => {
                      if let Some(logger) = &self.logger {
                        logger.info(format!(
                          "EXECUTING JOB {} FROM STORE {} SUCCEEDED",
                          &to_execute.alias,
                          &value.clone().alias
                        ))
                      }
                    }
                    Err(e) => {
                      if let Some(logger) = &self.logger {
                        logger.err(e)
                      }
                    }
                  };
                }

                if let Some(v) = next {
                  if (should_run) {
                    to_execute.start_time = v;
                    self.dirty = true;
                  }
                } else {
                  match value.remove_job(&to_execute.alias) {
                    Ok(_v) => {
                      if let Some(logger) = &self.logger {
                        logger.info(format!(
                          "REMOVING JOB {} FROM STORE {} SUCCEEDED",
                          &to_execute.alias,
                          &value.clone().alias
                        ));
                        self.dirty = true;
                      }
                    }
                    Err(e) => {
                      if let Some(logger) = &self.logger {
                        logger.err(e);
                        self.dirty = true;
                      }
                    }
                  };
                }
              }
            };
          }
        }
        Err(_e) => println!(
          "Failed to get jobs that are ready to execute for Store {}",
          &cpy.alias
        ),
      }
    }
  }

  fn is_dirty(&self) -> bool {
    self.dirty.clone()
  }

  fn set_dirty(&mut self, next: bool) {
    self.dirty = next;
  }

  async fn add_store(
    &mut self,
    alias: String,
    store: Store,
  ) -> Result<(), String> {
    let mut store = store;

    match store.startup().await {
      Ok(_) => match self.stores.entry(alias.clone()) {
        Entry::Occupied(_entry) => match store.teardown() {
          Ok(_) => {
            Err(format!("store alias {} already exists in stores", &alias))
          }
          Err(_e) => Err(format!(
            "Store alias {} started, and failed to insert and teardown",
            &alias
          )),
        },
        Entry::Vacant(entry) => {
          self.dirty = true;
          entry.insert(store);
          Ok(())
        }
      },
      Err(e) => Err(e),
    }
  }

  fn add_job(
    &mut self,
    alias: String,
    store_alias: String,
    executor: String,
    start_time: i64,
    end_time: Option<i64>,
    job: Box<dyn Work>,
  ) -> Result<(), String> {
    match self.stores.entry(store_alias.clone()) {
      Entry::Occupied(mut entry) => {
        let store = entry.get_mut();
        self.dirty = true;
        store.add_job(alias, executor, start_time, end_time, job)
      }
      Entry::Vacant(_entry) => {
        Err(format!("Store {} is not found in stores", &store_alias))
      }
    }
  }

  fn add_executor(
    &mut self,
    alias: String,
    executor: Executor,
  ) -> Result<(), String> {
    match executor.startup() {
      Ok(_) => match self.executors.entry(alias.clone()) {
        Entry::Occupied(_entry) => match executor.teardown() {
          Ok(_) => Err(format!("Executor alias {} already exists", &alias)),
          Err(_e) => Err(format!(
            "Executor alias {} started, and failed to insert and teardown",
            &alias
          )),
        },
        Entry::Vacant(entry) => {
          entry.insert(executor);
          self.dirty = true;
          Ok(())
        }
      },
      Err(e) => Err(e),
    }
  }

  fn remove_store(&mut self, alias: &String) -> Result<(), String> {
    match self.stores.entry(alias.clone()) {
      Entry::Occupied(mut entry) => {
        let store = entry.get_mut();
        match store.teardown() {
          Ok(_) => match self.stores.remove(alias) {
            Some(_) => {
              self.dirty = true;
              Ok(())
            }
            None => {
              Err(String::from("Failed to remove store from scheduler stores"))
            }
          },
          Err(e) => Err(e),
        }
      }
      Entry::Vacant(_entry) => Err(String::from("Store was not found")),
    }
  }

  fn modify_job(
    &mut self,
    alias: String,
    store_alias: String,
  ) -> Result<(), String> {
    match self.stores.entry(store_alias.clone()) {
      Entry::Occupied(mut entry) => {
        let store = entry.get_mut();
        self.dirty = true;
        store.modify_job(&alias)
      }
      Entry::Vacant(_entry) => {
        Err(format!("Store {} was not found in stores", &store_alias))
      }
    }
  }

  fn pause_job(
    &mut self,
    alias: String,
    store_alias: String,
  ) -> Result<(), String> {
    match self.stores.entry(store_alias.clone()) {
      Entry::Occupied(mut entry) => {
        let store = entry.get_mut();
        self.dirty = true;
        store.pause_job(alias)
      }
      Entry::Vacant(_entry) => {
        Err(format!("Store {} was not found in stores", &store_alias))
      }
    }
  }

  fn resume_job(
    &mut self,
    alias: String,
    store_alias: String,
  ) -> Result<(), String> {
    match self.stores.entry(store_alias.clone()) {
      Entry::Occupied(mut entry) => {
        let store = entry.get_mut();
        self.dirty = true;
        store.resume_job(alias)
      }
      Entry::Vacant(_entry) => {
        Err(format!("Store {} was not found in stores", &store_alias))
      }
    }
  }

  fn remove_job(
    &mut self,
    alias: String,
    store_alias: String,
  ) -> Result<(), String> {
    match self.stores.entry(store_alias.clone()) {
      Entry::Occupied(mut entry) => {
        let store = entry.get_mut();
        self.dirty = true;
        store.remove_job(&alias)
      }
      Entry::Vacant(_entry) => {
        Err(format!("Store {} was not found in stores", &store_alias))
      }
    }
  }

  fn remove_executor(&mut self, alias: &String) -> Result<(), String> {
    match self.executors.entry(alias.clone()) {
      Entry::Occupied(mut entry) => {
        let exctr = entry.get_mut();
        match exctr.teardown() {
          Ok(_) => match self.executors.remove(alias) {
            Some(_v) => {
              self.dirty = true;
              Ok(())
            }
            None => Err(format!(
              "Executor alias {} failed to remove the executor",
              &alias
            )),
          },
          Err(e) => Err(e),
        }
      }
      Entry::Vacant(_entry) => Err(format!(
        "Executor {} was not found in the schedulers executors",
        &alias
      )),
    }
  }

  fn create_snapshot(&mut self) -> Vec<u8> {
    bincode::serialize(&self).unwrap()
  }

  fn save_snapshot(&mut self) {
    let snap = self.create_snapshot();
    let db = sled::open("./horo").unwrap();
    db.insert("scope", snap).unwrap();
  }

  fn load_snapshot_from_disk(&mut self) {
    let db = sled::open("./horo").unwrap();
    let disk_snap = db.get("scope").unwrap();
    match disk_snap {
      Some(snap) => {
        let schdlr: bincode::Result<Scheduler> = bincode::deserialize(&snap);
        match schdlr {
          Ok(v) => {
            self.stores = v.stores;
            self.ledger = v.ledger;
            self.executors = v.executors;
            self.logger = v.logger;
          }
          Err(_) => {}
        }
      }
      None => {}
    }
  }

  fn load_snapshot_from_mem(&mut self, snapshot: Vec<u8>) {
    let schdlr: bincode::Result<Scheduler> = bincode::deserialize(&snapshot);
    match schdlr {
      Ok(v) => {
        self.stores = v.stores;
        self.ledger = v.ledger;
        self.executors = v.executors;
        self.logger = v.logger;
      }
      Err(_) => {}
    }
  }

  fn vclone(&self) -> Box<dyn Schedule> {
    Box::new(self.clone())
  }
}
