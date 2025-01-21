use crate::domain::User;
use crate::repository::user::UserRepository;
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::async_trait;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

/// Claims are encoded in the JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: usize, // Expiration time (as a timestamp)
}

/// business logic for authorisation.
#[async_trait]
pub trait AuthenticationService: Send + Sync {
    /// Returns a JWT when credentials are valid.
    async fn login(&self, email: String, password: String) -> Result<Option<String>, sqlx::Error>;

    /// Checks if a JWT is valid given credentials.
    async fn verify_jwt(&self, token: &str) -> Result<Option<User>, sqlx::Error>;
}

/// AuthentcationServiceImpl requires:
/// User repository for recieving information about users.
/// The secret key for signing JWT's.
pub struct AuthenticationServiceImpl<U: UserRepository> {
    user_repository: U,
    secret_key: String,
}

impl<U: UserRepository> AuthenticationServiceImpl<U> {
    pub fn new(user_repository: U, secret_key: String) -> Self {
        Self {
            user_repository,
            secret_key,
        }
    }
}

// Implement the authentication service trait for AuthenticationServiceImpl.
#[async_trait]
impl<U: UserRepository> AuthenticationService for AuthenticationServiceImpl<U> {
    async fn login(&self, email: String, password: String) -> Result<Option<String>, sqlx::Error> {
        let user = match self.user_repository.from_email(email).await? {
            Some(user) => user,
            None => {
                return Ok(None);
            }
        };
        match verify_password(&password, &user.password) {
            true => Ok(generate_jwt(user.user_id, self.secret_key.clone())?),
            false => Ok(None),
        }
    }

    async fn verify_jwt(&self, token: &str) -> Result<Option<User>, sqlx::Error> {
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret_key.clone().as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims);

        match claims {
            Ok(claims) => Ok(self
                .user_repository
                .from_uuid(Uuid::from_str(&claims.user_id).expect("Failed to generate uuid."))
                .await?),
            Err(_) => Ok(None),
        }
    }
}

fn generate_jwt(user_id: Uuid, secret_key: String) -> Result<Option<String>, sqlx::Error> {
    // calculate experation time.
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Invalid time")
        .timestamp() as usize;

    let claims = Claims {
        user_id: user_id.to_string(),
        exp: expiration,
    };

    // generate jwt
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .expect("JWT creation failed");

    Ok(Some(token))
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    // Verify the password against the hashed password
    verify(password, hashed_password).unwrap_or(false)
}
