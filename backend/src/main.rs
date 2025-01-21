use crate::controller::activity::*;
use crate::controller::authentication::authentication_routes;
use crate::controller::ticket::*;
use crate::domain::User;
use crate::repository::activity::*;
use crate::repository::ticket::*;
use crate::service::activity::*;
use crate::service::authentication::*;
use crate::service::place::*;
use crate::service::ticket::*;
use crate::service::user::UserService;
use crate::AuthenticationService;
use dotenv::dotenv;
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Config;
use rocket::Request;
use rocket::State;
use rocket_cors::AllowedOrigins;
use rocket_cors::{AllowedHeaders, CorsOptions};
use std::env;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
extern crate rocket;
use crate::controller::place::*;
use crate::controller::user::*;
use crate::docs::ApiDoc;
use crate::repository::place::PlaceRepositoryImpl;
use crate::repository::user::UserRepositoryImpl;
use crate::service::user::UserServiceImpl;
use sqlx::PgPool;

pub mod controller;
pub mod docs;
pub mod domain;
pub mod repository;
pub mod service;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let authentication_service = request
            .guard::<&State<Arc<dyn AuthenticationService>>>()
            .await
            .unwrap();

        match request.headers().get_one("Authorization") {
            None => {
                println!("hello");
                Outcome::Error((Status::Unauthorized, ()))
            }
            Some(autherisation_header) => match autherisation_header.strip_prefix("Bearer ") {
                None => Outcome::Error((Status::Unauthorized, ())),
                Some(jwt) => match authentication_service.verify_jwt(jwt).await {
                    Ok(Some(user)) => request::Outcome::Success(user.clone()),
                    Ok(None) => {
                        println!("hello2");
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                    Err(_) => {
                        println!("hello3");
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                },
            },
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Read the database path from enviousment variables.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = env::var("DINO_SECRET_KEY").expect("DINO_SECRET_KEY must be set");

    let pool = PgPool::connect_lazy(&database_url).expect("Failed to connect to the database");

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(), // Allow all origins
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS configuration");

    // Build layers
    let user_repository = UserRepositoryImpl::new(pool.clone());
    let user_service: Arc<dyn UserService> =
        Arc::new(UserServiceImpl::new(user_repository.clone()));

    let authentication_service: Arc<dyn AuthenticationService> = Arc::new(
        AuthenticationServiceImpl::new(user_repository.clone(), secret_key),
    );

    // Create a custom Rocket configuration
    let config = Config {
        port: 8000,
        address: "0.0.0.0".parse().expect("Invalid address"),
        ..Config::debug_default() // Use default values for other settings
    };

    let _rocket = rocket::custom(config)
        .manage(user_service)
        .manage(authentication_service)
        .mount(
            "/",
            SwaggerUi::new("/docs/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/users", user_routes())
        .mount("/login", authentication_routes())
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
