use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use colored::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

// use crate::event::Event;
use crate::executor::Executor;
use crate::job::Work;
use crate::logger::Logger;
use crate::scheduler::{Msg, Schedule, SchedulerState};
use crate::store::{Silo, Store};
// type Listener = Box<dyn Fn(Event) -> ()>;

#[derive(Clone, Debug)]
pub struct Scheduler {
  pub state: SchedulerState,
  pub stores: HashMap<String, Store>,
  pub executors: HashMap<String, Executor>,
  pub logger: Option<Logger>,
  // pub listeners: HashMap<String, Box<dyn Fn(Event) -> ()>>,
}

impl Scheduler {
  pub fn new(logger: Option<Logger>) -> Self {
    Scheduler {
      state: SchedulerState::Uninitialized,
      stores: HashMap::new(),
      executors: HashMap::new(),
      logger, // listeners: Box::new(HashMap::new()),
    }
  }
}

#[async_trait]
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
      match cpy.store.get_due_jobs() {
        Ok(ready) => {
          for to_execute in ready {
            let executioner = self.executors.get(&to_execute.executor);
            match executioner {
              None => println!("NOTHING GOING ON"),
              Some(e) => {
                // Only when measuring:
                // get_elapsed_time(to_execute.start_time);
                // TODO: Check Triggers
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

                // for listener in &self.listeners {
                //     listener.set("job id", "job status", "job event");
                // }

                // TODO: Check next only for timetrigger update
                // start_time. So check the option that is
                //returned to update start_time and delete if None

                match value.store.remove_job(&to_execute.alias) {
                  Ok(_v) => {
                    if let Some(logger) = &self.logger {
                      logger.info(format!(
                        "REMOVING JOB {} FROM STORE {} SUCCEEDED",
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

  async fn add_store(
    &mut self,
    alias: String,
    store: Box<dyn Silo>,
  ) -> Result<(), String> {
    let mut store = Store::new(alias.clone(), store);

    match store.store.startup().await {
      Ok(_) => match self.stores.entry(alias.clone()) {
        Entry::Occupied(_entry) => match store.store.teardown() {
          Ok(_) => {
            Err(format!("store alias {} already exists in stores", &alias))
          }
          Err(_e) => Err(format!(
            "Store alias {} started, and failed to insert and teardown",
            &alias
          )),
        },
        Entry::Vacant(entry) => {
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
        store
          .store
          .add_job(alias, executor, start_time, end_time, job)
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
        match store.store.teardown() {
          Ok(_) => match self.stores.remove(alias) {
            Some(_) => Ok(()),
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
        store.store.modify_job(&alias)
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
        store.store.pause_job(alias)
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
        store.store.resume_job(alias)
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
        store.store.remove_job(&alias)
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
            Some(_v) => Ok(()),
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

  fn vclone(&self) -> Box<dyn Schedule> {
    Box::new(self.clone())
  }
}
