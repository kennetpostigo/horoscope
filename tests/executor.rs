use async_std::task;
use chrono::prelude::*;
use k9::assert_equal;
use std::collections::HashMap;

use horoscope::executor::Executor;
use horoscope::job::{sys, Job};

#[test]
fn executor_startup_always_ok() {
  task::block_on(async {
    let exctr = Executor::new(String::from("exa"));
    assert_equal!(exctr.startup(), Ok(()), "Startup should be Ok");
  });
}

#[test]
fn executor_teardown_always_ok() {
  task::block_on(async {
    let exctr = Executor::new(String::from("exa"));
    assert_equal!(exctr.teardown(), Ok(()), "Teardown should be Ok");
  });
}

#[test]
fn executor_execute() {
  task::block_on(async {
    let start_time = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = -10000000000;
      now + delay
    };

    let sjob = sys::Job::new(
      String::from("jobby"),
      String::from("echo"),
      vec![format!("test")],
    );

    let job = Job::new(
      String::from("jobby"),
      String::from("exo"),
      start_time,
      None,
      HashMap::new(),
      Box::new(sjob),
    );

    let exctr = Executor::new(format!("exo"));

    assert_equal!(
      exctr.execute(&job.job).await,
      Ok(()),
      "job execution should succeed"
    );
  })
}
