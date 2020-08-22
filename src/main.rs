mod scheduler;
mod job_store;
mod executor;
mod job;
mod event;

use crate::scheduler::blocking_scheduler::{Scheduler};

fn main() {
   let scheduler = Scheduler::new();
}
