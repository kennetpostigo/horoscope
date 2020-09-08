// use async_trait::async_trait;
// use chrono::prelude::*;
// use colored::*;
// use sqlx::postgres::{PgConnection, PgPool};
// use sqlx::Pool;
// use std::collections::hash_map::Entry;
// use std::collections::HashMap;

// use crate::job::{Job, Work};
// use crate::store::Silo;

// #[derive(Clone, Debug)]
// pub struct Store {
//   pub alias: String,
//   pub pool: Option<Pool<PgConnection>>, // pub jobs: HashMap<String, Job>,
//                                         // logger
// }

// impl Store {
//   pub fn new(alias: String) -> Self {
//     Store {
//       alias: alias.clone(),
//       pool: None,
//       // jobs: HashMap::new(),
//     }
//   }
// }

// #[async_trait]
// impl Silo for Store {
//   async fn start(&mut self) -> Result<(), String> {
//     let pool = PgPool::builder()
//       .max_size(5)
//       .build("postgresql://test:test@localhost:5432/test")
//       .await;

//     match pool {
//       Ok(pg) => {
//         self.pool = Some(pg);
//         Ok(())
//       }
//       Err(e) => {
//         println!("SHIT DONE FUCKED UP");
//         Err(format!("{}", e))
//       }
//     }
//   }

//   fn add_job(
//     &mut self,
//     alias: String,
//     executor: String,
//     start_time: i64,
//     end_time: Option<i64>,
//     job: Box<dyn Work>,
//   ) -> Result<(), String> {
    
//   }

//   fn modify_job(&mut self, alias: &String) -> Result<(), String> {}

//   fn pause_job(&mut self, alias: String) -> Result<(), String> {}

//   fn resume_job(&mut self, alias: String) -> Result<(), String> {}

//   fn remove_job(&mut self, alias: &String) -> Result<(), String> {}

//   fn get_due_jobs(&mut self) -> Result<Vec<&Job>, String> {}

//   fn teardown(&self) -> Result<(), String> {
//     println!(
//       "{}{}",
//       "::::   Tearing Down JobStore "
//         .truecolor(0, 0, 0)
//         .bold()
//         .on_green(),
//       "   ::::".truecolor(0, 0, 0).bold().on_green()
//     );
//     Ok(())
//   }

//   fn vclone(&self) -> Box<dyn Silo> {
//     Box::new(self.clone())
//   }
// }
