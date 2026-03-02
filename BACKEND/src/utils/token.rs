use pasetors::Local;
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{local, version4::V4};

use crate::error::AppError;

pub fn create_token(user_id: &str, secret_key: &[u8]) -> Result<String, AppError> {
    let sk = SymmetricKey::<V4>::from(secret_key).map_err(|_| AppError::InvalidKey)?;

    let mut claims = Claims::new().map_err(|_| AppError::TokenCreationFailed)?;
    claims
        .add_additional("user_id", user_id)
        .map_err(|_| AppError::TokenCreationFailed)?;
    let now = chrono::Utc::now();
    let expiration = now + chrono::Duration::hours(24);

    claims
        .expiration(&expiration.to_rfc3339())
        .map_err(|_| AppError::TokenCreationFailed)?;

    local::encrypt(&sk, &claims, None, Some(b"implicit assertion"))
        .map_err(|_| AppError::TokenCreationFailed)
}

pub fn verify_token(token: &str, secret_key: &[u8]) -> Result<Claims, AppError> {
    let sk = SymmetricKey::<V4>::from(secret_key).map_err(|_| AppError::InvalidKey)?;
    let validation_rules = ClaimsValidationRules::new();
    let untrusted =
        UntrustedToken::<Local, V4>::try_from(token).map_err(|_| AppError::TokenMalformed)?;

    let trusted = local::decrypt(
        &sk,
        &untrusted,
        &validation_rules,
        None,
        Some(b"implicit assertion"),
    )
    .map_err(|e| {
        if e.to_string().contains("expired") {
            AppError::TokenExpired
        } else {
            AppError::TokenInvalidSignature
        }
    })?;

    trusted
        .payload_claims()
        .cloned()
        .ok_or(AppError::TokenMalformed)
}
