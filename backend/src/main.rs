use crate::controller::authentication::authentication_routes;
use crate::domain::User;
use crate::service::authentication::*;
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
use crate::controller::user::*;
use crate::docs::ApiDoc;
use crate::repository::user::UserRepositoryImpl;
use crate::service::user::UserServiceImpl;
use sqlx::PgPool;

// Import all the different layers that make up the backend.
pub mod controller;
pub mod docs;
pub mod domain;
pub mod repository;
pub mod service;

// This is responsible for protecting the endpoints with JWT.
//
// If an endpoint requires a User as argument, this piece of code is called.
// It reades the header if a JWT is given.
// it reades the user_id from the jwt and recieves the user from the database.
// Then that user is being parsed into the endpoint.
//
// If the JWT is not in the header, it returns access denied.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Recieve the authentication service from rocket for recieving and validating the jwt.
        let authentication_service = request
            .guard::<&State<Arc<dyn AuthenticationService>>>()
            .await
            .unwrap();

        match request.headers().get_one("Authorization") {
            None => Outcome::Error((Status::Unauthorized, ())),
            Some(autherisation_header) => match autherisation_header.strip_prefix("Bearer ") {
                None => Outcome::Error((Status::Unauthorized, ())),
                Some(jwt) => match authentication_service.verify_jwt(jwt).await {
                    Ok(Some(user)) => request::Outcome::Success(user.clone()),
                    Ok(None) => Outcome::Error((Status::Unauthorized, ())),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                },
            },
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Read the database path from enviousment variables.
    // You can set them in bash like so:
    // ```bash
    // export DATABASE_URL="postgres://username:password@host:port/database_name"
    // ```
    //
    // And same for the secret key that is being used to sign the JWT.
    //
    // ```bash
    // export SECRET_KEY="My secret key :)"
    // ```
    // the program will panic if these are not set.
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRETKEY must be set for generating JWT");

    // Connect to postgres database.
    let pool = PgPool::connect_lazy(&database_url).expect("Failed to connect to the database");

    // Alow request from any origin.
    // You should customize this if you want to make your backend more secure.
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(), // Allow all origins
        allowed_headers: AllowedHeaders::some(&["Authorization", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS configuration");

    // Build the repository layers and service layers.
    let user_repository = UserRepositoryImpl::new(pool.clone());
    let user_service: Arc<dyn UserService> =
        Arc::new(UserServiceImpl::new(user_repository.clone()));

    let authentication_service: Arc<dyn AuthenticationService> = Arc::new(
        AuthenticationServiceImpl::new(user_repository.clone(), secret_key),
    );

    // Add here more repositories and services when your backend grows.

    // Set rocket configuration.
    let config = Config {
        port: 8000,
        address: "0.0.0.0".parse().expect("Invalid address"),
        ..Config::debug_default()
    };

    // And combine everything into your beautiful backend.
    let _rocket = rocket::custom(config)
        // Here the service layers are given as arguments to the endpoints.
        // Add more service layers when you backend grows.
        .manage(user_service)
        .manage(authentication_service)
        // expose swagger ui.
        // Go to http://localhost:8000/docs to view your endpoint documentation.
        .mount(
            "/",
            SwaggerUi::new("/docs/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        // Mount all your routes here.
        .mount("/users", user_routes())
        .mount("/login", authentication_routes())
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}
