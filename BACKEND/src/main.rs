mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod repository;
mod services;
mod state;
mod utils;

use crate::middleware::auth::auth_middleware;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Duration};
use tower_governor::GovernorLayer;
use tower_http::{
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::Span;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let config = config::Config::from_env().expect("Failed to load configuration");
    let config = Arc::new(config);

    let cors_layer = middleware::cors::config_cors(&config.app_domain)
        .expect("Failed to configure CORS. Invalid APP_DOMAIN.");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(15))
        .idle_timeout(Some(Duration::from_secs(30)))
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Failed to create HTTP client");

    let url_repository = repository::url_repo::UrlRepository::new(pool.clone());
    let user_repository = repository::user_repo::UserRepository::new(pool.clone());

    let generator = services::generator::CodeGenerator::new(8);

    let safe_browsing = Arc::new(services::safe_browsing::SafeBrowsingService::new(
        http_client.clone(),
        config.safe_browsing_api_key.clone(),
    ));

    let url_service =
        services::url_service::UrlService::new(url_repository, generator, safe_browsing);
    let url_service = Arc::new(url_service);

    let user_service =
        services::user_service::UserService::new(user_repository, config.paseto_key.clone());
    let user_service = Arc::new(user_service);

    let google_service = services::google_auth::GoogleAuthService::new(
        http_client,
        config.google_client_id.clone(),
        config.google_client_secret.clone(),
        config.google_redirect_uri.clone(),
    );
    let google_service = Arc::new(google_service);

    let state = Arc::new(AppState {
        pool: pool.clone(),
        url_service,
        user_service,
        google_service,
        config: config.clone(),
    });

    let governor_conf = Arc::new(middleware::rate_limit::build_rate_limit_config());

    let private_routes = Router::new()
        .route("/stats/{code}", get(handlers::stats::get_stats))
        .route("/user/urls", get(handlers::url::list_user_urls))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));
    let x_request_id = axum::http::HeaderName::from_static("x-request-id");

    let app = Router::new()
        .merge(private_routes)
        .route("/", post(handlers::create::create_url))
        .route("/{code}", get(handlers::redirect::redirect))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/google", post(handlers::auth::google_auth))
        .layer(GovernorLayer::new(governor_conf))
        .route("/health", get(handlers::health::health_check))
        .layer(cors_layer)
        .layer(RequestBodyLimitLayer::new(1_048_576)) // 1 MB global body limit
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("unknown");

                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        request_id = %request_id,
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>, latency: Duration, _span: &Span| {
                        tracing::info!(
                            status = %response.status(),
                            latency_ms = %latency.as_millis(),
                            "response"
                        );
                    },
                ),
        )
        .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Background job: clean expired URLs every hour
    let cleanup_repo = repository::url_repo::UrlRepository::new(pool.clone());
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            match cleanup_repo.delete_expired_urls().await {
                Ok(count) if count > 0 => {
                    tracing::info!("Cleaned up {} expired URLs and their analytics", count);
                }
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Failed to clean up expired URLs: {}", e);
                }
            }
        }
    });

    tracing::info!("Server running on http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .unwrap();

    tracing::info!("Server shut down gracefully");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Received Ctrl+C, starting graceful shutdown..."),
        _ = terminate => tracing::info!("Received SIGTERM, starting graceful shutdown..."),
    }
}
