use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct Chat {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct NewChat {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatUser {
    pub chat_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize)]
pub struct NewChatUser {
    pub chat_id: i64,
    pub user_id: i64,
}
