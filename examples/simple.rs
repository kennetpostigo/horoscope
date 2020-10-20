use async_std::task;
use chrono::prelude::*;
use std::time::Duration;

use horoscope::executor::Executor;
use horoscope::job::network::{Job, NetType};
use horoscope::logger::Logger;
use horoscope::scheduler::{blocking, daemon, Msg, Schedule};
use horoscope::store::Store;

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

  blk_scheduler.load_snapshot_from_disk();

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
    .add_store(String::from("jobStore-test"), store)
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

  let njob2 = Job::new(
    String::from("job-2"),
    String::from("https://ping.me/"),
    NetType::Get,
    None,
    None,
  );

  task::sleep(Duration::from_secs(3)).await;

  sender.send(Msg::Snapshot).await.unwrap();

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
