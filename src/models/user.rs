use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::schema;


#[derive(Debug, Clone, Queryable, Identifiable, Serialize, Selectable)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub hashed_password: String,
    pub status: Option<String>,
    pub bio: Option<String>,
    pub is_banned: bool,
    pub is_active: bool
}


#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub hashed_password: String
}


impl NewUser {
    pub async fn insert(&self, pool: &super::AppPool) -> i64 {
        let mut conn = pool.get().await.unwrap();
        let user_id: i64 = diesel::insert_into(schema::users::table)
            .values(self)
            .returning(schema::users::id)
            .get_result(&mut conn)
            .await.unwrap();
        user_id
    }
}