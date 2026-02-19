use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UrlModel {
    pub id: i64,
    pub original_url: String,
    pub short_code: String,
    pub click_count: i32,
    pub created_by_ip: Option<IpNetwork>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Validate)]
pub struct UrlRequest {
    #[validate(url)]
    pub original_url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub short_code: String,
    pub original_url: String,
    pub short_url: String, //URL COMPLETA
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
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

#[derive(Serialize)]
pub struct UrlStatsResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
    pub total_clicks: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub stats: Vec<StatModel>,
}
