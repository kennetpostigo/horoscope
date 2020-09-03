use async_trait::async_trait;
use chrono::prelude::*;
use chrono::Utc;

use crate::trigger;

#[derive(Clone, Debug)]
pub enum Day {
  Mon,
  Tue,
  Wed,
  Thu,
  Fri,
  Sat,
  Sun,
}

#[derive(Clone, Debug)]
pub struct Time(u32, u32);

fn day_to_chrono_day(day: &Day) -> Weekday {
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

#[derive(Clone, Debug)]
struct Trigger {
  interval: Option<u128>,
  day: Option<Day>,
  time: Option<Time>,
}

#[async_trait]
impl trigger::Fire for Trigger {
  async fn should_run(&mut self) -> bool {
    let now = Utc::now();
    let day_match = match &self.day {
      Some(d) => now.weekday() == day_to_chrono_day(d),
      None => true,
    };

    let time_match = match &self.time {
      Some(Time(h, m)) => {
        let hour = now.hour();
        let min = now.minute();

        &min == m && &hour == h
      }
      None => true,
    };

    day_match && time_match
  }

  async fn next(&mut self) -> Option<u128> {
    let run: bool = self.should_run().await;

    if run {
      self.interval.clone()
    } else {
      None
    }
  }

  fn vclone(&self) -> Box<dyn trigger::Fire> {
    Box::new(self.clone())
  }
}
