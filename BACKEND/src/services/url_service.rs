use crate::{
    error::{AppError, AppResult},
    models::url::{PaginationMeta, UrlModel, UrlStatsResponse},
    repository::url_repo::UrlRepository,
    services::{
        generator::CodeGenerator, safe_browsing::SafeBrowsingService, validator::UrlValidator,
    },
};
use chrono::{Duration, Utc};
use ipnetwork::IpNetwork;
use std::sync::Arc;
use uuid::Uuid;

pub struct UrlService {
    url_repository: UrlRepository,
    code_generator: CodeGenerator,
    safe_browsing: Arc<SafeBrowsingService>,
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
        user_id: Option<Uuid>,
    ) -> Result<UrlModel, AppError> {
        UrlValidator::validate(url, app_domain)?;

        self.safe_browsing.check_url(url).await?;

        let expiration_date = Utc::now() + Duration::days(30);
        let expires_at = Some(expiration_date);
        for _ in 1..=4 {
            let new_short_code = self.code_generator.generate();
            let create_url = self
                .url_repository
                .create_url(
                    self.url_repository.pool(),
                    url,
                    &new_short_code,
                    expires_at,
                    client_ip,
                    user_id,
                )
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
        let model = self
            .url_repository
            .get_url_by_code(self.url_repository.pool(), code)
            .await?;

        if let Some(value) = model.expires_at {
            if value < Utc::now() {
                return Err(AppError::Gone);
            }
        }

        let repo_clone = self.url_repository.clone();
        let code_string = code.to_string();
        let model_id = model.id;
        let user_agent = Self::truncate(user_agent, 512);
        let referrer = Self::truncate(referrer, 2048);

        tokio::spawn(async move {
            let mut tx = match repo_clone.pool().begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to start transaction for click tracking: {}", e);
                    return;
                }
            };

            if let Err(e) = repo_clone
                .increment_click_count(&mut *tx, &code_string)
                .await
            {
                tracing::error!("Failed to increment click count for {}: {}", code_string, e);
                return;
            }
            if let Err(e) = repo_clone
                .record_click(&mut *tx, model_id, user_agent, client_ip, referrer)
                .await
            {
                tracing::error!(
                    "Failed to record click analytics for {}: {}",
                    code_string,
                    e
                );
                return;
            }

            if let Err(e) = tx.commit().await {
                tracing::error!(
                    "Failed to commit click transaction for {}: {}",
                    code_string,
                    e
                );
            }
        });

        Ok(model.original_url)
    }

    pub async fn get_stats(
        &self,
        code: &str,
        app_domain: &str,
        limit: i64,
        offset: i64,
        caller_id: Uuid,
    ) -> AppResult<UrlStatsResponse> {
        let url = self
            .url_repository
            .get_url_by_code(self.url_repository.pool(), code)
            .await?;

        if let Some(owner_id) = url.user_id {
            if owner_id != caller_id {
                return Err(AppError::Forbidden);
            }
        } else {
            return Err(AppError::Forbidden);
        }

        let total_records = self.url_repository.count_stats(code).await?;

        let stats = self
            .url_repository
            .get_code_stats(self.url_repository.pool(), code, limit, offset)
            .await?;

        let short_url = format!("{}/{}", app_domain, url.short_code);
        let total_pages = (total_records + limit - 1) / limit;
        let current_page = (offset / limit) + 1;

        Ok(UrlStatsResponse {
            short_code: url.short_code,
            short_url,
            original_url: url.original_url,
            total_clicks: url.click_count,
            created_at: url.created_at,
            expires_at: url.expires_at,
            pagination: PaginationMeta {
                page: current_page,
                per_page: limit,
                total_records,
                total_pages,
            },
            stats,
        })
    }

    fn truncate(value: Option<String>, max_len: usize) -> Option<String> {
        value.map(|s| {
            if s.len() > max_len {
                s[..max_len].to_string()
            } else {
                s
            }
        })
    }
}
