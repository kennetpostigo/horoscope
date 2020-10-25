use async_std::task;
use chrono::prelude::*;
use horoscope::executor::Executor;
use horoscope::job::network::NetType;
use horoscope::job::{network, sys, Job, Status, Work};
use horoscope::trigger::{test_trigger, Trigger};
use k9::assert_equal;
use mockito::mock;
use std::collections::HashMap;

#[test]
fn sys_job_startup_ok() {
  task::block_on(async {
    let job = sys::Job::new(
      String::from("one"),
      String::from("echo"),
      vec![format!("test")],
    );
    assert_equal!(job.startup().await, Ok(()), "Startup should be ok");
    assert_equal!(job.alias, String::from("one"), "Job alias should match");
    assert_equal!(job.script, String::from("echo"), "Job script should match");
    assert_equal!(job.args, vec![format!("test")], "Args should match");
  });
}

#[test]
fn sys_job_teardown_ok() {
  task::block_on(async {
    let job = sys::Job::new(
      String::from("one"),
      String::from("echo"),
      vec![format!("test")],
    );
    assert_equal!(job.teardown().await, Ok(()), "Teardown should be ok");
  });
}

#[test]
fn sys_job_func() {
  task::block_on(async {
    let job = sys::Job::new(
      String::from("one"),
      String::from("echo"),
      vec![format!("test")],
    );
    assert_equal!(
      job.func().await,
      Status::Success,
      "func should run to success"
    );
  });
}

#[test]
fn sys_job_func_fail() {
  task::block_on(async {
    let job = sys::Job::new(
      String::from("one"),
      String::from("no_sir"),
      vec![format!("test")],
    );
    assert_equal!(
      job.func().await,
      Status::Failure(format!(
        "Failed to successfully run {} with {:?}",
        &job.script, &job.args
      )),
      "func should fail"
    );
  });
}

#[test]
fn net_job_startup_ok() {
  task::block_on(async {
    let job = network::Job::new(
      String::from("one"),
      String::from("https://ping.me/"),
      NetType::Get,
      None,
      None,
    );

    assert_equal!(job.startup().await, Ok(()), "Startup should be ok");
    // assert_eq!(job.alias, String::from("one"), "Job alias should match");
    // assert_eq!(job.script, String::from("ls"), "Job script should match");
    // assert_equal!(job.args, vec![], "Args should match");
  });
}

#[test]
fn net_job_func() {
  task::block_on(async {
    let url = mockito::server_url();

    let _m = mock("GET", "/success").with_status(200).create();

    let job = network::Job::new(
      String::from("one"),
      format!("{}/success", url),
      NetType::Get,
      None,
      None,
    );

    assert_equal!(
      job.func().await,
      Status::Success,
      "func should run to success"
    );
  });
}

#[test]
fn net_job_func_fail() {
  task::block_on(async {
    let url = mockito::server_url();
    let _m = mock("GET", "/fail").with_status(500).create();

    let job = network::Job::new(
      String::from("one"),
      format!("{}/fail", url),
      NetType::Get,
      None,
      None,
    );

    assert_equal!(
      job.func().await,
      Status::Failure(String::from("Unable to complete request")),
      "func should run to success"
    );
  });
}

#[test]
fn net_job_teardown_ok() {
  task::block_on(async {
    let job = network::Job::new(
      String::from("one"),
      String::from("https://ping.me/"),
      NetType::Get,
      None,
      None,
    );

    assert_equal!(job.teardown().await, Ok(()), "Teardown should be ok");
  });
}

#[test]
fn job_creation() {
  task::block_on(async {
    let start_time = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = 10000000000;
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

    assert_equal!(&job.alias, &format!("jobby"), "Jobs alias should match");
    assert_equal!(&job.executor, &format!("exo"), "Job executor should match");
    assert_equal!(&job.start_time, &start_time, "job start_time should match");
    assert_equal!(&job.end_time, &None, "Job end_time should match");
    assert_equal!(job.triggers.len(), 0, "Should be no triggers")
  });
}

#[test]
fn job_add_trigger() {
  let start_time = {
    let now = Utc::now().timestamp_nanos();
    let delay: i64 = 10000000000;
    now + delay
  };

  let sjob = sys::Job::new(
    String::from("jobby"),
    String::from("echo"),
    vec![format!("test")],
  );

  let mut job = Job::new(
    String::from("jobby"),
    String::from("exo"),
    start_time,
    None,
    HashMap::new(),
    Box::new(sjob),
  );

  let trig = test_trigger::Trigger::new(format!("triggy"), true, None);

  job
    .add_trigger(Trigger::new(format!("triggy"), Box::new(trig)))
    .unwrap();

  assert_equal!(job.triggers.len(), 1, "Job trigger should have been added");
}

#[test]
fn job_validate_triggers() {
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

    let mut job = Job::new(
      String::from("jobby"),
      String::from("exo"),
      start_time,
      None,
      HashMap::new(),
      Box::new(sjob),
    );

    let trig =
      test_trigger::Trigger::new(format!("triggy"), true, Some(start_time));

    job
      .add_trigger(Trigger::new(format!("triggy"), Box::new(trig)))
      .unwrap();

    assert_equal!(
      job.validate_triggers().await,
      (true, Some(start_time)),
      "Job trigger should have been added"
    );
  })
}
