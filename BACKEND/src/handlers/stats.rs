use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::Arc;

use crate::{AppState, error::AppResult};

pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> AppResult<impl IntoResponse> {
    let stats = state
        .service
        .get_stats(&code, &state.config.app_domain)
        .await?;

    Ok(Json(stats))
}
