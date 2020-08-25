pub mod memory;
pub mod pg;
pub mod redis;

use std::fmt::Debug;

use crate::job::{Job, Work};

#[derive(Clone, Debug)]
pub enum JobState {
    Success,
    Failure,
}
pub trait Ledger
where
    Self: Send + Sync,
{
    fn start(&mut self);
    fn add_job(
        &mut self,
        job: Box<dyn Work>,
        alias: String,
        executor: String,
        recurring: u128,
        until_success: i32,
        start_time: u128,
    );
    fn remove_job(&mut self, alias: &String);
    fn get_due_jobs(&mut self) -> Vec<&Job>;
    fn teardown(&self);
    fn vclone(&self) -> Box<dyn Ledger>;
}

pub struct Store {
    pub alias: String,
    pub store: Box<dyn Ledger>,
}

impl Store {
    pub fn new(store: Box<dyn Ledger>, alias: String) -> Store {
        Store { store, alias }
    }
}

impl Clone for Store {
    fn clone(&self) -> Self {
        Store {
            alias: self.alias.clone(),
            store: self.store.vclone(),
        }
    }
}

impl Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Job")
            .field("alias", &self.alias)
            .field("job", &"<store>")
            .finish()
    }
}
