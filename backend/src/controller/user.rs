use crate::domain::User;
use crate::service::user::UserService;
use rocket::get;
use rocket::post;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

/// Request body for creating a user.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

// Utoipa is the crate that generates swagger documentation for your endpoints.
// The documentation for each endpoint is combined in docs.rs
// Make sure to add your endpoint in docs.rs when you write new endpoints.
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = bool),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Creates a user. The email should be unique.",
    operation_id = "createUser",
    tag = "Users"
)]
#[post("/", data = "<payload>")]
async fn create_user(
    payload: Json<CreateUserRequest>,
    user_service: &State<Arc<dyn UserService>>,
) -> Json<bool> {
    // Convert `CreateUserRequest` to `User`
    let user = User {
        user_id: Uuid::new_v4(), // Generate a new UUID for the user
        name: payload.name.clone(),
        email: payload.email.clone(),
        password: payload.password.clone(),
    };

    // Call the `create` method and await its result
    match user_service.create(user).await {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

/// Response for recieving user information.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetUserResponse {
    name: String,
    email: String,
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 201, description = "User recieved successfully", body = GetUserResponse),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Recieve user details.",
    operation_id = "createUser",
    tag = "Users",
    security(
        ("jwt_auth" = [])
    )
)]
#[get("/")]
async fn get_user(
    // user is recieved by decoding the JWT.
    // when a User is required as argument for an endpoiint, is automatically protected with JWT.
    user: User,
) -> Result<Json<GetUserResponse>, status::Custom<String>> {
    Ok(Json(GetUserResponse {
        email: user.email,
        name: user.name,
    }))
}

// Combine all the user routes.
pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user, get_user]
}
