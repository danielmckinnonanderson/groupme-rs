use serde::{ Serialize, Deserialize };


#[derive(Serialize, Deserialize)]
pub struct OutgoingBotMessage {
    pub bot_id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupMeMessage {
    pub attachments: Option<Vec<String>>,
    pub avatar_url: Option<String>,
    pub created_at: Option<i64>,
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub sender_id: Option<String>,
    pub sender_type: Option<String>,
    pub source_guid: Option<String>,
    pub system: Option<bool>,
    pub text: Option<String>,
    pub user_id: String,
}

