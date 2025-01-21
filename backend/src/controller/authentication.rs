use crate::service::authentication::AuthenticationService;
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct GetPlacesRequest;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct LoginResponse {
    jwt: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct LoginRequest {
    email: String,
    password: String,
}

// Return type should later be CreateUserRepsonse
#[utoipa::path(
    get,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 201, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Invalid input data"),
        (status = 500, description = "Internal server error")
    ),
    description = "Recieve a jwt when creditials are valid.",
    operation_id = "Login",
    tag = "Authentication"
)]
#[get("/", data = "<payload>")]
async fn login(
    payload: Json<LoginRequest>,
    authentication_service: &State<Arc<dyn AuthenticationService>>,
) -> Result<Json<LoginResponse>, status::Custom<String>> {
    match authentication_service
        .login(payload.email.clone(), payload.password.clone())
        .await
    {
        Ok(jwt) => match jwt {
            Some(jwt) => Ok(Json(LoginResponse { jwt })),
            None => Err(status::Custom(
                Status::InternalServerError,
                "Access denied".to_string(),
            )),
        },
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Internal server error".to_string(),
        )),
    }
}

pub fn authentication_routes() -> Vec<rocket::Route> {
    routes![login]
}
