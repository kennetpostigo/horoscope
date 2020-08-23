use async_trait::async_trait;

#[derive(Clone, Debug)]
pub enum Status {
    Success,
    Retry,
    Failure,
}

#[derive(Clone, Debug)]
pub struct Job<T: Work> {
    pub alias: String,
    pub executor: String,
    pub recurring: i128,
    pub until_success: i32,
    pub start_time: u128,
    pub job: T,
}

impl<T: Work> Job<T> {
    // TODO: figure out how to default recurring to 0
    pub fn new(
        job: T,
        alias: String,
        executor: String,
        recurring: i128,
        until_success: i32,
        start_time: u128
    ) -> Job<T> {
        Job {
            job,
            alias,
            recurring,
            until_success,
            executor,
            start_time
        }
    }
}

#[async_trait]
pub trait Work {
    async fn func(&self) -> Status;
}
