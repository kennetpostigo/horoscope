use async_trait::async_trait;

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
    pub fn new(alias: String, url: String, method: NetType, body: Option<String>) -> Self {
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
    async fn startup(&self) {
        println!("Starting Network Job: {}", self.alias);
    }

    async fn func(&self) -> Status {
        match &self.method {
            NetType::Get => match surf::get(&self.url).recv_string().await {
                Ok(msg) => {
                    print!("{}", msg);
                    Status::Success
                }
                Err(_) => Status::Failure,
            },
            NetType::Post => match &self.body {
                Some(bdy) => {
                    let data = serde_json::json!(bdy);
                    match http_types::Body::from_json(&data) {
                        Ok(bdy) => {
                            let res = surf::post(&self.url).body(bdy).await;

                            match res {
                                Ok(r) => {
                                    println!("{}", r.status());
                                    Status::Success
                                }
                                Err(_) => Status::Failure,
                            }
                        }
                        Err(_) => Status::Failure,
                    }
                }
                None => {
                    let res = surf::post(&self.url).await;
                    match res {
                        Ok(r) => {
                            println!("{}", r.status());
                            Status::Success
                        }
                        Err(_) => Status::Failure,
                    }
                }
            },
        }
    }

    async fn teardown(&self) {
        println!("Tearing down Network Job: {}", self.alias);
    }

    fn vclone(&self) -> Box<dyn Work> {
        Box::new(self.clone())
    }
}
