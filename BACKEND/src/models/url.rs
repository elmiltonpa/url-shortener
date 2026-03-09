use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UrlModel {
    pub id: i64,
    pub original_url: String,
    pub short_code: String,
    pub click_count: i64,
    pub created_by_ip: Option<IpNetwork>,
    pub user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct ClaimUrlsRequest {
    pub codes: Vec<String>,
}

#[derive(Serialize)]
pub struct ClaimUrlsResponse {
    pub success: bool,
    pub message: String,
    pub count: u64,
}

#[derive(Deserialize, Validate)]
pub struct UrlRequest {
    #[validate(
        url,
        length(max = 2048, message = "URL must not exceed 2048 characters")
    )]
    pub original_url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub short_code: String,
    pub original_url: String,
    pub short_url: String,
    pub click_count: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct UrlListResponse {
    pub urls: Vec<UrlResponse>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StatModel {
    pub id: i64,
    pub url_id: i64,
    pub user_agent: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub referrer: Option<String>,
    pub country_code: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl PaginationQuery {
    const DEFAULT_PAGE: i64 = 1;
    const DEFAULT_PER_PAGE: i64 = 20;
    const MAX_PER_PAGE: i64 = 100;

    pub fn page(&self) -> i64 {
        self.page.unwrap_or(Self::DEFAULT_PAGE).max(1)
    }

    pub fn per_page(&self) -> i64 {
        self.per_page
            .unwrap_or(Self::DEFAULT_PER_PAGE)
            .clamp(1, Self::MAX_PER_PAGE)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.per_page()
    }
}

#[derive(Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub per_page: i64,
    pub total_records: i64,
    pub total_pages: i64,
}

#[derive(Serialize)]
pub struct UrlStatsResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
    pub total_clicks: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub pagination: PaginationMeta,
    pub stats: Vec<StatModel>,
}
