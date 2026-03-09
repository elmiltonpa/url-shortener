use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::{AppError, AppResult},
    models::url::{UrlRequest, UrlResponse},
    state::AppState,
};

pub async fn create_url(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: axum::http::Request<axum::body::Body>,
) -> AppResult<impl IntoResponse> {
    let user_id = extract_optional_user_id(&req, &state);

    let body = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|_| AppError::ValidationError("Invalid request body".to_string()))?;

    let payload: UrlRequest =
        serde_json::from_slice(&body).map_err(|e| AppError::ValidationError(e.to_string()))?;

    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let client_ip = Some(addr.ip().into());
    let url_model = state
        .url_service
        .shorten_url(
            &payload.original_url,
            &state.config.app_domain,
            client_ip,
            user_id,
        )
        .await?;
    let short_url = format!("{}/{}", state.config.app_domain, url_model.short_code);

    let response = UrlResponse {
        short_code: url_model.short_code,
        original_url: url_model.original_url,
        short_url,
        click_count: url_model.click_count,
        created_at: url_model.created_at,
        expires_at: url_model.expires_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

fn extract_optional_user_id(
    req: &axum::http::Request<axum::body::Body>,
    state: &AppState,
) -> Option<Uuid> {
    let header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;

    let token = header.strip_prefix("Bearer ")?;
    let secret_key = state.config.paseto_key.as_bytes();

    let claims = crate::utils::token::verify_token(token, secret_key).ok()?;

    claims
        .get_claim("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
}
