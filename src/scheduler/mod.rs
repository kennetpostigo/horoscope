pub mod blocking;

use async_channel;
use async_channel::{Receiver, Sender};
use async_std::prelude::*;
use async_std::stream;
use async_std::task;
use async_trait::async_trait;
use futures::{select, FutureExt};
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::executor::Executor;
use crate::job::Work;
use crate::store::Ledger;

#[derive(Clone, Debug)]
pub enum SchedulerState {
    Uninitialized,
    Running,
    Stopped,
}

impl fmt::Display for SchedulerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchedulerState::Uninitialized => write!(f, "Scheduler::Uninitialized"),
            SchedulerState::Running => write!(f, "Scheduler::Running"),
            SchedulerState::Stopped => write!(f, "Scheduler::Stopped"),
        }
    }
}

pub enum Msg {
    // Scheduler Messages
    // ------------------------------------------------------------------------
    // Executor Msgs:
    AddExecutor(String, Executor),
    RemoveExecutor(String),

    // Store Msgs
    AddStore(String, Box<dyn Ledger>),
    // ModifyStore(String, String),
    RemoveStore(String),

    // Job Msgs
    AddJob(String, String, String, u128, Option<u128>, Box<dyn Work>),
    ModifyJob(String, String),
    RemoveJob(String, String),
    PauseJob(String, String),
    ResumeJob(String, String),

    // Listener Msgs
    // AddListener(String, String, String),
    // RemoveListener(String, String, String),

    // User Messages
    // ------------------------------------------------------------------------
    // Common:
    Log(String, String, String),
}

pub fn get_elapsed_time(start_time: u128) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SOMETHING WENT WRONG WITH THE JOB START DATE");

    println!("{}", now.as_nanos() - start_time * 1000000);
}

pub fn daemon(scheduler: Box<dyn Schedule>) -> (Sender<Msg>, Receiver<Msg>) {
    let mut schdlr = scheduler;
    let (s, r) = async_channel::unbounded();
    let (s_cpy, r_cpy) = (s.clone(), r.clone());
    let mut interval = stream::interval(Duration::from_micros(50));

    task::spawn(async move {
        let sender = s_cpy;
        let reader = r_cpy;
        schdlr.startup();
        loop {
            select! {
                m = reader.recv().fuse() => {
                    println!("Msg is being proxied");
                    match m {
                        Ok(msg) => schdlr.proxy(msg, &sender, &reader),
                        Err(e) => println!("{}", e)
                    }
                },
                i = interval.next().fuse() => {
                    match i {
                        Some(_) => schdlr.check_jobs().await,
                        None => println!("Nothing in innterval hit")
                    }
                }
            };
        }
    });
    (s, r)
}

#[async_trait]
pub trait Schedule
where
    Self: Send + Sync,
{
    fn proxy(&mut self, msg: Msg, sender: &Sender<Msg>, reader: &Receiver<Msg>);

    fn startup(&mut self);

    async fn check_jobs(&mut self);

    fn add_store(&mut self, alias: String, store: Box<dyn Ledger>) -> Result<(), String>;

    fn add_job(
        &mut self,
        alias: String,
        store_alias: String,
        executor: String,
        start_time: u128,
        end_time: Option<u128>,
        job: Box<dyn Work>,
    ) -> Result<(), String>;

    fn add_executor(&mut self, alias: String, executor: Executor) -> Result<(), String>;

    fn modify_job(&mut self, alias: String, store_alias: String) -> Result<(), String>;

    fn pause_job(&mut self, alias: String, store_alias: String) -> Result<(), String>;

    fn resume_job(&mut self, alias: String, store_alias: String) -> Result<(), String>;

    fn remove_store(&mut self, alias: &String) -> Result<(), String>;

    fn remove_job(&mut self, alias: String, job_alias: String) -> Result<(), String>;

    fn remove_executor(&mut self, alias: &String) -> Result<(), String>;

    fn vclone(&self) -> Box<dyn Schedule>;
}
