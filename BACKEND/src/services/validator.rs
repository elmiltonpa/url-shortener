use crate::error::AppError;
use url::Url;

pub struct UrlValidator;

impl UrlValidator {
    pub fn validate(url: &str, app_domain: &str) -> Result<String, AppError> {
        let parsed_url = Url::parse(url)
            .map_err(|_| AppError::ValidationError("Invalid URL format".to_string()))?;

        let scheme = parsed_url.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(AppError::ValidationError(
                "Only http and https protocols are allowed".to_string(),
            ));
        }

        let host = parsed_url
            .host_str()
            .ok_or_else(|| AppError::ValidationError("URL must have a valid host".to_string()))?;

        let parts: Vec<&str> = host.split('.').collect();

        if parts.len() < 2 {
            return Err(AppError::ValidationError(
                "URL must include a domain and a TLD (e.g., .com)".to_string(),
            ));
        }

        let tld = parts.last().unwrap();
        if tld.len() < 2 {
            return Err(AppError::ValidationError(
                "The TLD is too short to be a valid public domain".to_string(),
            ));
        }

        if let Ok(parsed_app_domain) = Url::parse(app_domain) {
            if host == parsed_app_domain.host_str().unwrap_or("") {
                return Err(AppError::ValidationError(
                    "Cannot shorten URLs from this domain".to_string(),
                ));
            }
        }

        Ok(parsed_url.to_string())
    }
}
