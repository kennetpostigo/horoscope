use async_trait::async_trait;
use std::collections::HashMap;

// use crate::event::Event;
use crate::executor::Executor;
use crate::job::Work;
use crate::scheduler::{Schedule, SchedulerState};
use crate::store::{Ledger, Store};

#[derive(Clone, Debug)]
pub struct Scheduler {
    pub job_stores: HashMap<String, Store>,
    pub executors: HashMap<String, Executor>,
    // pub listeners: Vec<Box<Fn(Event) -> ()>>,
    pub state: SchedulerState,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            executors: HashMap::new(),
            job_stores: HashMap::new(),
            // listeners: vec![],
            state: SchedulerState::Uninitialized,
        }
    }
}

#[async_trait]
impl Schedule for Scheduler {
    async fn startup(&mut self) {
        println!(":: Scheduler starting up ::");
        self.state = SchedulerState::Running;
        loop {
            for (_key, value) in &mut self.job_stores {
                let cpy = &mut value.clone();
                let ready = cpy.store.get_due_jobs();
                for to_execute in ready {
                    let executioner = self.executors.get(&to_execute.executor);
                    match executioner {
                        None => {
                            return;
                        }
                        Some(e) => {
                            // Only when measuring:
                            // get_elapsed_time(to_execute.start_time);
                            e.execute(&to_execute.job).await;
                            value.store.remove_job(&to_execute.alias);
                        }
                    };
                }
            }
        }
    }

    fn add_store(&mut self, job_store: Box<dyn Ledger>, alias: String) {
        let mut store = Store::new(job_store, alias.clone());
        store.store.start();
        self.job_stores.entry(alias).or_insert(store);
    }

    fn add_job(
        &mut self,
        store_alias: String,
        alias: String,
        job: Box<dyn Work>,
        executor: String,
        recurring: u128,
        until_success: i32,
        start_time: u128,
    ) {
        let store = self.job_stores.get_mut(&store_alias);
        match store {
            Some(r) => {
                r.store
                    .add_job(job, alias, executor, recurring, until_success, start_time);
            }
            None => println!("Nothing"),
        }
    }

    fn add_executor(&mut self, executor: Executor, alias: String) {
        executor.start();
        self.executors.entry(alias).or_insert(executor);
    }

    fn remove_store(&mut self, alias: &String) {
        self.job_stores.remove(alias);
        return;
    }

    fn remove_job(&mut self, alias: String, job_alias: String) {
        let store = self.job_stores.get_mut(&alias);
        match store {
            Some(s) => s.store.remove_job(&job_alias),
            None => (),
        }
    }

    fn remove_executor(&mut self, alias: &String) {
        self.executors.remove(alias);
        return;
    }
}
