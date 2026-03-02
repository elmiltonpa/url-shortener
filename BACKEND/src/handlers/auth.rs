use crate::{
    error::AppResult,
    models::user::{AuthResponse, GoogleAuthRequest, LoginRequest, RegisterRequest},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use validator::Validate;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| crate::error::AppError::ValidationError(e.to_string()))?;

    let response: AuthResponse = state
        .user_service
        .register(&payload.username, &payload.email, &payload.password)
        .await?;

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    payload
        .validate()
        .map_err(|e| crate::error::AppError::ValidationError(e.to_string()))?;

    let response = state
        .user_service
        .login(&payload.email, &payload.password)
        .await?;

    Ok((StatusCode::OK, Json(response)))
}

pub async fn google_auth(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GoogleAuthRequest>,
) -> AppResult<impl IntoResponse> {
    let user_info = state.google_service.authenticate(payload.code).await?;
    let response = state
        .user_service
        .authenticate_with_google(&user_info.sub, &user_info.email, &user_info.name)
        .await?;

    Ok((StatusCode::OK, Json(response)))
}
