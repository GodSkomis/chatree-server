use serde::{Deserialize, Serialize};
use sqlx::postgres::PgDatabaseError;

use crate::models::errors::ModelError;


pub type UserID = i64;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: UserID,
    pub username: String,
    pub name: String,
    pub hashed_password: String,
    pub status: Option<String>,
    pub bio: Option<String>,
    pub is_banned: bool,
    pub is_active: bool
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserAthorizeDTO {
    pub id: UserID,
    pub hashed_password: String
}

impl User {
    pub async fn find(user_id: UserID, pool: &super::AppPool) -> Result<Option<Self>, ModelError> {
        let result = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool)
        .await;

        match result {
            Ok(_user) => Ok(_user),
            Err(e) => {
                tracing::error!("{:?}", e);
                Err(ModelError::UnexpectedError("Internal server error".to_string()))
            }
        }
    }

    pub async fn authorize(username: String, pool: &super::AppPool) -> Result<Option<UserAthorizeDTO>, ModelError> {
        let dto = sqlx::query_as!(
            UserAthorizeDTO,
            "SELECT id, hashed_password FROM users WHERE username = $1",
            username
        )
        .fetch_optional(pool)
        .await;
        
        match dto {
            Ok(_dto) => Ok(_dto),
            Err(e) => {
                tracing::error!("{:?}", e);
                Err(ModelError::UnexpectedError("Internal server error".to_string()))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub id: UserID,
    pub username: String,
    pub name: String,
    pub hashed_password: String,
    pub status: Option<String>,
    pub bio: Option<String>,
}

impl NewUser {
    pub async fn insert(&self, pool: &super::AppPool) -> Result<i64, ModelError> {
        let user_record = sqlx::query!(
            r#"
                INSERT INTO
                users (id, username, name, hashed_password, status, bio)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id;
            "#,
            self.id,
            self.username.clone(),
            self.name,
            self.hashed_password,
            self.status,
            self.bio
        )
            .fetch_one(pool)
            .await;
    
        match user_record {
            Ok(_record) => Ok(_record.id),
            Err(sqlx::Error::Database(db_err)) => {
                if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    if pg_err.code() == "23505" {
                        return Err(ModelError::ClientError(
                            format!(
                                "User with given username ({}) already exists",
                                self.username
                            )
                        ));
                    }
                }
                Err(ModelError::UnexpectedError("Internal server error".to_string()))
            }
            Err(e) => {
                tracing::error!("{:?}", e);
                Err(ModelError::UnexpectedError("Internal server error".to_string()))
            }
        }
    }

}