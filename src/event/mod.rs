// use crate::store::JobState;
use std::fmt::Debug;
pub trait Eventful
where
  Self: Send + Sync + Clone + Debug, {
}

#[derive(Clone, Debug)]
pub struct Event
where
  Self: Send + Sync, {
  status: String,
  id: String,
  time: i64, // when it occured
              // event: Box<dyn Eventful>, // defined by type, like Job, Store, Executor, Scheduler
}
