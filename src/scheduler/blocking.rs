use async_channel::{Receiver, Sender};
use async_trait::async_trait;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;

// use crate::event::Event;
use crate::executor::Executor;
use crate::job::Work;
use crate::scheduler::{Msg, Schedule, SchedulerState};
use crate::store::{Silo, Store};

// type Listener = Box<dyn Fn(Event) -> ()>;

#[derive(Clone, Debug)]
pub struct Scheduler {
  pub state: SchedulerState,
  pub stores: HashMap<String, Store>,
  pub executors: HashMap<String, Executor>,
  // pub listeners: HashMap<String, Box<dyn Fn(Event) -> ()>>,
}

impl Scheduler {
  pub fn new() -> Self {
    Scheduler {
      state: SchedulerState::Uninitialized,
      stores: HashMap::new(),
      executors: HashMap::new(),
      // listeners: Box::new(HashMap::new()),
    }
  }
}

#[async_trait]
impl Schedule for Scheduler {
  fn startup(&mut self) {
    println!(":: Scheduler starting up ::");
    self.state = SchedulerState::Running;
    println!("State of the scheduler {}", &self.state);
  }

  fn proxy(
    &mut self,
    msg: Msg,
    _sender: &Sender<Msg>,
    _reader: &Receiver<Msg>,
  ) {
    println!("Msg came in");
    match msg {
      Msg::AddExecutor(alias, exctr) => match self.add_executor(alias, exctr) {
        Ok(_) => println!("Add Executor was fine"),
        Err(e) => println!("{}", e),
      },
      Msg::RemoveExecutor(alias) => match self.remove_executor(&alias) {
        Ok(_) => println!("Remove Executor was fine"),
        Err(e) => println!("{}", e),
      },
      Msg::AddStore(alias, store) => match self.add_store(alias, store) {
        Ok(_) => println!("Add Store was fine"),
        Err(e) => println!("{}", e),
      },
      // TODO: Implement Modify Store
      // Msg::ModifyStore(alias, properties) => scheduler.modify_store(alias, properties),
      Msg::RemoveStore(alias) => match self.remove_store(&alias) {
        Ok(_) => println!("Remove Store was fine"),
        Err(e) => println!("{}", e),
      },
      Msg::AddJob(alias, store_alias, executor, start_time, end_time, job) => {
        match self.add_job(
          alias,
          store_alias,
          executor,
          start_time,
          end_time,
          job,
        ) {
          Ok(_) => println!("Addd Job was fine"),
          Err(e) => println!("{}", e),
        }
      }
      // TODO: Implement ModifyJob
      Msg::ModifyJob(alias, store_alias) => {
        match self.modify_job(alias, store_alias) {
          Ok(_) => println!("Modify Job was fine"),
          Err(e) => println!("{}", e),
        }
      }
      Msg::RemoveJob(alias, store_alias) => {
        match self.remove_job(alias, store_alias) {
          Ok(_) => println!("Remove Job was fine"),
          Err(e) => println!("{}", e),
        }
      }
      // TODO: Implement Pause Job
      Msg::PauseJob(alias, store_alias) => {
        match self.pause_job(alias, store_alias) {
          Ok(_) => println!("Pause Job was fine"),
          Err(e) => println!("{}", e),
        }
      }
      // TODO: Implement Resume Job
      Msg::ResumeJob(alias, store_alias) => {
        match self.resume_job(alias, store_alias) {
          Ok(_) => println!("Resume Job was fine"),
          Err(e) => println!("{}", e),
        }
      }
      Msg::Log(id, _status, _result) => {
        println!("Hello {}", id);
      }
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
                  Ok(_v) => println!("execute job v: {}", &to_execute.alias),
                  Err(e) => println!("execute job e: {}", e),
                };

                // for listener in &self.listeners {
                //     listener.set("job id", "job status", "job event");
                // }

                // TODO: Check next only for timetrigger update
                // start_time. So check the option that is
                //returned to update start_time and delete if None

                match value.store.remove_job(&to_execute.alias) {
                  Ok(_v) => println!("remove job v: {}", &to_execute.alias),
                  Err(e) => println!("remove job e: {}", e),
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

  fn add_store(
    &mut self,
    alias: String,
    store: Box<dyn Silo>,
  ) -> Result<(), String> {
    let mut store = Store::new(alias.clone(), store);

    match store.store.start() {
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
    start_time: u128,
    end_time: Option<u128>,
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
