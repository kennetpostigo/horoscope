use std::time::SystemTime;

pub struct Job {
    name: String,
    datetime: SystemTime,
    func: Box<Fn() -> ()>,
}
impl Job {}
