use crate::{config, services};
use std::sync::Arc;

pub struct AppState {
    pub url_service: Arc<services::url_service::UrlService>,
    pub user_service: Arc<services::user_service::UserService>,
    pub google_service: Arc<services::google_auth::GoogleAuthService>,
    pub config: Arc<config::Config>,
}
