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

    pub fn verify_password(request_password_hash: String, database_password_hash: String) -> bool {
        let parsed_hash = PasswordHash::new(&database_password_hash).unwrap();

        let argon2 = Argon2::default();

        argon2
            .verify_password(request_password_hash.as_bytes(), &parsed_hash)
            .is_ok()
    }
}