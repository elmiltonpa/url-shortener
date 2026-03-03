use crate::{state::AppState, utils::token::verify_token};
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(header) = auth_header {
        header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];
    let secret_key = state.config.paseto_key.as_bytes();

    let claims = verify_token(token, secret_key).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = claims
        .get_claim("user_id")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
