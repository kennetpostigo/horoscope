// use crate::event::Event;
use crate::executor::Executor;
use crate::job::Work;
use crate::job_store::memory_job_store::JobStore;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub enum SchedulerState {
    Uninitialized,
    Running,
    Stopped,
}

fn getElapsedTime(start_time: u128) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");

    println!("{}", now.as_nanos() - start_time * 1000000);
}

#[derive(Clone, Debug)]
pub struct Scheduler<T: Work + Clone> {
    pub job_stores: HashMap<String, JobStore<T>>,
    pub executors: HashMap<String, Executor>,
    // pub listeners: Vec<Arc<Fn(Event) -> ()>>,
    pub state: SchedulerState,
}

impl<T: Work + Clone> Scheduler<T> {
    pub fn new() -> Scheduler<T> {
        println!(":: Scheduler starting up ::");
        Scheduler {
            executors: HashMap::new(),
            job_stores: HashMap::new(),
            // listeners: vec![],
            state: SchedulerState::Uninitialized,
        }
    }

    pub async fn start(&mut self) {
        self.state = SchedulerState::Running;
        loop {
            for (_key, value) in &mut self.job_stores {
                let cpy = &mut value.clone();
                let ready = cpy.get_due_jobs();
                for to_execute in ready {
                    let executioner = self.executors.get(&to_execute.executor);
                    match executioner {
                        None => {
                            return;
                        }
                        Some(e) => {
                            // Only when measuring: 
                            // getElapsedTime(to_execute.start_time);
                            e.execute(&to_execute.job).await;
                            value.remove_job(&to_execute.alias);
                        }
                    };
                }
            }
        }
    }

    pub fn add_job_store(&mut self, mut job_store: JobStore<T>, alias: String) {
        job_store.start();
        self.job_stores.entry(alias).or_insert(job_store);
    }

    pub fn add_job(
        &mut self,
        store_alias: String,
        alias: String,
        job: T,
        executor: String,
        recurring: i128,
        until_success: i32,
        start_time: u128,
    ) {
        let store = self.job_stores.get_mut(&store_alias);
        match store {
            Some(r) => {
                r.add_job(job, alias, executor, recurring, until_success, start_time);
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

    pub fn remove_job(&mut self, alias: String, job_alias: String) {
        let store = self.job_stores.get_mut(&alias);
        match store {
            Some(s) => s.remove_job(&job_alias),
            None => (),
        }
    }

    pub fn remove_executor(&mut self, alias: &String) {
        self.executors.remove(alias);
        return;
    }
}
