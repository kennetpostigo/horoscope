use async_trait::async_trait;

pub enum Status {
    Success,
    Retry,
    Failure,
}

pub struct Job<T: Work> {
    pub executor: String,
    pub job: T,
}

impl<T: Work> Job<T> {
    pub fn new(job: T, alias: String) -> Job<T> {
        Job {
            job,
            executor: alias,
        }
    }
}

#[async_trait]
pub trait Work {
    async fn func(&self) -> Status;
}
