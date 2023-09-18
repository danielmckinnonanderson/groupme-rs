use std::error::Error;
use std::time::Duration;

use reqwest::{Client, ClientBuilder, StatusCode, blocking::{Client as BlockingClient, ClientBuilder as BlockingClientBuilder}};
use thiserror::Error;

use crate::OutgoingBotMessage;

pub type BotId = str;

const BASE_URL: &str = "https://api.groupme.com/v3";

#[derive(Clone)]
pub struct GroupmeClient {
    client: Client,
    bot_id: String,
}

#[derive(Clone)]
pub struct SyncGroupmeClient {
    client: BlockingClient, 
    bot_id: String,
}

#[derive(Debug, Error)]
pub enum PostBotMsgError {
    ReqBodySerError,
    ReqNotCompleted,
}

impl std::fmt::Display for PostBotMsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReqBodySerError => write!(f, "Content could not be parsed into JSON"),
            Self::ReqNotCompleted => write!(f, "POST request was not completed")
        }
    }
}

impl SyncGroupmeClient {
    pub fn new(bot_id: String) -> Self {
        SyncGroupmeClient { 
            client: BlockingClientBuilder::new()
                .https_only(true)
                .timeout(Duration::new(6, 0))
                .build()
                .unwrap(),
            bot_id
        }
    }

    pub fn post_bot_msg(&self, content: String) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/bots/post", BASE_URL);

        let body = OutgoingBotMessage {
            bot_id: self.bot_id.to_owned(),
            text: content.to_string()
        };

        let body_str: String = match serde_json::to_string_pretty(&body) {
            Ok(s) => s,
            Err(_) => return Err(Box::new(PostBotMsgError::ReqBodySerError))
        };

        let resp = match self.client 
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_str)
            .send() {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("{e}");
                    return Err(Box::new(PostBotMsgError::ReqNotCompleted));
                }
            };

        match resp.status() {
            StatusCode::BAD_REQUEST => {
                todo!();
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                todo!();
            },
            StatusCode::ACCEPTED => {
                Ok(())
            },
            StatusCode::NOT_FOUND => {
                todo!();
            }
            unmatched =>  {
                println!("{unmatched}");
                todo!();
            }
        }
    }
}

impl GroupmeClient {
    pub fn new(bot_id: String) -> Self {
        GroupmeClient {
            bot_id,
            client: ClientBuilder::new()
                .https_only(true)
                .timeout(Duration::new(10, 0))
                .build()
                .unwrap()
        }
    }


    /// Post a message with `content` using the calling client's `bot_id` as the context.
    /// Returns unit type if the content was accepted by the server.
    /// Returns an error if the content cannot be parsed into JSON,
    /// or if the server responds with a status that is not '200'.
    pub async fn post_bot_message(&self, content: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/bots/post", BASE_URL);

        let body = OutgoingBotMessage {
            bot_id: self.bot_id.to_owned(),
            text: content.to_string()
        };

        let body_str: String = match serde_json::to_string_pretty(&body) {
            Ok(s) => s,
            Err(_) => return Err(Box::new(PostBotMsgError::ReqBodySerError))
        };

        let resp = self.client 
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body_str)
            .send()
            .await?;

        match resp.status() {
            StatusCode::BAD_REQUEST => {
                todo!();
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                todo!();
            },
            StatusCode::ACCEPTED => {
                Ok(())
            },
            StatusCode::NOT_FOUND => {
                todo!();
            }
            unmatched =>  {
                println!("{unmatched}");
                todo!();
            }
        }
    }
}
