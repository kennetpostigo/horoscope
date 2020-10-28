use chrono::prelude::*;
use k9::assert_equal;
use std::collections::HashMap;

use horoscope::job::Status;
use horoscope::ledger::{memory, Ledger};

#[test]
pub fn ledger_creation() {
  let ml = memory::Ledger::new();
  let ml_2 = ml.clone();
  let ledg = Ledger::new(format!("horo"), Box::new(ml));

  assert_equal!(&ledg.alias, &format!("horo"), "Ledger alias should match");
  assert_equal!(
    ml_2,
    memory::Ledger {
      data: HashMap::new(),
      ts: vec![]
    },
    "Memory Ledger should implement PartialEQ"
  );
}

#[test]
pub fn memory_ledger_insert() {
  let mut ledg = Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));

  assert_equal!(
    ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Waiting,
      &1
    ),
    (),
    "Memory Ledger should succeed to insert"
  );

  assert_equal!(
    ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Waiting,
      &1
    ),
    (),
    "Memory Ledger should succeed to insert to same place"
  );

  assert_equal!(
    ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Running,
      &2
    ),
    (),
    "Memory Ledger should succeed even when time is different"
  );

  assert_equal!(
    ledg.ledger.insert(
      &format!("store"),
      &format!("job"),
      &Status::Running,
      &1
    ),
    (),
    "Memory Ledger should succeed even when Status is different"
  );

  assert_equal!(
    ledg.ledger.insert(
      &format!("store"),
      &format!("job1"),
      &Status::Running,
      &1
    ),
    (),
    "Memory Ledger should succeed even when job is different"
  );

  assert_equal!(
    ledg.ledger.insert(
      &format!("store1"),
      &format!("job"),
      &Status::Running,
      &1
    ),
    (),
    "Memory Ledger should succeed with different root"
  );
}

#[test]
pub fn memory_ledger_entry() {
  let mut ledg = Ledger::new(format!("horo"), Box::new(memory::Ledger::new()));

  ledg.ledger.insert(
    &format!("store"),
    &format!("job"),
    &Status::Waiting,
    &Utc::now().timestamp_nanos(),
  );

  assert_equal!(
    ledg.ledger.entry(
      &format!("store"),
      &format!("job"),
      &Status::Waiting,
      &Utc::now().timestamp_nanos()
    ),
    true,
    "Memory Ledger should succeed find the entry"
  );

  assert_equal!(
    ledg
      .ledger
      .entry(&format!("store"), &format!("job"), &Status::Running, &2),
    false,
    "Memory Ledger should not find the entry"
  );

  assert_equal!(
    ledg
      .ledger
      .entry(&format!("store"), &format!("job"), &Status::Running, &1),
    false,
    "Memory Ledger should not find the entry"
  );

  assert_equal!(
    ledg.ledger.entry(
      &format!("store"),
      &format!("job1"),
      &Status::Running,
      &1
    ),
    false,
    "Memory Ledger should not find the entry"
  );

  assert_equal!(
    ledg.ledger.entry(
      &format!("store1"),
      &format!("job"),
      &Status::Running,
      &1
    ),
    false,
    "Memory Ledger should not find the entry"
  );
}
