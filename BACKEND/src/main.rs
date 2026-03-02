mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod repository;
mod services;
mod utils;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::{sync::Arc, time::Duration};
use tower_governor::GovernorLayer;

pub struct AppState {
    pub url_service: Arc<services::url_service::UrlService>,
    pub user_service: Arc<services::user_service::UserService>,
    pub config: Arc<config::Config>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let config = config::Config::from_env().expect("No se pudo cargar la config");
    let config = Arc::new(config);

    let cors_layer = middleware::cors::config_cors(&config.app_domain)
        .expect("No se pudo configurar CORS. APP_DOMAIN inválido.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(Some(Duration::from_secs(30)))
        .connect(&config.database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .expect("No se pudo crear el cliente HTTP");

    const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let url_repository = repository::url_repo::UrlRepository::new(pool.clone());
    let user_repository = repository::user_repo::UserRepository::new(pool.clone());
    let generator = services::generator::CodeGenerator::new(8, ALPHABET);
    let safe_browsing = Arc::new(services::safe_browsing::SafeBrowsingService::new(
        http_client,
        config.safe_browsing_api_key.clone(),
    ));

    let url_service =
        services::url_service::UrlService::new(url_repository, generator, safe_browsing);
    let url_service = Arc::new(url_service);

    let user_service =
        services::user_service::UserService::new(user_repository, config.jwt_secret.clone());
    let user_service = Arc::new(user_service);

    let state = Arc::new(AppState {
        url_service,
        user_service,
        config: config.clone(),
    });

    let governor_conf = Arc::new(middleware::rate_limit::build_rate_limit_config());

    let app = Router::new()
        .route("/", post(handlers::create::create_url))
        .route("/{code}", get(handlers::redirect::redirect))
        .route("/stats/{code}", get(handlers::stats::get_stats))
        .layer(GovernorLayer::new(governor_conf))
        .layer(cors_layer)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server corriendo en http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}
