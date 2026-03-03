use anyhow::Result;
use dotenvy::dotenv;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub app_domain: String,
    pub safe_browsing_api_key: String,
    pub paseto_key: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;
        let server_port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()?;
        let app_domain =
            std::env::var("APP_DOMAIN").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let safe_browsing_api_key = std::env::var("SAFE_BROWSING_API_KEY")?;
        let paseto_key = std::env::var("PASETO_KEY")?;
        let google_client_id = std::env::var("GOOGLE_CLIENT_ID")?;
        let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET")?;
        let google_redirect_uri = std::env::var("GOOGLE_REDIRECT_URI")?;
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "".to_string());

        Ok(Self {
            database_url,
            server_port,
            app_domain,
            safe_browsing_api_key,
            paseto_key,
            google_client_id,
            google_client_secret,
            google_redirect_uri,
            redis_url,
        })
    }
}
