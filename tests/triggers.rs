use async_std::task;
use chrono::prelude::*;
use k9::assert_equal;

use horoscope::job::sys::Job;
use horoscope::trigger::{
  and_trigger, job_trigger, or_trigger, retry_trigger, time_trigger, Fire,
  Trigger,
};

#[test]
fn trigger_creation() {
  task::block_on(async {
    let tt = time_trigger::Trigger::new(format!("trigga"), None, None, None);
    let mut trig = Trigger::new(format!("trigga"), Box::new(tt));

    assert_equal!(
      &trig.alias,
      &format!("trigga"),
      "trigger name should be set"
    );

    assert_equal!(
      trig.trigger.should_run().await,
      false,
      "Trigger should be set"
    );
  });
}

#[test]
fn time_trigger_should_run_with_day() {
  task::block_on(async {
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      Some(time_trigger::get_today()),
      None,
    );

    assert_equal!(
      tt.should_run().await,
      true,
      "Time Trigger should run because of the day"
    )
  });
}

#[test]
fn time_trigger_should_run_with_time() {
  task::block_on(async {
    let now = Utc::now();
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      None,
      Some(time_trigger::Time(now.hour(), now.minute())),
    );

    assert_equal!(
      tt.should_run().await,
      true,
      "Time Trigger should run because of the time"
    )
  });
}

#[test]
fn time_trigger_should_run_with_day_time() {
  task::block_on(async {
    let now = Utc::now();
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      Some(time_trigger::get_today()),
      Some(time_trigger::Time(now.hour(), now.minute())),
    );

    assert_equal!(
      tt.should_run().await,
      true,
      "Time Trigger should run because of the day and time match"
    )
  });
}

#[test]
fn time_trigger_should_run_fail_on_day_mismatch() {
  task::block_on(async {
    let now = Utc::now();
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      Some(time_trigger::cycle_day(time_trigger::get_today())),
      Some(time_trigger::Time(now.hour(), now.minute())),
    );

    assert_equal!(
      tt.should_run().await,
      false,
      "Time Trigger should fail because of the day mismatch"
    )
  });
}

#[test]
fn time_trigger_should_run_fail_on_time_mismatch() {
  task::block_on(async {
    let now = Utc::now();
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      Some(time_trigger::get_today()),
      Some(time_trigger::Time(now.hour() + 1, now.minute() + 1)),
    );

    assert_equal!(
      tt.should_run().await,
      false,
      "Time Trigger should fail because of the time mismatch"
    )
  });
}
