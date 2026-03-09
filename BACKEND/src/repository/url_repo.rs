use crate::error::{AppError, AppResult};
use crate::models::url::{StatModel, UrlModel};
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use sqlx::{Executor, PgPool, Postgres, query_as};
use uuid::Uuid;

#[derive(Clone)]
pub struct UrlRepository {
    pool: PgPool,
}

impl UrlRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn create_url<'a, E>(
        &self,
        executor: E,
        original_url: &str,
        short_code: &str,
        expires_at: Option<DateTime<Utc>>,
        created_by_ip: Option<IpNetwork>,
        user_id: Option<Uuid>,
    ) -> AppResult<UrlModel>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let url: UrlModel = query_as::<_, UrlModel>(
            r#"
            INSERT INTO urls (original_url, short_code, created_by_ip, expires_at, user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                id,
                original_url,
                short_code,
                click_count,
                created_by_ip,
                user_id,
                created_at,
                expires_at
            "#,
        )
        .bind(original_url)
        .bind(short_code)
        .bind(created_by_ip)
        .bind(expires_at)
        .bind(user_id)
        .fetch_one(executor)
        .await?;

        Ok(url)
    }

    pub async fn get_url_by_code<'a, E>(&self, executor: E, short_code: &str) -> AppResult<UrlModel>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let url: UrlModel = query_as::<_, UrlModel>(
            r#"
            SELECT
                id,
                original_url,
                short_code,
                click_count,
                created_by_ip,
                user_id,
                created_at,
                expires_at
            FROM urls
            WHERE short_code = $1
            "#,
        )
        .bind(short_code)
        .fetch_optional(executor)
        .await?
        .ok_or(AppError::NotFound)?;

        Ok(url)
    }

    pub async fn increment_click_count<'a, E>(&self, executor: E, short_code: &str) -> AppResult<()>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query(
            r#"
            UPDATE urls
            SET click_count = click_count + 1
            WHERE short_code = $1
            "#,
        )
        .bind(short_code)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn record_click<'a, E>(
        &self,
        executor: E,
        url_id: i64,
        user_agent: Option<String>,
        ip_address: Option<IpNetwork>,
        referrer: Option<String>,
    ) -> AppResult<()>
    where
        E: Executor<'a, Database = Postgres>,
    {
        sqlx::query(
            r#"
            INSERT INTO url_analytics (url_id, user_agent, ip_address, referrer)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(url_id)
        .bind(user_agent)
        .bind(ip_address)
        .bind(referrer)
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn get_code_stats<'a, E>(
        &self,
        executor: E,
        short_code: &str,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<StatModel>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let stats: Vec<StatModel> = query_as::<_, StatModel>(
            r#"
            SELECT
                ua.id,
                ua.url_id,
                ua.user_agent,
                ua.ip_address,
                ua.referrer,
                ua.country_code,
                ua.created_at
            FROM url_analytics ua
            JOIN urls u ON ua.url_id = u.id
            WHERE u.short_code = $1
            ORDER BY ua.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(short_code)
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .await?;
        Ok(stats)
    }

    pub async fn count_stats(&self, short_code: &str) -> AppResult<i64> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM url_analytics ua
            JOIN urls u ON ua.url_id = u.id
            WHERE u.short_code = $1
            "#,
        )
        .bind(short_code)
        .fetch_one(self.pool())
        .await?;
        Ok(row.0)
    }

    pub async fn delete_expired_urls(&self) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            WITH expired AS (
                SELECT id FROM urls
                WHERE expires_at IS NOT NULL AND expires_at < NOW()
            ),
            deleted_analytics AS (
                DELETE FROM url_analytics
                WHERE url_id IN (SELECT id FROM expired)
            )
            DELETE FROM urls
            WHERE id IN (SELECT id FROM expired)
            "#,
        )
        .execute(self.pool())
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_user_urls<'a, E>(
        &self,
        executor: E,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<UrlModel>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let urls: Vec<UrlModel> = query_as::<_, UrlModel>(
            r#"
            SELECT
                id,
                original_url,
                short_code,
                click_count,
                created_by_ip,
                user_id,
                created_at,
                expires_at
            FROM urls
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .await?;
        Ok(urls)
    }

    pub async fn count_user_urls(&self, user_id: Uuid) -> AppResult<i64> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM urls
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(self.pool())
        .await?;
        Ok(row.0)
    }

    pub async fn update_links(&self, user_id: Uuid, short_codes: Vec<String>) -> AppResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE urls
            SET user_id = $1
            WHERE short_code = ANY($2)
            AND user_id IS NULL;
            "#,
        )
        .bind(user_id)
        .bind(short_codes)
        .execute(self.pool())
        .await?;

        Ok(result.rows_affected())
    }
}
