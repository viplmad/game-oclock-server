use actix_web::{get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{LoggedUser, NewUserDTO, PasswordChangeDTO};
use crate::services::users_service;

use super::base::{handle_action_result, handle_create_result, handle_get_result};

#[utoipa::path(
    get,
    path = "/api/v1/myself",
    tag = "Users",
    responses(
        (status = 200, description = "User obtained", body = UserDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/myself")]
pub async fn get_current_user(pool: web::Data<PgPool>, logged_user: LoggedUser) -> impl Responder {
    let get_result = users_service::get_user(&pool, logged_user.id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    request_body(content = NewUserDTO, description = "User to be created", content_type = "application/json"),
    responses(
        (status = 201, description = "User created", body = UserDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/users")]
pub async fn post_user(pool: web::Data<PgPool>, body: web::Json<NewUserDTO>) -> impl Responder {
    let create_result = users_service::create_user(&pool, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/change-password",
    tag = "Users",
    request_body(content = PasswordChangeDTO, description = "Password change request", content_type = "application/json"),
    responses(
        (status = 204, description = "Password changed"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/change-password")]
pub async fn change_password(
    pool: web::Data<PgPool>,
    body: web::Json<PasswordChangeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let change_password_result =
        users_service::change_user_password(&pool, logged_user.id, body.0).await;
    handle_action_result(change_password_result)
}
