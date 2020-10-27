use async_std::task;
use chrono::prelude::*;
use k9::assert_equal;

use horoscope::job::Status;
use horoscope::ledger::{memory, Ledger};
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
fn chrono_to_day() {
  let mon = chrono::Weekday::Mon;
  let tue = chrono::Weekday::Tue;
  let wed = chrono::Weekday::Wed;
  let thu = chrono::Weekday::Thu;
  let fri = chrono::Weekday::Fri;
  let sat = chrono::Weekday::Sat;
  let sun = chrono::Weekday::Sun;
  assert_equal!(time_trigger::chrono_day_to_day(mon), time_trigger::Day::Mon);
  assert_equal!(time_trigger::chrono_day_to_day(tue), time_trigger::Day::Tue);
  assert_equal!(time_trigger::chrono_day_to_day(wed), time_trigger::Day::Wed);
  assert_equal!(time_trigger::chrono_day_to_day(thu), time_trigger::Day::Thu);
  assert_equal!(time_trigger::chrono_day_to_day(fri), time_trigger::Day::Fri);
  assert_equal!(time_trigger::chrono_day_to_day(sat), time_trigger::Day::Sat);
  assert_equal!(time_trigger::chrono_day_to_day(sun), time_trigger::Day::Sun);
}

#[test]
fn day_to_chrono() {
  let mon = &time_trigger::Day::Mon;
  let tue = &time_trigger::Day::Tue;
  let wed = &time_trigger::Day::Wed;
  let thu = &time_trigger::Day::Thu;
  let fri = &time_trigger::Day::Fri;
  let sat = &time_trigger::Day::Sat;
  let sun = &time_trigger::Day::Sun;
  assert_equal!(time_trigger::day_to_chrono_day(mon), chrono::Weekday::Mon);
  assert_equal!(time_trigger::day_to_chrono_day(tue), chrono::Weekday::Tue);
  assert_equal!(time_trigger::day_to_chrono_day(wed), chrono::Weekday::Wed);
  assert_equal!(time_trigger::day_to_chrono_day(thu), chrono::Weekday::Thu);
  assert_equal!(time_trigger::day_to_chrono_day(fri), chrono::Weekday::Fri);
  assert_equal!(time_trigger::day_to_chrono_day(sat), chrono::Weekday::Sat);
  assert_equal!(time_trigger::day_to_chrono_day(sun), chrono::Weekday::Sun);
}

#[test]
fn cycle_day() {
  let mon = time_trigger::Day::Mon;
  let tue = time_trigger::Day::Tue;
  let wed = time_trigger::Day::Wed;
  let thu = time_trigger::Day::Thu;
  let fri = time_trigger::Day::Fri;
  let sat = time_trigger::Day::Sat;
  let sun = time_trigger::Day::Sun;
  assert_equal!(time_trigger::cycle_day(mon), time_trigger::Day::Tue);
  assert_equal!(time_trigger::cycle_day(tue), time_trigger::Day::Wed);
  assert_equal!(time_trigger::cycle_day(wed), time_trigger::Day::Thu);
  assert_equal!(time_trigger::cycle_day(thu), time_trigger::Day::Fri);
  assert_equal!(time_trigger::cycle_day(fri), time_trigger::Day::Sat);
  assert_equal!(time_trigger::cycle_day(sat), time_trigger::Day::Sun);
  assert_equal!(time_trigger::cycle_day(sun), time_trigger::Day::Mon);
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

#[test]
fn time_trigger_should_run_fail_on_day_time_mismatch() {
  task::block_on(async {
    let now = Utc::now();
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      Some(time_trigger::cycle_day(time_trigger::get_today())),
      Some(time_trigger::Time(now.hour() + 1, now.minute() + 1)),
    );

    assert_equal!(
      tt.should_run().await,
      false,
      "Time Trigger should fail because of the day & time mismatch"
    )
  });
}

#[test]
fn time_trigger_should_run_fail_on_day_mismatch_nothing() {
  task::block_on(async {
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      Some(time_trigger::cycle_day(time_trigger::get_today())),
      None,
    );

    assert_equal!(
      tt.should_run().await,
      false,
      "Time Trigger should fail because of the day mismatch and nothing time"
    )
  });
}

#[test]
fn time_trigger_should_run_fail_on_nothing_time_mismatch() {
  task::block_on(async {
    let now = Utc::now();
    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      None,
      Some(time_trigger::Time(now.hour() + 1, now.minute() + 1)),
    );

    assert_equal!(
      tt.should_run().await,
      false,
      "Time Trigger should fail because of the nothing and time mismatch"
    )
  });
}

#[should_panic(expected = "trigger::time_trigger - DOES NOT REQUIRE LEDGER")]
#[test]
fn time_trigger_should_panic_with_ledger() {
  task::block_on(async {
    let now = Utc::now();

    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      None,
      Some(time_trigger::Time(now.hour() + 1, now.minute() + 1)),
    );

    let mut ledg =
      Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));

    assert_equal!(
      tt.should_run_with_ledger(&mut ledg).await,
      false,
      "Time Trigger should fail because it doesn't require ledger"
    )
  });
}

#[test]
fn time_trigger_next_some() {
  task::block_on(async {
    let now = Utc::now();

    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      Some(5),
      None,
      Some(time_trigger::Time(now.hour(), now.minute())),
    );

    assert_equal!(
      tt.next().await != None,
      true,
      "Time Trigger next should result in Some(interval) with interval"
    )
  });
}

#[test]
fn time_trigger_next_none() {
  task::block_on(async {
    let now = Utc::now();

    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      None,
      None,
      Some(time_trigger::Time(now.hour(), now.minute())),
    );

    assert_equal!(
      tt.next().await,
      None,
      "Time Trigger next should result in None without interval"
    )
  });
}

#[test]
fn time_trigger_next_mismatch() {
  task::block_on(async {
    let now = Utc::now();

    let mut tt = time_trigger::Trigger::new(
      format!("trigga"),
      Some(5),
      None,
      Some(time_trigger::Time(now.hour() + 1, now.minute() + 1)),
    );

    assert_equal!(
      tt.next().await,
      None,
      "Time Trigger next should result in None without interval"
    )
  });
}

#[test]
fn job_trigger_should_run_with_ledger() {
  task::block_on(async {
    let time = Utc::now().timestamp_nanos();
    let mut ledg =
      Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));
    let mut jt = job_trigger::Trigger::new(
      format!("trigga"),
      format!("job"),
      format!("store"),
      Status::Waiting,
      time,
    );

    &ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Waiting,
      &time,
    );

    assert_equal!(
      jt.should_run_with_ledger(&mut ledg).await,
      true,
      "Job Trigger should run"
    );
  });
}

#[test]
fn job_trigger_should_run_with_ledger_false() {
  task::block_on(async {
    let time = Utc::now().timestamp_nanos();
    let mut ledg =
      Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));
    let mut jt = job_trigger::Trigger::new(
      format!("trigga"),
      format!("job"),
      format!("store"),
      Status::Waiting,
      time,
    );

    assert_equal!(
      jt.should_run_with_ledger(&mut ledg).await,
      false,
      "Job Trigger shouldn't run"
    );
  });
}

#[should_panic(expected = "trigger::job_trigger - REQUIRES SHOULD_RUN")]
#[test]
fn job_trigger_should_run() {
  task::block_on(async {
    let time = Utc::now().timestamp_nanos();

    let mut jt = job_trigger::Trigger::new(
      format!("trigga"),
      format!("job"),
      format!("store"),
      Status::Waiting,
      time,
    );

    assert_equal!(
      jt.should_run().await,
      true,
      "Job Trigger should_run should panic"
    );
  });
}

#[test]
fn job_trigger_next() {
  task::block_on(async {
    let time = Utc::now().timestamp_nanos();
    let mut ledg =
      Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));
    let mut jt = job_trigger::Trigger::new(
      format!("trigga"),
      format!("job"),
      format!("store"),
      Status::Waiting,
      time,
    );

    &ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Waiting,
      &time,
    );

    assert_equal!(jt.next().await, None, "Job Trigger next");
  });
}

#[test]
fn job_trigger_needs_ledger() {
  task::block_on(async {
    let time = Utc::now().timestamp_nanos();

    let jt = job_trigger::Trigger::new(
      format!("trigga"),
      format!("job"),
      format!("store"),
      Status::Waiting,
      time,
    );

    assert_equal!(jt.needs_ledger(), true, "Job Trigger should need ledger");
  });
}
