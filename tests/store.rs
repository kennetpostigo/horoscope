use async_std::task;
use chrono::prelude::*;
use k9::assert_equal;

use horoscope::job::sys::Job;
use horoscope::store::Store;

#[test]
fn store_alias_check() {
  task::block_on(async {
    let store = Store::new(String::from("exa"));
    assert_equal!(
      store.alias,
      String::from("exa"),
      "store name should match after creation"
    );
  });
}

#[test]
fn store_startup_always_ok() {
  task::block_on(async {
    let mut store = Store::new(String::from("exa"));
    assert_equal!(store.startup().await, Ok(()), "Startup should be Ok");
  });
}

#[test]
fn store_teardown_always_ok() {
  task::block_on(async {
    let store = Store::new(String::from("exa"));
    assert_equal!(store.teardown(), Ok(()), "Teardown should be Ok");
  });
}

#[test]
fn store_add_job() {
  task::block_on(async {
    let mut store = Store::new(String::from("exa"));

    let start_time = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = 10000000000;
      let start_time = now + delay;
      // println!("{}\n{}", now, now + delay);
      start_time
    };

    let job = Job::new(String::from("job-1"), String::from("ls"), vec![]);

    store
      .add_job(
        String::from("one"),
        String::from("exec-one"),
        start_time,
        None,
        Box::new(job),
      )
      .unwrap();

    assert_equal!(store.jobs.len(), 1, "Store should have a job")
  });
}

#[test]
fn store_remove_job() {
  task::block_on(async {
    let mut store = Store::new(String::from("exa"));

    let start_time = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = 10000000000;
      let start_time = now + delay;
      // println!("{}\n{}", now, now + delay);
      start_time
    };

    let job = Job::new(String::from("job-1"), String::from("ls"), vec![]);

    store
      .add_job(
        String::from("one"),
        String::from("exec-one"),
        start_time,
        None,
        Box::new(job),
      )
      .unwrap();

    store.remove_job(&String::from("one")).unwrap();

    assert_equal!(store.jobs.len(), 0, "Store should no job")
  });
}

#[test]
fn store_get_due_jobs() {
  task::block_on(async {
    let mut store = Store::new(String::from("exa"));

    let start_time = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = -500000000;
      let start_time = now + delay;
      // println!("{}\n{}", now, now + delay);
      start_time
    };

    let job = Job::new(String::from("job-1"), String::from("ls"), vec![]);

    store
      .add_job(
        String::from("one"),
        String::from("exec-one"),
        start_time,
        None,
        Box::new(job),
      )
      .unwrap();

    assert_equal!(
      store.get_due_jobs().unwrap().len(),
      1,
      "Store should have a job that is due"
    )
  });
}

#[test]
fn store_get_due_jobs_2() {
  task::block_on(async {
    let mut store = Store::new(String::from("exa"));

    let start_time = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = 500000000;
      let start_time = now + delay;
      // println!("{}\n{}", now, now + delay);
      start_time
    };

    let job = Job::new(String::from("job-1"), String::from("ls"), vec![]);

    store
      .add_job(
        String::from("one"),
        String::from("exec-one"),
        start_time,
        None,
        Box::new(job),
      )
      .unwrap();

    assert_equal!(
      store.get_due_jobs().unwrap().len(),
      0,
      "Store should not have a job that is due"
    )
  });
}

#[test]
fn store_get_due_jobs_3() {
  task::block_on(async {
    let mut store = Store::new(String::from("exa"));

    let start_time = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = -500000000;
      let start_time = now + delay;
      // println!("{}\n{}", now, now + delay);
      start_time
    };

    let start_time_2 = {
      let now = Utc::now().timestamp_nanos();
      let delay: i64 = 500000000;
      let start_time = now + delay;
      // println!("{}\n{}", now, now + delay);
      start_time
    };

    let job = Job::new(String::from("job-1"), String::from("ls"), vec![]);
    let job2 = Job::new(String::from("job-2"), String::from("ls"), vec![]);

    store
      .add_job(
        String::from("one"),
        String::from("exec-one"),
        start_time,
        None,
        Box::new(job),
      )
      .unwrap();

    store
      .add_job(
        String::from("two"),
        String::from("exec-two"),
        start_time_2,
        None,
        Box::new(job2),
      )
      .unwrap();

    assert_equal!(
      store.get_due_jobs().unwrap().len(),
      1,
      "Store should have a job that is due"
    )
  });
}
