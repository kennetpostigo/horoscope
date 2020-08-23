mod event;
mod executor;
mod job;
mod job_store;
mod scheduler;

use crate::scheduler::blocking_scheduler::Scheduler;
use crate::job_store::memory_job_store::JobStore;
use crate::executor::Executor;
use crate::job::Job;
use async_trait::async_trait;

enum NetType {
    Get,
    Post
}

struct NetworkJob<T: Send + Sync> {
    pub url: String,
    pub method: NetType,
    pub body: Option<T>,
}

#[async_trait]
impl<T: Sync + Send> job::Work for NetworkJob<T> {
    async fn func(&self) -> job::Status {
        match &self.method {
            Get => {
                match surf::get(&self.url).recv_string().await {
                    Ok(msg) => {
                        print!("{}", msg);
                        return job::Status::Success;
                    }
                    Err(_) => return job::Status::Failure
                }
            } 
            Post => {
                 let data = serde_json::json!({ "name": "chashu" });
                 match http_types::Body::from_json(&data) {
                     Ok(bdy) => {
                        let res = surf::post(&self.url)
                        .body(bdy)
                        .await;

                        match res {
                            Ok(r) => {
                                println!("{}", r.status());
                                job::Status::Success
                            },
                            Err(_) => job::Status::Failure
                        }
                        
                     },
                     Err(_) => job::Status::Failure
                 }
                 
              }
        }
    }
}

#[async_std::main]
async fn main() {
    let store: JobStore<NetworkJob<String>> = JobStore::new(String::from("jobStore-test"));
    let exec: Executor = Executor::new(String::from("executor-test"));
    let njob: NetworkJob<String> = NetworkJob {  
        url: String::from("https://ping.me/"), 
        method: NetType::Get, 
        body: None 
    };

    let mut scheduler: Scheduler<NetworkJob<String>> = Scheduler::new();
    scheduler.add_job_store(store, String::from("jobStore-test"));
    scheduler.add_executor(exec, String::from("executor-test"));
    scheduler.add_job(String::from("job-1"), njob, String::from("executor-test"), String::from("jobStore-test"));
    scheduler.start().await;
}
