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
    let store2 = store.clone();

    assert_equal!(schdlr.add_store(format!("store"), store).await, Ok(()));
    assert_equal!(
      schdlr.add_store(format!("store"), store2).await,
      Err(format!("store alias store already exists in stores"))
    );
  })
}

#[test]
fn scheduler_remove_store() {
  task::block_on(async {
    let logger = Logger::new(true, vec![]);
    let mut schdlr =
      blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));
    schdlr.startup();

    let store = Store::new(String::from("jobStore-test"));

    assert_equal!(schdlr.add_store(format!("store"), store).await, Ok(()));
    assert_equal!(
      schdlr.remove_store(&format!("store")),
      Ok(())
    );
    assert_equal!(
      schdlr.remove_store(&format!("store")),
      Err(format!("Store was not found"))
    );
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
    let exec2 = exec.clone();
    let store = Store::new(String::from("jobStore-test"));

    schdlr.add_store(format!("store"), store).await.unwrap();

    assert_equal!(schdlr.add_executor(format!("exec"), exec), Ok(()));
    assert_equal!(
      schdlr.add_executor(format!("exec"), exec2),
      Err(format!("Executor alias exec already exists"))
    );
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
    let job2 = job.clone();

    schdlr.add_store(format!("store"), store).await.unwrap();
    schdlr.add_executor(format!("exec"), exec).unwrap();

    assert_equal!(
      schdlr.add_job(
        format!("job"),
        format!("store"),
        format!("exec"),
        Utc::now().timestamp_nanos(),
        None,
        Box::new(job),
      ),
      Ok(())
    );

    assert_equal!(
      schdlr.add_job(
        format!("job"),
        format!("store-1"),
        format!("exec"),
        Utc::now().timestamp_nanos(),
        None,
        Box::new(job2),
      ),
      Err(format!("Store store-1 is not found in stores"))
    );
  })
}
