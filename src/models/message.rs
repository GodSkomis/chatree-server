use chrono::NaiveDateTime;
use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub chat_id: i64,
    pub content: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct NewMessage {
    pub user_id: i64,
    pub chat_id: i64,
    pub content: String
}
