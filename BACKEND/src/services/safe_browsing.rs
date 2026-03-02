use crate::error::AppError;
use crate::models::safe_url::{
    ClientInfo, SafeBrowsingRequest, SafeBrowsingResponse, ThreatEntry, ThreatInfo, ThreatType,
};
use reqwest::Client;

pub struct SafeBrowsingService {
    client: Client,
    api_key: String,
}

impl SafeBrowsingService {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }

    pub async fn check_url(&self, url_to_check: &str) -> Result<(), AppError> {
        let api_url = format!(
            "https://safebrowsing.googleapis.com/v4/threatMatches:find?key={}",
            self.api_key
        );

        let payload = SafeBrowsingRequest {
            client: ClientInfo {
                client_id: "url-shortener-rust".to_string(),
                client_version: "1.0.0".to_string(),
            },
            threat_info: ThreatInfo {
                threat_types: vec![
                    ThreatType::Malware,
                    ThreatType::SocialEngineering,
                    ThreatType::UnwantedSoftware,
                ],
                platform_types: vec!["ANY_PLATFORM".to_string()],
                threat_entry_types: vec!["URL".to_string()],
                threat_entries: vec![ThreatEntry {
                    url: url_to_check.to_string(),
                }],
            },
        };

        let response = self
            .client
            .post(api_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Network error with Google Safe Browsing: {}", e);
                AppError::Internal(anyhow::anyhow!(e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            tracing::error!("Google Safe Browsing API error: {}", status);
            return Err(AppError::Internal(anyhow::anyhow!(
                "Safe Browsing API returned status: {}",
                status
            )));
        }

        let result: SafeBrowsingResponse = response
            .json()
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

        if let Some(matches) = result.matches {
            if !matches.is_empty() {
                for m in &matches {
                    tracing::warn!(
                        "Blocked malicious URL. Type: {:?}, Platform: {}",
                        m.threat_type,
                        m.platform_type
                    );
                }
                return Err(AppError::UrlMalicious);
            }
        }

        Ok(())
    }
}
