use crate::error::{AppError, AppResult};
use crate::models::url::UrlModel;
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use sqlx::{PgPool, query_as};

#[derive(Clone)]
pub struct UrlRepository {
    pool: PgPool,
}

impl UrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_url(
        &self,
        original_url: &str,
        short_code: &str,
        expires_at: Option<DateTime<Utc>>,
        created_by_ip: Option<IpNetwork>,
    ) -> AppResult<UrlModel> {
        let url: UrlModel = query_as::<_, UrlModel>(
            r#"
            INSERT INTO urls (original_url, short_code, created_by_ip, expires_at)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                original_url,
                short_code,
                click_count,
                created_by_ip,
                created_at,
                expires_at
            "#,
        )
        .bind(original_url)
        .bind(short_code)
        .bind(created_by_ip)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(url)
    }

    pub async fn get_url_by_code(&self, short_code: &str) -> AppResult<UrlModel> {
        let url: UrlModel = query_as::<_, UrlModel>(
            r#"
            SELECT
                id,
                original_url,
                short_code,
                click_count,
                created_by_ip,
                created_at,
                expires_at
            FROM urls
            WHERE short_code = $1
            "#,
        )
        .bind(short_code)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound)?;

        Ok(url)
    }

    pub async fn increment_click_count(&self, short_code: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE urls
            SET click_count = click_count + 1
            WHERE short_code = $1
            "#,
        )
        .bind(short_code)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
