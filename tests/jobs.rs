use async_std::task;
use chrono::prelude::*;
use k9::assert_equal;
use mockito::mock;
use std::collections::HashMap;

use horoscope::job::network::NetType;
use horoscope::job::{network, sys, Job, Status, Work};
use horoscope::ledger::{memory, Ledger};
use horoscope::trigger::{job_trigger, test_trigger, Trigger};
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
      Status::Failure(format!("Unable to complete request")),
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
fn job_add_trigger_with_existing_alias() {
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
  let trig_2 = trig.clone();

  job
    .add_trigger(Trigger::new(format!("triggy"), Box::new(trig)))
    .unwrap();

  assert_equal!(
    job.add_trigger(Trigger::new(format!("triggy"), Box::new(trig_2))),
    Err(format!("Trigger triggy already exists")),
    "Job trigger should have been added"
  );
}

#[test]
fn job_remove_trigger() {
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

    job.remove_trigger(format!("triggy")).unwrap();

    assert_equal!(
      job.triggers.len(),
      0,
      "Job trigger added should have been removed"
    );
  });
}

#[test]
fn job_remove_nonexisting_trigger() {
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

    assert_equal!(
      job.remove_trigger(format!("triggy")),
      Err(format!("Trigger triggy doesn't exists")),
      "Job trigger that doesn't exist should result in an error"
    );
  });
}

#[test]
fn job_validate_triggers() {
  task::block_on(async {
    let mut ledg =
      Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));

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

    ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Waiting,
      &Utc::now().timestamp_nanos(),
    );

    let jtrig = job_trigger::Trigger::new(
      format!("trigga"),
      format!("job"),
      format!("store"),
      Status::Waiting,
      Utc::now().timestamp_nanos(),
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

    job
      .add_trigger(Trigger::new(format!("trigga"), Box::new(jtrig)))
      .unwrap();

    assert_equal!(
      job.validate_triggers(&mut ledg).await,
      (true, Some(start_time)),
      "Job trigger should have been added"
    );
  })
}

#[test]
fn job_validate_triggers_failure() {
  task::block_on(async {
    let mut ledg =
      Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));

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

    ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Waiting,
      &Utc::now().timestamp_nanos(),
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
      test_trigger::Trigger::new(format!("triggy"), false, None);

    let jtrig = job_trigger::Trigger::new(
      format!("trigga"),
      format!("job"),
      format!("store"),
      Status::Waiting,
      Utc::now().timestamp_nanos() - 100000000000,
    );

    job
      .add_trigger(Trigger::new(format!("triggy"), Box::new(trig)))
      .unwrap();

    job
      .add_trigger(Trigger::new(format!("trigga"), Box::new(jtrig)))
      .unwrap();

    assert_equal!(
      job.validate_triggers(&mut ledg).await,
      (false, None),
      "Job trigger should have been added"
    );
  })
}

#[test]
fn job_pause_job() {
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

    job.pause_job().unwrap();

    assert_equal!(job.state, Status::Paused, "Job should be paused");
  })
}

#[test]
fn job_resume_job() {
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

    job.pause_job().unwrap();
    job.resume_job().unwrap();

    assert_equal!(job.state, Status::Running, "Job should be paused");
  })
}

#[test]
fn trigger_vclone() {
  task::block_on(async {
    let sjob = sys::Job::new(
      String::from("jobby"),
      String::from("echo"),
      vec![format!("test")],
    );

    let sjc = sjob.vclone();

    assert_equal!(sjc.startup().await, Ok(()));

    let njob = network::Job::new(
      String::from("jobby"),
      String::from("http://ping.me"),
      NetType::Get,
      None,
      None,
    );

    let njc = njob.vclone();

    assert_equal!(njc.startup().await, Ok(()));
  });
}
