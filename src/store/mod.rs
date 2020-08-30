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
pub trait Silo
where
    Self: Send + Sync,
{
    fn start(&mut self) -> Result<(), String> {
        println!(":: Starting JobStore ::");
        Ok(())
    }

    fn add_job(
        &mut self,
        alias: String,
        executor: String,
        start_time: u128,
        end_time: Option<u128>,
        job: Box<dyn Work>,
    ) -> Result<(), String>;

    fn modify_job(&mut self, alias: &String) -> Result<(), String>;

    fn pause_job(&mut self, alias: String) -> Result<(), String>;

    fn resume_job(&mut self, alias: String) -> Result<(), String>;

    fn remove_job(&mut self, alias: &String) -> Result<(), String>;

    fn get_due_jobs(&mut self) -> Result<Vec<&Job>, String>;

    fn teardown(&self) -> Result<(), String> {
        println!(":: Tearing Down JobStore ::");
        Ok(())
    }

    fn vclone(&self) -> Box<dyn Silo>;
}

pub struct Store {
    pub alias: String,
    pub store: Box<dyn Silo>,
}

impl Store {
    pub fn new(alias: String, store: Box<dyn Silo>) -> Store {
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
