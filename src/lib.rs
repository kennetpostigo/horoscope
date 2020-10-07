pub mod event;
pub mod executor;
pub mod job;
pub mod ledger;
pub mod logger;
pub mod scheduler;
pub mod store;
pub mod trigger;

use async_std::task;
use chrono::prelude::*;
use std::time::Duration;

use crate::executor::Executor;
use crate::job::network::{Job, NetType};
use crate::logger::Logger;
use crate::scheduler::{blocking, daemon, Msg, Schedule};
use crate::store::memory::Store;

#[async_std::main]
async fn main() {
  let start_time = {
    let now = Utc::now().timestamp_nanos();
    let delay: i64 = 10000000000;
    let start_time = now + delay;
    // println!("{}\n{}", now, now + delay);
    start_time
  };

  let logger = Logger::new(true, vec![]);
  let mut blk_scheduler =
    blocking::Scheduler::new(String::from("blk_scheduler"), Some(logger));

  let store = Store::new(String::from("jobStore-test"));
  let exec = Executor::new(String::from("executor-test"));
  let njob = Job::new(
    String::from("job-1"),
    String::from("https://ping.me/"),
    NetType::Get,
    None,
    None,
  );

  blk_scheduler
    .add_store(String::from("jobStore-test"), Box::new(store))
    .await
    .unwrap();
  blk_scheduler
    .add_executor(String::from("executor-test"), exec)
    .unwrap();
  blk_scheduler
    .add_job(
      String::from("job-1"),
      String::from("jobStore-test"),
      String::from("executor-test"),
      start_time,
      None,
      Box::new(njob),
    )
    .unwrap();

  let (sender, _reader) = daemon(Box::new(blk_scheduler));

  // sender.send(Msg::AddExecutor(String::from("trigger"), Box::new(exec)));

  match sender
    .send(Msg::Log(
      String::from("some id"),
      String::from("some status"),
      String::from("some result"),
    ))
    .await
  {
    Ok(_u) => (),
    Err(_err) => (),
  };

  // sender
  //     .send(Msg::AddStore(
  //         String::from("jobStore-test"),
  //         Box::new(store),
  //     ))
  //     .await
  //     .unwrap();
  // sender
  //     .send(Msg::AddExecutor(String::from("executor-test"), exec))
  //     .await
  //     .unwrap();

  let njob2 = Job::new(
    String::from("job-2"),
    String::from("https://ping.me/"),
    NetType::Get,
    None,
    None,
  );

  task::sleep(Duration::from_secs(3)).await;

  sender
    .send(Msg::AddJob(
      String::from("job-2"),
      String::from("jobStore-test"),
      String::from("executor-test"),
      start_time - 2000000000,
      None,
      Box::new(njob2),
    ))
    .await
    .unwrap();

  task::sleep(Duration::from_secs(300000)).await;
}
