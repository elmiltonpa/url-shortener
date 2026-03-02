use crate::{error::AppResult, state::AppState};
use axum::{
    extract::{ConnectInfo, Path, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect},
};
use std::{net::SocketAddr, sync::Arc};

pub async fn redirect(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> AppResult<impl IntoResponse> {
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let referrer = headers
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let ip_address = Some(addr.ip().into());
    let resolve_url = state
        .url_service
        .resolve_url(&code, user_agent, ip_address, referrer)
        .await?;
    Ok(Redirect::permanent(&resolve_url))
}
