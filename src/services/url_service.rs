use crate::{
    error::{AppError, AppResult},
    models::url::{UrlModel, UrlStatsResponse},
    repository::url_repo::UrlRepository,
    services::{
        generator::CodeGenerator, safe_browsing::SafeBrowsingService, validator::UrlValidator,
    },
};
use chrono::{Duration, Utc};
use ipnetwork::IpNetwork;
use std::sync::Arc;

pub struct UrlService {
    pub url_repository: UrlRepository,
    pub code_generator: CodeGenerator,
    pub safe_browsing: Arc<SafeBrowsingService>,
}

impl UrlService {
    pub fn new(
        repo: UrlRepository,
        generator: CodeGenerator,
        safe_browsing: Arc<SafeBrowsingService>,
    ) -> Self {
        Self {
            url_repository: repo,
            code_generator: generator,
            safe_browsing,
        }
    }

    pub async fn shorten_url(
        &self,
        url: &str,
        app_domain: &str,
        client_ip: Option<IpNetwork>,
    ) -> Result<UrlModel, AppError> {
        UrlValidator::validate(url, app_domain)?;

        self.safe_browsing.check_url(url).await?;

        let expiration_date = Utc::now() + Duration::days(30);
        let expires_at = Some(expiration_date);
        for _ in 1..=4 {
            let new_short_code = self.code_generator.generate();
            let create_url = self
                .url_repository
                .create_url(url, &new_short_code, expires_at, client_ip)
                .await;
            match create_url {
                Ok(model) => return Ok(model),
                Err(err) if err.is_unique_violation() => {
                    tracing::warn!(
                        "Collision detected for code: {}. Retrying...",
                        new_short_code
                    );
                    continue;
                }
                Err(e) => return Err(e),
            }
        }

        Err(AppError::Conflict)
    }

    pub async fn resolve_url(
        &self,
        code: &str,
        user_agent: Option<String>,
        client_ip: Option<IpNetwork>,
        referrer: Option<String>,
    ) -> Result<String, AppError> {
        let model = self.url_repository.get_url_by_code(code).await?;

        if let Some(value) = model.expires_at {
            if value < Utc::now() {
                return Err(AppError::Gone);
            }
        }

        let repo_clone = self.url_repository.clone();
        let code_string = code.to_string();
        tokio::spawn(async move {
            let _ = repo_clone.increment_click_count(&code_string).await;
            let _ = repo_clone
                .record_click(model.id, user_agent, client_ip, referrer)
                .await;
        });

        Ok(model.original_url)
    }

    pub async fn get_stats(&self, code: &str, app_domain: &str) -> AppResult<UrlStatsResponse> {
        let url = self.url_repository.get_url_by_code(code).await?;

        let stats = self.url_repository.get_code_stats(code).await?;

        let short_url = format!("{}/{}", app_domain, url.short_code);

        Ok(UrlStatsResponse {
            short_code: url.short_code,
            short_url: short_url,
            original_url: url.original_url,
            total_clicks: url.click_count,
            created_at: url.created_at,
            expires_at: url.expires_at,
            stats,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use sqlx::postgres::PgPoolOptions;
//     use std::time::Duration;
//     #[tokio::test]
//     async fn test_create_url_integration() {
//         let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let pool = PgPoolOptions::new()
//             .max_connections(5)
//             .acquire_timeout(Duration::from_secs(30))
//             .connect(&db_url)
//             .await
//             .unwrap();
//         let repo = UrlRepository::new(pool);
//         let generator = CodeGenerator::new(1, "123abc");
//         let service = UrlService::new(repo, generator);
//         let result = service
//             .shorten_url("https://google.com", "http://localhost", None)
//             .await;
//         assert!(result.is_ok());
//         let model = result.unwrap();
//         assert_eq!(model.original_url, "https://google.com");
//         println!("Short code generado: {}", model.short_code);
//     }
// }
