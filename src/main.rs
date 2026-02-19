mod config;
mod error;
mod handlers;
mod models;
mod repository;
mod services;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct AppState {
    pub service: Arc<services::url_service::UrlService>,
    pub config: Arc<config::Config>,
}

#[tokio::main]
async fn main() {
    let config = config::Config::from_env().expect("No se pudo cargar la config");
    let config = Arc::new(config);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    const ALPHABET: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let repository = repository::url_repo::UrlRepository::new(pool);
    let generator = services::generator::CodeGenerator::new(8, ALPHABET);

    let service = services::url_service::UrlService::new(repository, generator);
    let service = Arc::new(service);

    let state = Arc::new(AppState {
        service,
        config: config.clone(),
    });

    let app = Router::new()
        .route("/", post(handlers::create::create_url))
        .route("/:code", get(handlers::redirect::redirect))
        .route("/stats/:code", get(handlers::stats::get_stats))
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
