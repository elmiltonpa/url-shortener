use crate::{
    error::AppError,
    models::google::{GoogleTokenResponse, GoogleUserInfo},
};
use reqwest::Client;
use serde_json::json;

pub struct GoogleAuthService {
    client_http: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl GoogleAuthService {
    pub fn new(
        client_http: Client,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            client_http,
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub async fn get_access_token(&self, code: String) -> Result<String, AppError> {
        let url = "https://oauth2.googleapis.com/token";

        let body = json!({
            "code": code,
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "redirect_uri": self.redirect_uri,
            "grant_type": "authorization_code"
        });

        let response = self.client_http.post(url).json(&body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response body".to_string());
            tracing::error!(
                "Google OAuth token exchange failed with status {}: {}",
                status,
                error_body
            );
            return Err(AppError::ValidationError(format!(
                "Google authentication failed: {}",
                status
            )));
        }

        let token_data = response.json::<GoogleTokenResponse>().await?;

        Ok(token_data.access_token)
    }

    pub async fn get_user_info(&self, access_token: String) -> Result<GoogleUserInfo, AppError> {
        let url = "https://www.googleapis.com/oauth2/v3/userinfo";

        let response = self
            .client_http
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response body".to_string());
            tracing::error!(
                "Google user info request failed with status {}: {}",
                status,
                error_body
            );
            return Err(AppError::ValidationError(format!(
                "Failed to retrieve Google user info: {}",
                status
            )));
        }

        let user_info = response.json::<GoogleUserInfo>().await?;

        Ok(user_info)
    }

    pub async fn authenticate(&self, code: String) -> Result<GoogleUserInfo, AppError> {
        let token = self.get_access_token(code).await?;
        let user_info = self.get_user_info(token).await?;

        Ok(user_info)
    }
}
