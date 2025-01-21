use crate::domain::User;
use rocket::async_trait;
use sqlx::types::chrono::NaiveDate;
use sqlx::types::Uuid;
use sqlx::PgPool;
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn delete(&self, user_id: Uuid) -> Result<u64, sqlx::Error>;

    async fn from_email(&self, email: String) -> Result<Option<User>, sqlx::Error>;

    async fn allowed_on_date(&self, user_id: Uuid, date: NaiveDate) -> Result<bool, sqlx::Error>;

    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error>;
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn delete(&self, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM users WHERE user_id = $1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT user_id, name, date_of_birth, email, password
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }

    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (user_id, name, date_of_birth, email, password)
             VALUES ($1, $2, $3, $4, $5)",
            user.user_id,
            user.name,
            user.date_of_birth,
            user.email,
            user.password,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn allowed_on_date(&self, user_id: Uuid, date: NaiveDate) -> Result<bool, sqlx::Error> {
        // Query to check if a ticket exists for the given user and date range
        let query_result = sqlx::query_scalar!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM tickets
                WHERE user_id = $1
                  AND $2 BETWEEN start_date AND end_date
            )
            "#,
            user_id,
            date
        )
        .fetch_one(&self.pool) // Use `fetch_one` because `EXISTS` always returns one row
        .await?;

        Ok(query_result.unwrap_or(false)) // Return the result, default to false if NULL
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, date_of_birth, email, password
             FROM users
             WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn from_email(&self, email: String) -> Result<Option<User>, sqlx::Error> {
        // Query the database for a user by email
        let query_result = sqlx::query_as!(
            User,
            r#"
            SELECT user_id, name, date_of_birth, email, password
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool) // Use `fetch_optional` to get an Option<User>
        .await?;

        Ok(query_result)
    }
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}
