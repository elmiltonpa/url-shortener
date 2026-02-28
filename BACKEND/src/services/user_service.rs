use crate::{
    error::{AppError, AppResult},
    models::user::{AuthResponse, UserModel, UserResponse},
    repository::user_repo::UserRepository,
    utils::{
        password::{hash_password, verify_password},
        token::create_token,
    },
};

pub struct UserService {
    pub user_repository: UserRepository,
    pub jwt_secret: String,
}

impl UserService {
    pub fn new(repo: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repository: repo,
            jwt_secret,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> AppResult<AuthResponse> {
        let user = self
            .user_repository
            .get_user_by_email(email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        let hash = user
            .password_hash
            .as_ref()
            .ok_or(AppError::ExternalAuthenticationRequired)?;

        let password_correct = verify_password(password, hash)?;

        if !password_correct {
            return Err(AppError::InvalidCredentials);
        }

        self.generate_auth_response(user).await
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> AppResult<AuthResponse> {
        let user_exists = self
            .user_repository
            .exists_by_email_or_username(email, username)
            .await?;

        if user_exists {
            return Err(AppError::UserAlreadyExists);
        };

        let password_hash = hash_password(password)?;

        let user_created = self
            .user_repository
            .create_user(username, email, Some(&password_hash), None)
            .await?;

        self.generate_auth_response(user_created).await
    }

    pub async fn authenticate_with_google(
        &self,
        google_id: &str,
        email: &str,
        username: &str,
    ) -> AppResult<AuthResponse> {
        let user = if let Some(user) = self
            .user_repository
            .get_user_by_google_id(google_id)
            .await?
        {
            user
        } else if let Some(user) = self.user_repository.get_user_by_email(email).await? {
            self.user_repository
                .link_google_id(user.id, google_id)
                .await?
        } else {
            let mut final_username = username.to_string();

            if self
                .user_repository
                .exists_by_email_or_username("", &final_username)
                .await?
            {
                let suffix = &nanoid::nanoid!(4);
                final_username = format!("{}_{}", final_username, suffix);
            }

            self.user_repository
                .create_user(&final_username, email, None, Some(google_id))
                .await?
        };

        self.generate_auth_response(user).await
    }

    async fn generate_auth_response(&self, user: UserModel) -> AppResult<AuthResponse> {
        let token = create_token(&user.id.to_string(), self.jwt_secret.as_bytes())?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                email: user.email,
                username: user.username,
            },
        })
    }
}
