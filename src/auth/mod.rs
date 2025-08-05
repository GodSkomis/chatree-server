pub mod auth;


pub mod password {
    use argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHash, PasswordHasher, PasswordVerifier, SaltString
        },
        Argon2
    };

    pub fn generate_password_hash(password: String) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
    }

    pub fn verify_password(request_password: String, database_password_hash: String) -> bool {
        let parsed_hash = PasswordHash::new(&database_password_hash).unwrap();

        Argon2::default()
            .verify_password(request_password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}




pub mod jwt_authorization {
    use std::{env, time::{SystemTime, UNIX_EPOCH}};

    use axum::{
        extract::FromRequestParts,
        http::{
            StatusCode,
            request::Parts,
        },
    };
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
    use serde::{Deserialize, Serialize};
    
    use crate::{models::user::UserID, settings::{AUTHORIZATION_HEADER, TICKET_LIFETIME}};

    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JWToken {
        pub user_id: UserID,
        exp: usize,
    }

    impl JWToken {
        pub fn verify(token: &str) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
            decode::<Self>(
                token,
                &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
                &Validation::default()
            )
        }

        pub fn new(user_id: i64) -> Self {
            Self {
                user_id: user_id,
                exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize + TICKET_LIFETIME
            }
        }

        pub fn encode(&self) -> String {
            encode(&Header::default(), &self, &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes())).unwrap()
        }
    }

    #[derive(Debug, Serialize)]
    pub struct JWTResponse {
        pub token: String
    }

    
    pub struct JWTAuthorize(pub TokenData<JWToken>);
    
    impl<S> FromRequestParts<S> for JWTAuthorize
    where
        S: Send + Sync,
    {
        type Rejection = (StatusCode, &'static str);
    
        async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

            let token_header = parts.headers.get(AUTHORIZATION_HEADER)
                .ok_or((StatusCode::UNAUTHORIZED, "`Authorization` header not found"))?;

            let token = token_header.to_str()
                .map_err(|_| (StatusCode::UNAUTHORIZED, "`Authorization` header is not valid UTF-8"))?;
            
            let jwt = JWToken::verify(token)
                .map_err(|err| {
                    tracing::debug!("Failed to parse jwt. Error: {:?}\nToken: `{}`", err, token);
                    (StatusCode::UNAUTHORIZED, "`Authorization` header contains invalid token")
                })?;

            Ok(JWTAuthorize(jwt))
        }
    }
}