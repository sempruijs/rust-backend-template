use crate::domain::User;
use crate::repository::user::*;
use bcrypt::hash;
use rocket::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserService: Send + Sync {
    // should be a service error.
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error>;

    async fn delete(&self, user_id: Uuid) -> Result<u64, sqlx::Error>;
}

pub struct UserServiceImpl<T: UserRepository> {
    user_repository: T,
}

impl<R: UserRepository> UserServiceImpl<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<R: UserRepository> UserService for UserServiceImpl<R> {
    async fn delete(&self, user_id: Uuid) -> Result<u64, sqlx::Error> {
        self.user_repository.delete(user_id).await
    }

    // should be a service error.
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        let mut user = user;
        user.password = hash_password(&user.password);
        self.user_repository.create(user).await
    }

    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_repository.get_all().await
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        self.user_repository.from_uuid(user_id).await
    }
}

pub fn hash_password(password: &str) -> String {
    // Generate a hashed password
    hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password")
}

#[cfg(test)]
mod tests {
    use crate::service::user::*;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    #[ignore]
    async fn test_crud_user() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect_lazy(&database_url).expect("Failed to connect to the database");

        let user_repository = UserRepositoryImpl::new(pool.clone());
        let user_service = UserServiceImpl::new(user_repository.clone());

        let user = User {
            name: String::from("Bob"),
            email: String::from("bob@example.com"),
            date_of_birth: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            password: String::from("alice"),
            user_id: Uuid::new_v4(),
        };

        let create = user_service.create(user.clone()).await;
        assert!(create.is_ok());

        let get_user = user_service.from_uuid(user.user_id).await.unwrap().unwrap();
        assert_eq!(user.name, get_user.name);
        assert_eq!(user.email, get_user.email);
        assert_eq!(user.date_of_birth, get_user.date_of_birth);
        assert_ne!(user.password, get_user.password);

        let delete_user = user_service.delete(user.user_id).await;
        assert!(delete_user.is_ok());

        let get_user = user_service.from_uuid(user.user_id).await.unwrap();
        assert!(get_user.is_none());
    }
}
