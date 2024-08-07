use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{
    ItemId, LoggedUser, NewUserDTO, PasswordChangeDTO, PasswordQuery, QuicksearchQuery, SearchDTO,
};
use crate::services::users_service;

use super::base::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_update_result, require_admin,
};

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = String, Path, description = "User id"),
    ),
    responses(
        (status = 200, description = "User obtained", body = UserDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/users/{id}")]
pub async fn get_user(pool: web::Data<PgPool>, path: web::Path<ItemId>) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = users_service::get_user(&pool, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/myself",
    tag = "Users",
    responses(
        (status = 200, description = "User obtained", body = UserDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/myself")]
pub async fn get_current_user(pool: web::Data<PgPool>, logged_user: LoggedUser) -> impl Responder {
    let get_result = users_service::get_user(&pool, &logged_user.id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/users/list",
    tag = "Users",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Users obtained", body = UserPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/users/list")]
pub async fn get_users(
    pool: web::Data<PgPool>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
) -> impl Responder {
    let search_result = users_service::search_users(&pool, body.0, query.0.q).await;
    handle_get_result(search_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "Users",
    params(
        PasswordQuery,
    ),
    request_body(content = NewUserDTO, description = "User to be created", content_type = "application/json"),
    responses(
        (status = 201, description = "User created", body = UserDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/users")]
pub async fn post_user(
    pool: web::Data<PgPool>,
    query: web::Query<PasswordQuery>,
    body: web::Json<NewUserDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&pool, &logged_user.id).await {
        return error;
    }

    let create_result = users_service::create_user(&pool, body.0, &query.0.password).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = String, Path, description = "User id"),
    ),
    request_body(content = NewUserDTO, description = "User to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "User updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/users/{id}")]
pub async fn put_user(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewUserDTO>,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = users_service::update_user(&pool, &id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/myself/change-password",
    tag = "Users",
    request_body(content = PasswordChangeDTO, description = "Password change request", content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 204, description = "Password changed"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/myself/change-password")]
pub async fn change_password(
    pool: web::Data<PgPool>,
    form: web::Form<PasswordChangeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let change_password_result =
        users_service::change_user_password(&pool, &logged_user.id, form.0).await;
    handle_action_result(change_password_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}/promote",
    tag = "Users",
    params(
        ("id" = String, Path, description = "User id"),
    ),
    responses(
        (status = 204, description = "User updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/users/{id}/promote")]
pub async fn promote_user(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&pool, &logged_user.id).await {
        return error;
    }

    let ItemId(id) = path.into_inner();
    let update_result = users_service::promote_user(&pool, &id).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}/demote",
    tag = "Users",
    params(
        ("id" = String, Path, description = "User id"),
    ),
    responses(
        (status = 204, description = "User updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/users/{id}/demote")]
pub async fn demote_user(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&pool, &logged_user.id).await {
        return error;
    }

    let ItemId(id) = path.into_inner();
    let update_result = users_service::demote_user(&pool, &id).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "Users",
    params(
        ("id" = String, Path, description = "User id"),
    ),
    responses(
        (status = 204, description = "User deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "User not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/users/{id}")]
pub async fn delete_user(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&pool, &logged_user.id).await {
        return error;
    }

    let ItemId(id) = path.into_inner();
    let delete_result = users_service::delete_user(&pool, &id).await;
    handle_delete_result(delete_result)
}
