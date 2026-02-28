use crate::{
    error::{AppError, AppResult},
    models::user::UserModel,
};
use sqlx::{PgPool, query_as, query_scalar};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: Option<&str>,
        google_id: Option<&str>,
    ) -> AppResult<UserModel> {
        let user = query_as::<_, UserModel>(
            r#"
            INSERT INTO users (username, email, password_hash, google_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, username, password_hash, google_id, created_at
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(google_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> AppResult<Option<UserModel>> {
        let user = query_as::<_, UserModel>(
            r#"
            SELECT id, email, username, password_hash, google_id, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_google_id(&self, google_id: &str) -> AppResult<Option<UserModel>> {
        let user = query_as::<_, UserModel>(
            r#"
            SELECT id, email, username, password_hash, google_id, created_at
            FROM users
            WHERE google_id = $1
            "#,
        )
        .bind(google_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> AppResult<Option<UserModel>> {
        let user = query_as::<_, UserModel>(
            r#"
            SELECT id, email, username, password_hash, google_id, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn exists_by_email_or_username(
        &self,
        email: &str,
        username: &str,
    ) -> AppResult<bool> {
        let exists = query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users
                WHERE email = $1 OR username = $2
            )
            "#,
        )
        .bind(email)
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    pub async fn link_google_id(&self, user_id: Uuid, google_id: &str) -> AppResult<UserModel> {
        let user = query_as::<_, UserModel>(
            r#"
            UPDATE users
            SET google_id = $2
            WHERE id = $1
            RETURNING id, email, username, password_hash, google_id, created_at
            "#,
        )
        .bind(user_id)
        .bind(google_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
