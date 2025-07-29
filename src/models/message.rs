use diesel::prelude::{Associations, Identifiable, Insertable, Queryable};
use chrono::NaiveDateTime;

use super::{user::User, chat::Chat};
use crate::schema::chat_messages;


#[derive(Debug, Clone, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Chat))]
#[diesel(table_name = chat_messages)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub chat_id: i64,
    pub content: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = chat_messages)]
pub struct NewMessage {
    pub user_id: i64,
    pub chat_id: i64,
    pub content: String
}
