use crate::domain::User;
use crate::service::user::UserService;
use crate::Status;
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct CreateUserRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

// Return type should later be CreateUserRepsonse
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
        date_of_birth: iso8601_str_to_date(&payload.date_of_birth).unwrap(), // Ensure this field has the correct type
        email: payload.email.clone(),
        password: payload.password.clone(),
    };

    // Call the `create` method and await its result
    match user_service.create(user).await {
        Ok(()) => Json(true),
        Err(_) => Json(false),
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetUserResponse {
    name: String,
    email: String,
}

// Return type should later be CreateUserRepsonse
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
    // user_service: &State<Arc<dyn UserService>>,
    user: User,
) -> Result<Json<GetUserResponse>, status::Custom<String>> {
    dbg!(&user);

    Ok(Json(GetUserResponse {
        email: user.email,
        name: user.name,
    }))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetUsersResponse {
    users: Vec<GetUserResponse>,
}

#[utoipa::path(
    get,
    path = "/users/all",
    responses(
        (status = 201, description = "Users recieved successfully", body = GetUsersResponse),
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
#[get("/all")]
async fn get_users(
    user_service: &State<Arc<dyn UserService>>,
    _user: User,
) -> Result<Json<GetUsersResponse>, status::Custom<String>> {
    match user_service.get_all().await {
        Ok(users) => Ok(Json(GetUsersResponse {
            users: users
                .into_iter()
                .map(|user| GetUserResponse {
                    email: user.email,
                    name: user.name,
                })
                .collect(),
        })),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Internal server error".to_string(),
        )),
    }
}

pub fn user_routes() -> Vec<rocket::Route> {
    routes![create_user, get_user, get_users]
}
