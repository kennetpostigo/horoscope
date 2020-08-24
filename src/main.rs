mod event;
mod executor;
mod job;
mod job_store;
mod scheduler;

use crate::executor::Executor;
use crate::job::{Status, Work};
use crate::job_store::memory_job_store::JobStore;
use crate::scheduler::blocking_scheduler::Scheduler;
use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
enum NetType {
    Get,
    Post,
}

#[derive(Clone, Debug)]
struct NetworkJob<T: Send + Sync + Clone> {
    pub url: String,
    pub method: NetType,
    pub body: Option<T>,
}

#[async_trait]
impl<T: Sync + Send + Clone> Work for NetworkJob<T> {
    async fn func(&self) -> Status {
        match &self.method {
            NetType::Get => match surf::get(&self.url).recv_string().await {
                Ok(msg) => {
                    print!("{}", msg);
                    return Status::Success;
                }
                Err(_) => return Status::Failure,
            },
            NetType::Post => {
                let data = serde_json::json!({ "name": "chashu" });
                match http_types::Body::from_json(&data) {
                    Ok(bdy) => {
                        let res = surf::post(&self.url).body(bdy).await;

                        match res {
                            Ok(r) => {
                                println!("{}", r.status());
                                Status::Success
                            }
                            Err(_) => Status::Failure,
                        }
                    }
                    Err(_) => Status::Failure,
                }
            }
        }
    }
}

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

    let store: JobStore<NetworkJob<String>> = JobStore::new(String::from("jobStore-test"));
    let exec: Executor = Executor::new(String::from("executor-test"));
    let njob: NetworkJob<String> = NetworkJob {
        url: String::from("https://ping.me/"),
        method: NetType::Get,
        body: None,
    };

    let mut scheduler: Scheduler<NetworkJob<String>> = Scheduler::new();
    scheduler.add_job_store(store, String::from("jobStore-test"));
    scheduler.add_executor(exec, String::from("executor-test"));
    scheduler.add_job(
        String::from("jobStore-test"),
        String::from("job-1"),
        njob,
        String::from("executor-test"),
        0,
        0,
        start_time,
    );
    
    scheduler.start().await;
}
