// use async_trait::async_trait;
// use colored::*;
// use async_std::

// use crate::job::{Status, Work};

// #[derive(Clone, Debug)]
// pub struct Job {
//   pub alias: String,
//   pub script: String,
//   pub args: String
// }

// impl Job {
//   pub fn new(
//     alias: String,
//     script: String,
//     args: String,
//   ) -> Self {
//     Job {
//       alias,
//       script,
//       args,
//     }
//   }
// }

// #[async_trait]
// impl Work for Job {
//   async fn startup(&self) -> Result<(), String> {
//     println!(
//       "{}{}{}",
//       "::::   Starting Sys Job ".truecolor(0, 0, 0).bold().on_green(),
//       self.alias.truecolor(0, 0, 0).bold().on_green(),
//       "   ::::".truecolor(0, 0, 0).bold().on_green()
//     );
//     Ok(())
//   }

//   async fn func(&self) -> Status {

//   }

//   async fn teardown(&self) -> Result<String, String> {
//     println!(
//       "{}{}{}",
//       "::::   Tearing Down Sys Job ".truecolor(0, 0, 0).bold().on_green(),
//       self.alias.truecolor(0, 0, 0).bold().on_green(),
//       "   ::::".truecolor(0, 0, 0).bold().on_green()
//     );
//     Ok(String::from(""))
//   }

//   fn vclone(&self) -> Box<dyn Work> {
//     Box::new(self.clone())
//   }
// }
