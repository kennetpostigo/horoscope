mod event;
mod executor;
mod job;
mod job_store;
mod scheduler;

use crate::executor::Executor;
use crate::job_store::memory::JobStore;
use crate::scheduler::blocking::Scheduler;
use crate::job::network::{NetworkJob, NetType};
use std::time::{SystemTime, UNIX_EPOCH};

#[async_std::main]
async fn main() {
    let start_time = {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");
        let delay: u128 = 30000;
        let start_time = now.as_millis() + delay;
        start_time
    };

    let store = JobStore::new(String::from("jobStore-test"));
    let exec = Executor::new(String::from("executor-test"));
    let njob = NetworkJob::new(
        String::from("job-1"),
        String::from("https://ping.me/"),
        NetType::Get,
        None,
    );

    let mut scheduler = Scheduler::new();
    scheduler.add_job_store(store, String::from("jobStore-test"));
    scheduler.add_executor(exec, String::from("executor-test"));
    scheduler.add_job(
        String::from("jobStore-test"),
        String::from("job-1"),
        Box::new(njob),
        String::from("executor-test"),
        0,
        0,
        start_time,
    );
    
    scheduler.start().await;
}
