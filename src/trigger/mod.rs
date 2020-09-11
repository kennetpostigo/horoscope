pub mod and_trigger;
pub mod job_trigger;
pub mod or_trigger;
pub mod time_trigger;

use async_trait::async_trait;
use std::fmt::Debug;

pub struct Trigger {
  pub alias: String,
  pub trigger: Box<dyn Fire>,
}

#[async_trait]
pub trait Fire: Send + Sync {
  async fn should_run(&mut self) -> bool;

  async fn next(&mut self) -> Option<i64>;

  fn vclone(&self) -> Box<dyn Fire>;
}

impl Clone for Trigger {
  fn clone(&self) -> Self {
    Trigger {
      alias: self.alias.clone(),
      trigger: self.trigger.vclone(),
    }
  }
}

impl Debug for Trigger {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Trigger")
      .field("alias", &self.alias)
      .field("trigger", &"<trigger>")
      .finish()
  }
}

// Job starttime tuesday 11 ms-epoch
// Trigger - TimeTrigger - interval:30ms - day:monday

// getDueJobs -> start-time <= now & all triggers pass => then add the job to result
