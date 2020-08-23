use crate::event::Event;
use crate::executor::Executor;
use crate::job::{Status, Work};
use crate::job_store::memory_job_store::JobStore;
use std::collections::HashMap;
use std::sync::Arc;

pub enum SchedulerState {
    Uninitialized,
    Running,
    Stopped,
}

pub struct Scheduler<T: Work> {
    pub job_stores: HashMap<String, JobStore<T>>,
    pub executors: HashMap<String, Executor>,
    pub listeners: Vec<Arc<Fn(Event) -> ()>>,
    pub state: SchedulerState,
}

impl<T: Work> Scheduler<T> {
    pub fn new() -> Scheduler<T> {
        println!(":: Scheduler starting up ::");
        Scheduler {
            executors: HashMap::new(),
            job_stores: HashMap::new(),
            listeners: vec![],
            state: SchedulerState::Uninitialized,
        }
    }

    pub async fn start(&mut self) {
        loop {
            for (_key, value) in &mut self.job_stores {
                let ready = value.get_due_jobs();
                for to_execute in &ready {
                    let executioner = self.executors.get(&to_execute.executor);
                    match executioner {
                        None => Status::Failure,
                        Some(e) => e.execute(&to_execute.job).await
                    };
                }
            }
        }
    }

    pub fn add_job_store(&mut self, mut job_store: JobStore<T>, alias: String) {
        job_store.start();
        self.job_stores.entry(alias).or_insert(job_store);
    }

    pub fn add_job(&mut self, alias: String, job: T, executor: String, job_alias: String) {
        let store = self.job_stores.get_mut(&job_alias);
        match store {
            Some(r) => {
                r.add_job(job, alias, executor);
                return;
            }
            None => println!("Nothing"),
        }
    }

    pub fn add_executor(&mut self, executor: Executor, alias: String) {
        executor.start();
        self.executors.entry(alias).or_insert(executor);
    }

    pub fn remove_job_store(&mut self, alias: &String) {
        self.job_stores.remove(alias);
        return;
    }

    pub fn remove_executor(&mut self, alias: &String) {
        self.executors.remove(alias);
        return;
    }
}
