use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub name: String,
    // pub given_name: String,
    // pub family_name: String,
    // pub picture: String,
    pub email: String,
    // pub email_verified: bool,
}
