use crate::{AppState, error::AppResult};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};
use std::sync::Arc;

pub async fn redirect(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> AppResult<impl IntoResponse> {
    let resolve_url = state.service.resolve_url(&code).await?;
    Ok(Redirect::temporary(&resolve_url))
}
