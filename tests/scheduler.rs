use async_std::task;
use chrono::prelude::*;
use k9::assert_equal;
use std::collections::HashMap;
use std::time::Duration;

use horoscope::executor::Executor;
use horoscope::job::{sys::Job, Status};
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
    assert_equal!(schdlr.remove_store(&format!("store")), Ok(()));
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
fn scheduler_remove_executor() {
  task::block_on(async {
    let logger = Logger::new(true, vec![]);
    let mut schdlr =
      blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));
    schdlr.startup();

    let exec = Executor::new(String::from("executor-test"));
    let store = Store::new(String::from("jobStore-test"));

    schdlr.add_store(format!("store"), store).await.unwrap();
    schdlr.add_executor(format!("exec"), exec).unwrap();

    assert_equal!(schdlr.remove_executor(&format!("exec")), Ok(()));
    assert_equal!(
      schdlr.remove_executor(&format!("exec-nope")),
      Err(format!(
        "Executor exec-nope was not found in the schedulers executors"
      ))
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

#[test]
fn scheduler_remove_job() {
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

    assert_equal!(schdlr.remove_job(format!("job"), format!("store"),), Ok(()));

    assert_equal!(
      schdlr.remove_job(format!("job"), format!("store"),),
      Err(format!("Job job was not found in the Store jobStore-test"))
    );
  })
}

#[test]
fn scheduler_pause_job() {
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

    assert_equal!(schdlr.pause_job(format!("job"), format!("store")), Ok(()));

    assert_equal!(
      schdlr.pause_job(format!("job1"), format!("store"),),
      Err(format!(
        "Failed to Pause Job job1, it\'s not found in Store jobStore-test"
      ))
    );

    assert_equal!(
      schdlr.pause_job(format!("job"), format!("store1"),),
      Err(format!("Store store1 was not found in stores"))
    );
  })
}

#[test]
fn scheduler_resume_job() {
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

    schdlr.pause_job(format!("job"), format!("store")).unwrap();

    assert_equal!(schdlr.resume_job(format!("job"), format!("store")), Ok(()));
    assert_equal!(
      schdlr.resume_job(format!("job1"), format!("store")),
      Err(format!(
        "Failed to Resume Job job1, it\'s not found in Store jobStore-test"
      ))
    );

    assert_equal!(
      schdlr.resume_job(format!("job"), format!("store1")),
      Err(format!("Store store1 was not found in stores"))
    );
  })
}

#[test]
fn scheduler_proxy() {
  task::block_on(async {
    let (w, r) = async_channel::unbounded();
    let logger = Logger::new(true, vec![]);
    let mut schdlr =
      blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));
    schdlr.startup();

    let exec = Executor::new(String::from("executor"));
    let store = Store::new(String::from("store"));
    let job = Job::new(format!("job"), format!("echo"), vec![format!("lol")]);

    schdlr
      .proxy(Msg::AddStore(format!("store"), store), &w, &r)
      .await;

    schdlr
      .proxy(Msg::AddExecutor(format!("executor"), exec), &w, &r)
      .await;

    schdlr
      .proxy(
        Msg::AddJob(
          format!("job"),
          format!("store"),
          format!("executor"),
          Utc::now().timestamp_nanos(),
          None,
          Box::new(job),
        ),
        &w,
        &r,
      )
      .await;

    assert_equal!(schdlr.stores.len(), 1);
    assert_equal!(schdlr.executors.len(), 1);
    assert_equal!(schdlr.stores.get("store").unwrap().jobs.len(), 1);

    schdlr
      .proxy(Msg::PauseJob(format!("job"), format!("store")), &w, &r)
      .await;

    assert_equal!(
      &schdlr
        .stores
        .get("store")
        .unwrap()
        .jobs
        .get("job")
        .unwrap()
        .state,
      &Status::Paused
    );

    schdlr
      .proxy(Msg::ResumeJob(format!("job"), format!("store")), &w, &r)
      .await;

    assert_equal!(
      &schdlr
        .stores
        .get("store")
        .unwrap()
        .jobs
        .get("job")
        .unwrap()
        .state,
      &Status::Running
    );

    schdlr
      .proxy(Msg::RemoveJob(format!("job"), format!("store")), &w, &r)
      .await;

    assert_equal!(schdlr.stores.get("store").unwrap().jobs.len(), 0);

    schdlr
      .proxy(Msg::RemoveExecutor(format!("executor")), &w, &r)
      .await;

    assert_equal!(schdlr.executors.len(), 0);

    schdlr
      .proxy(Msg::RemoveStore(format!("store")), &w, &r)
      .await;

    assert_equal!(schdlr.stores.len(), 0);
  })
}

#[test]
fn scheduler_check_jobs() {
  task::block_on(async {
    let start_time = Utc::now().timestamp_nanos() - 500000000000;

    let logger = Logger::new(true, vec![]);
    let mut schdlr =
      blocking::Scheduler::new(String::from("scheduler"), Some(logger));

    schdlr.load_snapshot_from_disk();

    let store = Store::new(String::from("store"));
    let exec = Executor::new(String::from("executor"));
    let job = Job::new(format!("job"), format!("echo"), vec![format!("test")]);
    let job2 = job.clone();

    schdlr
      .add_store(String::from("store"), store)
      .await
      .unwrap();
    schdlr.add_executor(String::from("executor"), exec).unwrap();
    schdlr
      .add_job(
        String::from("job"),
        String::from("store"),
        String::from("executor"),
        start_time,
        None,
        Box::new(job),
      )
      .unwrap();

    schdlr.check_jobs().await;

    assert_equal!(schdlr.stores.get("store").unwrap().jobs.len(), 0);

    schdlr
      .add_job(
        String::from("job2"),
        String::from("store"),
        String::from("executor"),
        start_time + 100000000000000000,
        None,
        Box::new(job2),
      )
      .unwrap();

    schdlr.check_jobs().await;

    assert_equal!(schdlr.stores.get("store").unwrap().jobs.len(), 1);
  })
}
