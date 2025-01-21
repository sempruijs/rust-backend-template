use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}
