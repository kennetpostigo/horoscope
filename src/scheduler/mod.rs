pub mod background;
pub mod blocking;

use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::executor::Executor;
use crate::job::Work;
use crate::store::{Ledger};

#[derive(Clone, Debug)]
pub enum SchedulerState {
    Uninitialized,
    Running,
    Stopped,
}

pub fn get_elapsed_time(start_time: u128) {
  let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");

  println!("{}", now.as_nanos() - start_time * 1000000);
}

#[async_trait]
pub trait Schedule where Self: Send + Sync {
    async fn startup(&mut self);
    fn add_store(&mut self, job_store: Box<dyn Ledger>, alias: String);
    fn add_job(
        &mut self,
        store_alias: String,
        alias: String,
        job: Box<dyn Work>,
        executor: String,
        recurring: u128,
        until_success: i32,
        start_time: u128,
    );
    fn add_executor(&mut self, executor: Executor, alias: String);

    fn remove_store(&mut self, alias: &String);
    fn remove_job(&mut self, alias: String, job_alias: String);
    fn remove_executor(&mut self, alias: &String);
}

