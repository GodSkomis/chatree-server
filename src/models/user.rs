use diesel::prelude::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::users;



#[derive(Debug, Clone, Queryable, Identifiable, Serialize)]
#[diesel(table_name = users)]
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
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub hashed_password: String
}