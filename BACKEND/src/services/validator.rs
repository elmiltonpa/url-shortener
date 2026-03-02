use crate::error::AppError;
use std::net::{IpAddr, Ipv6Addr};
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

        let host_lower = host.to_lowercase();
        if host_lower == "localhost" || host_lower == "127.0.0.1" || host_lower == "::1" {
            return Err(AppError::ValidationError(
                "Cannot shorten URLs pointing to localhost".to_string(),
            ));
        }
        if let Ok(ip) = host.parse::<IpAddr>() {
            if Self::is_private_ip(ip) {
                return Err(AppError::ValidationError(
                    "Cannot shorten URLs pointing to private IP addresses".to_string(),
                ));
            }
        }

        if host.parse::<IpAddr>().is_err() {
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
        }
        if let Ok(parsed_app_domain) = Url::parse(app_domain) {
            if host_lower == parsed_app_domain.host_str().unwrap_or("").to_lowercase() {
                return Err(AppError::ValidationError(
                    "Cannot shorten URLs from this domain".to_string(),
                ));
            }
        }

        Ok(parsed_url.to_string())
    }

    fn is_private_ip(ip: IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                ipv4.is_loopback()
                    || ipv4.is_private()
                    || ipv4.is_link_local()
                    || ipv4.is_broadcast()
                    || ipv4.is_documentation()
                    || ipv4.is_unspecified()
            }
            IpAddr::V6(ipv6) => {
                if let Some(mapped_ipv4) = ipv6.to_ipv4_mapped() {
                    return Self::is_private_ip(IpAddr::V4(mapped_ipv4));
                }

                ipv6.is_loopback()
                    || ipv6.is_unspecified()
                    || Self::is_ipv6_unique_local(&ipv6)
                    || Self::is_ipv6_link_local(&ipv6)
            }
        }
    }

    fn is_ipv6_unique_local(ipv6: &Ipv6Addr) -> bool {
        (ipv6.segments()[0] & 0xfe00) == 0xfc00
    }

    fn is_ipv6_link_local(ipv6: &Ipv6Addr) -> bool {
        (ipv6.segments()[0] & 0xffc0) == 0xfe80
    }
}
