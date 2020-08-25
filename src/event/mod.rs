use crate::store::JobState;

#[derive(Clone, Debug)]
pub struct Event where Self: Send + Sync {
    pub job_state: JobState,
    pub job_id: Option<String>,
    pub executed_at: Option<u128>,
}
