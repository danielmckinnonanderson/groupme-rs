use std::error::Error;
use std::time::Duration;

use reqwest::{Client, ClientBuilder, StatusCode};
use thiserror::Error;

use crate::OutgoingBotMessage;

pub type BotId = str;

const BASE_URL: &'static str = "https://api.groupme.com/v3";

pub struct GroupmeClient {
    client: Client,
    bot_id: String,
}

#[derive(Debug, Error)]
pub enum PostBotMsgError {
    MsgBodyRejected,
    ReqBodySerError,
}

impl std::fmt::Display for PostBotMsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl GroupmeClient {
    pub fn new(bot_id: &BotId) -> Self {
        GroupmeClient {
            bot_id: bot_id.to_string(),
            client: ClientBuilder::new()
                .https_only(true)
                .timeout(Duration::new(10, 0))
                .build()
                .unwrap()
        }
    }

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
