use reqwest::{Client, ClientBuilder};

use crate::OutgoingBotMessage;

const BASE_URL: &'static str = "https://api.groupme.com/v3";

pub struct GroupmeClient {
    client: Client
}

impl GroupmeClient {
    pub fn new() -> Self {
        GroupmeClient {
            client: ClientBuilder::new()
                .https_only(true)
                .build()
                .unwrap()
        }
    }

    pub async fn post_message(&self, content: &str) {
        let url = format!("{}/bots/post", BASE_URL);
        let body = OutgoingBotMessage {
            bot_id: "some-value", // TODO
            text: content
        };

        let req = self.client
            .post(url)
            .body(body)
            .build()
            .unwrap()
    }
}
