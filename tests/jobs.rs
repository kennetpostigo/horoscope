use async_std::task;
use chrono::prelude::*;
use horoscope::job::network::NetType;
use horoscope::job::{network, sys, Status, Work};
use k9::assert_equal;
use mockito::mock;

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
