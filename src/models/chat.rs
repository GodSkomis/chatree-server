use diesel::prelude::{Associations, Identifiable, Insertable, Queryable};
use serde::Serialize;

use crate::schema::*;
use super::user::User;


#[derive(Debug, Clone, Queryable, Identifiable, Serialize)]
#[diesel(table_name = chats)]
pub struct Chat {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Queryable, Identifiable, Associations)]
#[diesel(table_name = chat_users)]
#[diesel(primary_key(chat_id, user_id))]
#[diesel(belongs_to(Chat))]
#[diesel(belongs_to(User))]
pub struct ChatUser {
    pub chat_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = chat_users)]
pub struct NewChatUser {
    pub chat_id: i64,
    pub user_id: i64,
}
