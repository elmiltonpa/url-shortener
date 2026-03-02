use crate::error::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub async fn hash_password(password: &str) -> Result<String, AppError> {
    let password = password.to_string();
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordHashFailed)?;

        Ok(hashed_password.to_string())
    })
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Password hashing task failed: {}", e)))?
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let password = password.to_string();
    let hash = hash.to_string();
    tokio::task::spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(&hash).map_err(|_| AppError::PasswordHashFailed)?;

        let is_valid = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    })
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!("Password verify task failed: {}", e)))?
}
