pub mod event;
pub mod executor;
pub mod job;
pub mod scheduler;
pub mod store;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::executor::Executor;
use crate::job::network::{Job, NetType};
use crate::scheduler::{background, blocking};
use crate::store::memory::Store;
//TODO: Figure out how to get rid of this trait
use crate::scheduler::Schedule;

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
    blk_scheduler.add_store(Box::new(store), String::from("jobStore-test"));
    blk_scheduler.add_executor(exec, String::from("executor-test"));
    blk_scheduler.add_job(
        String::from("jobStore-test"),
        String::from("job-1"),
        Box::new(njob),
        String::from("executor-test"),
        0,
        0,
        start_time,
    );

    let mut bg_scheduler = background::Scheduler::new(Box::new(blk_scheduler));

    bg_scheduler.startup();

    println!("TEST");
}
