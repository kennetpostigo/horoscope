use crate::job_store::memory_job_store::JobState;

pub struct Event {
  job_state: JobState,
}