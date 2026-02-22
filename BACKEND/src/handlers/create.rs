use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::net::SocketAddr;
use std::sync::Arc;
use validator::Validate;

use crate::{
    AppState,
    error::{AppError, AppResult},
    models::url::{UrlRequest, UrlResponse},
};

pub async fn create_url(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<UrlRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let client_ip = Some(addr.ip().into());
    let url_model = state
        .service
        .shorten_url(&payload.original_url, &state.config.app_domain, client_ip)
        .await?;
    let short_url = format!("{}/{}", state.config.app_domain, url_model.short_code);

    let response = UrlResponse {
        short_code: url_model.short_code,
        original_url: url_model.original_url,
        short_url,
        created_at: url_model.created_at,
        expires_at: url_model.expires_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
