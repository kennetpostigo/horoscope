use async_trait::async_trait;
use chrono::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::trigger::Fire;
use crate::ledger::Ledger;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Day {
  Mon,
  Tue,
  Wed,
  Thu,
  Fri,
  Sat,
  Sun,
}

pub fn cycle_day(day: Day) -> Day {
  match day {
    Day::Mon => Day::Tue,
    Day::Tue => Day::Wed,
    Day::Wed => Day::Thu,
    Day::Thu => Day::Fri,
    Day::Fri => Day::Sat,
    Day::Sat => Day::Sun,
    Day::Sun => Day::Mon,
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Time(pub u32, pub u32);

pub fn day_to_chrono_day(day: &Day) -> Weekday {
  match day {
    Day::Mon => Weekday::Mon,
    Day::Tue => Weekday::Tue,
    Day::Wed => Weekday::Wed,
    Day::Thu => Weekday::Thu,
    Day::Fri => Weekday::Fri,
    Day::Sat => Weekday::Sat,
    Day::Sun => Weekday::Sun,
  }
}

pub fn chrono_day_to_day(day: Weekday) -> Day {
  match day {
    Weekday::Mon => Day::Mon,
    Weekday::Tue => Day::Tue,
    Weekday::Wed => Day::Wed,
    Weekday::Thu => Day::Thu,
    Weekday::Fri => Day::Fri,
    Weekday::Sat => Day::Sat,
    Weekday::Sun => Day::Sun,
  }
}

pub fn get_today() -> Day {
  let now = chrono::Utc::now();
  chrono_day_to_day(now.weekday())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trigger {
  alias: String,
  interval: Option<i64>,
  day: Option<Day>,
  time: Option<Time>,
}

impl Trigger {
  pub fn new(
    alias: String,
    interval: Option<i64>,
    day: Option<Day>,
    time: Option<Time>,
  ) -> Self {
    Trigger {
      alias,
      interval,
      day,
      time,
    }
  }
}

#[derive(PartialEq)]
enum DateTimeMatch {
  Match,
  MisMatch,
  Nothing,
}

#[async_trait ]
#[typetag::serde(name = "TimeTrigger")]
impl Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    let now = Utc::now();
    let day_match = match &self.day {
      Some(d) => match now.weekday() == day_to_chrono_day(d) {
        true => DateTimeMatch::Match,
        false => DateTimeMatch::MisMatch,
      },
      None => DateTimeMatch::Nothing,
    };

    let time_match = match &self.time {
      Some(Time(h, m)) => {
        let hour = now.hour();
        let min = now.minute();

        match &min == m && &hour == h {
          true => DateTimeMatch::Match,
          false => DateTimeMatch::MisMatch,
        }
      }
      None => DateTimeMatch::Nothing,
    };

    match (day_match, time_match) {
      (DateTimeMatch::Match, DateTimeMatch::Match) => true,
      (DateTimeMatch::Match, DateTimeMatch::MisMatch) => false,
      (DateTimeMatch::Match, DateTimeMatch::Nothing) => true,
      (DateTimeMatch::MisMatch, DateTimeMatch::Match) => false,
      (DateTimeMatch::MisMatch, DateTimeMatch::MisMatch) => false,
      (DateTimeMatch::MisMatch, DateTimeMatch::Nothing) => false,
      (DateTimeMatch::Nothing, DateTimeMatch::Match) => true,
      (DateTimeMatch::Nothing, DateTimeMatch::MisMatch) => false,
      (DateTimeMatch::Nothing, DateTimeMatch::Nothing) => false,
    }
  }

  async fn should_run_with_ledger(&mut self, _ledger: &mut Ledger) -> bool {
    panic!("trigger::time_trigger - DOES NOT REQUIRE LEDGER")
  }

  async fn next(&mut self) -> Option<i64> {
    let run: bool = self.should_run().await;

    if run {
      let now = Utc::now().timestamp_nanos();
      match self.interval.clone() {
        Some(interval) => Some(now + interval),
        None => None,
      }
    } else {
      None
    }
  }

  fn vclone(&self) -> Box<dyn Fire> {
    Box::new(self.clone())
  }
}
