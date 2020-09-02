pub mod event;
pub mod executor;
pub mod job;
pub mod ledger;
pub mod scheduler;
pub mod store;
pub mod trigger;

use async_std::task;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::executor::Executor;
use crate::job::network::{Job, NetType};
use crate::scheduler::{blocking, daemon, Msg, Schedule};
use crate::store::memory::Store;

#[async_std::main]
async fn main() {
  let start_time = {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");
    let delay: u128 = 10000;
    let start_time = now.as_millis() + delay;
    start_time
  };

  let store = Store::new(String::from("jobStore-test"));
  let exec = Executor::new(String::from("executor-test"));
  let njob = Job::new(
    String::from("job-1"),
    String::from("https://ping.me/"),
    NetType::Get,
    None,
  );

  let mut blk_scheduler = blocking::Scheduler::new();

  blk_scheduler
    .add_store(String::from("jobStore-test"), Box::new(store))
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
    Ok(_u) => println!("Message Sent!"),
    Err(err) => println!("Err: {}", err),
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
  // sender
  //     .send(Msg::AddJob(
  //         String::from("job-1"),
  //         String::from("jobStore-test"),
  //         String::from("executor-test"),
  //         start_time,
  //         None,
  //         Box::new(njob),
  //     ))
  //     .await
  //     .unwrap();

  println!("TEST");
  task::sleep(Duration::from_secs(30)).await;
}
