use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;

use crate::state::AppState;

pub async fn health_check(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();

    let status = if db_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    let body = serde_json::json!({
        "status": if db_ok { "healthy" } else { "unhealthy" },
        "database": if db_ok { "connected" } else { "disconnected" },
    });

    (status, Json(body))
}
