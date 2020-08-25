use crate::store::JobState;

#[derive(Clone, Debug)]
pub struct Event {
  pub job_state: JobState,
  pub job_id: Option<String>,
  pub executed_at: Option<u128>
}