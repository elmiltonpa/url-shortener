use anyhow::Result;
use dotenvy::dotenv;

pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;
        let redis_url = std::env::var("REDIS_URL")?;
        let server_port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()?;

        Ok(Self {
            database_url,
            redis_url,
            server_port,
        })
    }
}
