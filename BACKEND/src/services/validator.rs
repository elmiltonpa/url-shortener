use crate::error::AppError;
use url::Url;

pub struct UrlValidator;

impl UrlValidator {
    pub fn validate(url: &str, app_domain: &str) -> Result<String, AppError> {
        let parsed_url = Url::parse(url)
            .map_err(|_| AppError::ValidationError("Invalid URL format".to_string()))?;

        let parsed_app_domain = Url::parse(app_domain).map_err(|_| {
            AppError::ValidationError("Invalid app_domain configuration".to_string())
        })?;

        if parsed_url.host_str() == parsed_app_domain.host_str()
            && parsed_url.port() == parsed_app_domain.port()
        {
            return Err(AppError::ValidationError(
                "Cannot shorten URLs from this domain".to_string(),
            ));
        }

        let scheme = parsed_url.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(AppError::ValidationError(
                "Only http and https protocols are allowed".to_string(),
            ));
        }
        if parsed_url.host_str().is_none() {
            return Err(AppError::ValidationError(
                "URL must have a valid host".to_string(),
            ));
        }
        Ok(parsed_url.to_string())
    }
}
