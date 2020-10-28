use async_std::task;
use chrono::prelude::*;
use k9::assert_equal;
use std::collections::HashMap;
use std::time::Duration;

use horoscope::executor::Executor;
use horoscope::job::sys::Job;
use horoscope::ledger::{memory, Ledger};
use horoscope::logger::Logger;
use horoscope::scheduler::{blocking, daemon, Msg, Schedule, SchedulerState};
use horoscope::store::Store;

#[test]
fn scheduler_creation() {
  let logger = Logger::new(true, vec![]);
  let schdlr =
    blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));

  assert_equal!(&schdlr.state, &SchedulerState::Uninitialized);
  // assert_equal!(&schdlr.ledger, &Ledger::new(id, Box::new(memory::Ledger::new())));
  assert_equal!(&schdlr.stores.len(), &0);
  assert_equal!(&schdlr.executors, &HashMap::new());
  assert_equal!(&schdlr.logger != &None, true);
  assert_equal!(&schdlr.dirty, &false);
}

#[test]
fn scheduler_startup() {
  let logger = Logger::new(true, vec![]);
  let mut schdlr =
    blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));

  assert_equal!(&schdlr.state, &SchedulerState::Uninitialized);
  schdlr.startup();
  assert_equal!(&schdlr.state, &SchedulerState::Running);
}

#[test]
fn scheduler_add_store() {
  task::block_on(async {
    let logger = Logger::new(true, vec![]);
    let mut schdlr =
      blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));
    schdlr.startup();

    let store = Store::new(String::from("jobStore-test"));
    schdlr.add_store(format!("store"), store).await.unwrap();

    assert_equal!(schdlr.stores.len(), 1);
  })
}

#[test]
fn scheduler_add_executor() {
  task::block_on(async {
    let logger = Logger::new(true, vec![]);
    let mut schdlr =
      blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));
    schdlr.startup();

    let exec = Executor::new(String::from("executor-test"));
    let store = Store::new(String::from("jobStore-test"));

    schdlr.add_store(format!("store"), store).await.unwrap();
    schdlr.add_executor(format!("exec"), exec).unwrap();

    assert_equal!(schdlr.executors.len(), 1);
  })
}

#[test]
fn scheduler_add_job() {
  task::block_on(async {
    let logger = Logger::new(true, vec![]);
    let mut schdlr =
      blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));
    schdlr.startup();

    let exec = Executor::new(String::from("executor-test"));
    let store = Store::new(String::from("jobStore-test"));
    let job = Job::new(format!("job"), format!("echo"), vec![format!("lol")]);

    schdlr.add_store(format!("store"), store).await.unwrap();
    schdlr.add_executor(format!("exec"), exec).unwrap();
    schdlr
      .add_job(
        format!("job"),
        format!("store"),
        format!("exec"),
        Utc::now().timestamp_nanos(),
        None,
        Box::new(job),
      )
      .unwrap();

    assert_equal!(schdlr.stores.get(&format!("store")).unwrap().jobs.len(), 1);
  })
}
