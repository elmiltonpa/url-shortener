use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

pub fn config_cors(origin_url: &str) -> Result<CorsLayer, axum::http::header::InvalidHeaderValue> {
    let origin = origin_url.parse::<HeaderValue>()?;

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Ok(cors)
}
