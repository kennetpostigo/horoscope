pub mod memory;
pub mod pg;
pub mod redis;

use crate::job::Work;

#[derive(Clone, Debug)]
pub enum JobState {
  Success,
  Failure
}

pub trait Store {
  fn new (alias: String) -> Self;
  
  fn startup(&self);

  fn add_job(&mut self, job: Box<dyn Work>);

  fn teardown(&self);
}