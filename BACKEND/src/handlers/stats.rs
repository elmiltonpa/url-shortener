use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::url::StatsQuery,
    state::AppState,
};

pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<String>,
    Path(code): Path<String>,
    Query(query): Query<StatsQuery>,
) -> AppResult<impl IntoResponse> {
    let caller_id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::ValidationError("Invalid user ID in token".to_string()))?;

    let stats = state
        .url_service
        .get_stats(
            &code,
            &state.config.app_domain,
            query.per_page(),
            query.offset(),
            caller_id,
        )
        .await?;

    Ok(Json(stats))
}
