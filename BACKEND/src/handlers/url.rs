use axum::{
    Extension, Json,
    extract::{Query, State},
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::url::PaginationQuery,
    state::AppState,
};

pub async fn list_user_urls(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<String>,
    Query(query): Query<PaginationQuery>,
) -> AppResult<impl IntoResponse> {
    let caller_id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::ValidationError("Invalid user ID in token".to_string()))?;

    let limit = query.per_page();
    let offset = query.offset();
    let app_domain = &state.config.app_domain;

    let result = state
        .url_service
        .get_user_urls(caller_id, app_domain, limit, offset)
        .await?;

    Ok(Json(result))
}
