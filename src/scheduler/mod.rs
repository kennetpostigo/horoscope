pub mod blocking;

use async_channel;
use async_channel::{Receiver, Sender};
use async_std::prelude::*;
use async_std::stream;
use async_std::task;
use async_trait::async_trait;
use chrono::prelude::*;
use futures::{select, FutureExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

use crate::executor::Executor;
use crate::job::Work;
use crate::store::Store;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SchedulerState {
  Uninitialized,
  Running,
  Stopped,
}

impl fmt::Display for SchedulerState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      SchedulerState::Uninitialized => write!(f, "Scheduler::Uninitialized"),
      SchedulerState::Running => write!(f, "Scheduler::Running"),
      SchedulerState::Stopped => write!(f, "Scheduler::Stopped"),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
  // Scheduler Messages
  // ------------------------------------------------------------------------
  LoadFromDisk,
  LoadFromSnapshot(Vec<u8>),
  Snapshot,
  // Executor Msgs:
  AddExecutor(String, Executor),
  RemoveExecutor(String),

  // Store Msgs
  AddStore(String, Store),
  // ModifyStore(String, String),
  RemoveStore(String),

  // Job Msgs
  AddJob(String, String, String, i64, Option<i64>, Box<dyn Work>),
  ModifyJob(String, String),
  RemoveJob(String, String),
  PauseJob(String, String),
  ResumeJob(String, String),

  // Listener Msgs
  // AddListener(String, String, String),
  // RemoveListener(String, String, String),

  // User Messages
  // ------------------------------------------------------------------------
  // Common:
  Log(String, String, String),
}

pub fn get_elapsed_time(start_time: i64) {
  let now = Utc::now().timestamp_nanos();
  println!("{}", now - start_time * 1000000);
}

pub fn daemon(
  scheduler: Box<dyn Schedule>,
  save_state: bool,
) -> (Sender<Msg>, Receiver<Msg>) {
  let mut schdlr = scheduler;
  let (s, r) = async_channel::unbounded();
  let (s_cpy, r_cpy) = (s.clone(), r.clone());
  let mut interval = stream::interval(Duration::from_millis(5));

  task::spawn(async move {
    let (sender, reader) = (s_cpy, r_cpy);
    schdlr.startup();
    loop {
      select! {
          m = reader.recv().fuse() => {
              match m {
                  Ok(msg) => schdlr.proxy(msg, &sender, &reader).await,
                  Err(e) => println!("{}", e)
              }
          },
          i = interval.next().fuse() => {
              match i {
                  Some(_) => {
                    schdlr.check_jobs().await;
                    if (schdlr.is_dirty()) {
                      if (save_state){
                        println!("Saving snapshot");
                        schdlr.save_snapshot();
                      }
                      schdlr.set_dirty(false);
                    }
                  },
                  None => println!("Nothing in interval hit")
              }
          }
      };
    }
  });
  (s, r)
}

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait Schedule: Send + Sync {
  async fn proxy(
    &mut self,
    msg: Msg,
    sender: &Sender<Msg>,
    reader: &Receiver<Msg>,
  );

  fn startup(&mut self);

  async fn check_jobs(&mut self);

  fn is_dirty(&self) -> bool;

  fn set_dirty(&mut self, next: bool);

  async fn add_store(
    &mut self,
    alias: String,
    store: Store,
  ) -> Result<(), String>;

  fn add_job(
    &mut self,
    alias: String,
    store_alias: String,
    executor: String,
    start_time: i64,
    end_time: Option<i64>,
    job: Box<dyn Work>,
  ) -> Result<(), String>;

  fn add_executor(
    &mut self,
    alias: String,
    executor: Executor,
  ) -> Result<(), String>;

  fn modify_job(
    &mut self,
    alias: String,
    store_alias: String,
  ) -> Result<(), String>;

  fn pause_job(
    &mut self,
    alias: String,
    store_alias: String,
  ) -> Result<(), String>;

  fn resume_job(
    &mut self,
    alias: String,
    store_alias: String,
  ) -> Result<(), String>;

  fn remove_store(&mut self, alias: &String) -> Result<(), String>;

  fn remove_job(
    &mut self,
    alias: String,
    job_alias: String,
  ) -> Result<(), String>;

  fn remove_executor(&mut self, alias: &String) -> Result<(), String>;

  fn create_snapshot(&mut self) -> Vec<u8>;

  fn save_snapshot(&mut self);

  fn load_snapshot_from_disk(&mut self);

  fn load_snapshot_from_mem(&mut self, snapshot: Vec<u8>);

  fn vclone(&self) -> Box<dyn Schedule>;
}
