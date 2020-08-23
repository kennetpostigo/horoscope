use crate::event::Event;
use crate::executor::Executor;
use crate::job::{Job, Status, Work};
use crate::job_store::memory_job_store::JobStore;
use std::collections::HashMap;
use std::sync::Arc;

pub enum SchedulerState {
    Uninitialized,
    Running,
    Stopped,
}

pub struct Scheduler<T: Work> {
    pub job_stores: HashMap<String, Arc<JobStore<T>>>,
    pub executors: HashMap<String, Executor>,
    pub listeners: Vec<Box<Fn(Event) -> ()>>,
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

    pub async fn start(&self) {
        println!("Looping");
        loop {
            for (key, value) in &self.job_stores {
                match Arc::try_unwrap(value.clone()) {
                    Ok(mut j) => {
                        let ready = j.getDueJobs();
                        for lilJ in &ready {
                            let executioner: Option<&Executor> = self.executors.get(&lilJ.executor);
                            match executioner {
                                Some(e) => {
                                    println!("SOME SUCCESSS");
                                    e.execute(&lilJ.job).await; 
                                },
                                None => {
                                    println!("FAILURE");
                                    Status::Failure;
                                }
                            }
                        } 
                    }
                    Err(e) => {
                        println!("Couldn't unwrap Arc job_store: \n{}", e.alias)
                    }
                }

            }
        }
    }

    pub fn add_job_store(&mut self, mut job_store: JobStore<T>, alias: String) {
        job_store.start();
        self.job_stores.entry(alias).or_insert(Arc::new(job_store));
    }

    pub fn add_job(&mut self, alias: String, job: T, executor: String, job_alias: String) {
        let store = self.job_stores.get(&job_alias);
        let res = match store {
            Some(r) => {
                match Arc::try_unwrap(r.clone()) {
                    Ok(ref mut m) => {
                        m.add_job(job, alias, executor);
                        return;
                    }
                    _ => return,
                }
            },
            None => println!("Nothing")
        };


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
