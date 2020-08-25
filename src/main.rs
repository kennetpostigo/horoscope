pub mod event;
pub mod executor;
pub mod job;
pub mod scheduler;
pub mod store;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::executor::Executor;
use crate::job::network::{NetType, NetworkJob};
use crate::scheduler::blocking::Scheduler;
use crate::scheduler::Schedule;
use crate::store::memory::MemoryStore;

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

    let store = MemoryStore::new(String::from("jobStore-test"));
    let exec = Executor::new(String::from("executor-test"));
    let njob = NetworkJob::new(
        String::from("job-1"),
        String::from("https://ping.me/"),
        NetType::Get,
        None,
    );

    let mut bscheduler = Scheduler::new();
    bscheduler.add_store(Box::new(store), String::from("jobStore-test"));
    bscheduler.add_executor(exec, String::from("executor-test"));
    bscheduler.add_job(
        String::from("jobStore-test"),
        String::from("job-1"),
        Box::new(njob),
        String::from("executor-test"),
        0,
        0,
        start_time,
    );

    bscheduler.startup().await;
    println!("TEST");
}
