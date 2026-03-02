use crate::{error::AppResult, models::user::UserModel};
use sqlx::{Executor, PgPool, Postgres, query_as, query_scalar};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn create_user<'a, E>(
        &self,
        executor: E,
        username: &str,
        email: &str,
        password_hash: Option<&str>,
        google_id: Option<&str>,
    ) -> AppResult<UserModel>
    where
        E: Executor<'a, Database = Postgres>,
    {
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
        .fetch_one(executor)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_email<'a, E>(
        &self,
        executor: E,
        email: &str,
    ) -> AppResult<Option<UserModel>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let user = query_as::<_, UserModel>(
            r#"
            SELECT id, email, username, password_hash, google_id, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(executor)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_google_id<'a, E>(
        &self,
        executor: E,
        google_id: &str,
    ) -> AppResult<Option<UserModel>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let user = query_as::<_, UserModel>(
            r#"
            SELECT id, email, username, password_hash, google_id, created_at
            FROM users
            WHERE google_id = $1
            "#,
        )
        .bind(google_id)
        .fetch_optional(executor)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id<'a, E>(
        &self,
        executor: E,
        user_id: Uuid,
    ) -> AppResult<Option<UserModel>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        let user = query_as::<_, UserModel>(
            r#"
            SELECT id, email, username, password_hash, google_id, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(executor)
        .await?;

        Ok(user)
    }

    pub async fn exists_by_email_or_username<'a, E>(
        &self,
        executor: E,
        email: &str,
        username: &str,
    ) -> AppResult<bool>
    where
        E: Executor<'a, Database = Postgres>,
    {
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
        .fetch_one(executor)
        .await?;

        Ok(exists)
    }

    pub async fn link_google_id<'a, E>(
        &self,
        executor: E,
        user_id: Uuid,
        google_id: &str,
    ) -> AppResult<UserModel>
    where
        E: Executor<'a, Database = Postgres>,
    {
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
        .fetch_one(executor)
        .await?;

        Ok(user)
    }
}
