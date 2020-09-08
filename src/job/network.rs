use async_trait::async_trait;
use colored::*;

use crate::job::{Status, Work};

#[derive(Clone, Debug)]
pub enum NetType {
  Get,
  Post,
}

#[derive(Clone, Debug)]
pub struct Job {
  pub alias: String,
  pub url: String,
  pub method: NetType,
  pub body: Option<String>,
}

impl Job {
  pub fn new(
    alias: String,
    url: String,
    method: NetType,
    body: Option<String>,
  ) -> Self {
    Job {
      alias,
      url,
      method,
      body,
    }
  }
}

#[async_trait]
impl Work for Job {
  async fn startup(&self) -> Result<(), String> {
    println!(
      "{}{}{}",
      "::::   Starting Network Job ".truecolor(0, 0, 0).bold().on_green(),
      self.alias.truecolor(0, 0, 0).bold().on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(())
  }

  async fn func(&self) -> Status {
    match &self.method {
      NetType::Get => match surf::get(&self.url).await {
        Ok(_msg) => {
          // println!("Network Job Response Status: {}", msg.status());
          Status::Success
        }
        Err(_) => Status::Failure(String::from("Unable to complete request")),
      },
      NetType::Post => match &self.body {
        Some(bdy) => {
          let data = serde_json::json!(bdy);
          match http_types::Body::from_json(&data) {
            Ok(bdy) => {
              let res = surf::post(&self.url).body(bdy).await;

              match res {
                Ok(_r) => {
                  // println!("Network Job Response Status: {}", r.status());
                  Status::Success
                }
                Err(_) => {
                  Status::Failure(String::from("Unable to complete request"))
                }
              }
            }
            Err(_) => Status::Failure(String::from("Unable to parse body")),
          }
        }
        None => {
          let res = surf::post(&self.url).await;
          match res {
            Ok(_r) => {
              // println!("Network Job Response Status: {}", r.status());
              Status::Success
            }
            Err(_) => {
              Status::Failure(String::from("Unable to complete request"))
            }
          }
        }
      },
    }
  }

  async fn teardown(&self) -> Result<String, String> {
    println!(
      "{}{}{}",
      "::::   Tearing Down Network Job ".truecolor(0, 0, 0).bold().on_green(),
      self.alias.truecolor(0, 0, 0).bold().on_green(),
      "   ::::".truecolor(0, 0, 0).bold().on_green()
    );
    Ok(String::from(""))
  }

  fn vclone(&self) -> Box<dyn Work> {
    Box::new(self.clone())
  }
}
