use crate::{
    AppState,
    error::AppResult,
    models::user::{AuthResponse, GoogleAuthRequest, LoginRequest, RegisterRequest},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
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
    let response = state
        .user_service
        .login(&payload.email, &payload.password)
        .await?;

    Ok((StatusCode::OK, Json(response)))
}

// pub async fn google_auth(
//     State(state): State<Arc<AppState>>,
//     Json(payload): Json<GoogleAuthRequest>,
// ) -> AppResult<impl IntoResponse> {
// }
