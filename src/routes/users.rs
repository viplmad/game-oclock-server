use actix_web::{Responder, delete, get, post, put, web};

use crate::models::{
    ErrorMessage, ItemId, LoggedUser, NewUserDTO, PasswordChangeDTO, PasswordQuery,
    QuicksearchQuery, SearchDTO, UserDTO, UserPageResult,
};
use crate::routes::helpers::require_admin_or_current_user;
use crate::services::UserService;

use super::helpers::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_update_result, require_admin,
};

/// Get a user
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
pub async fn get_user(
    user_service: web::Data<UserService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();

    if let Err(error) = require_admin_or_current_user(&user_service, &logged_user.id, &id).await {
        return error;
    }

    let get_result = user_service.get_user(&id).await;
    handle_get_result(get_result)
}

/// Get current user
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
pub async fn get_current_user(
    user_service: web::Data<UserService>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = user_service.get_user(&logged_user.id).await;
    handle_get_result(get_result)
}

/// Search users
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
    user_service: web::Data<UserService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&user_service, &logged_user.id).await {
        return error;
    }

    let search_result = user_service.search_users(body.0, query.0.q).await;
    handle_get_result(search_result)
}

/// Create a user
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
    user_service: web::Data<UserService>,
    query: web::Query<PasswordQuery>,
    body: web::Json<NewUserDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&user_service, &logged_user.id).await {
        return error;
    }

    let create_result = user_service.create_user(body.0, &query.0.password).await;
    handle_create_result(create_result)
}

/// Update a user
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
    user_service: web::Data<UserService>,
    path: web::Path<ItemId>,
    body: web::Json<NewUserDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();

    if let Err(error) = require_admin_or_current_user(&user_service, &logged_user.id, &id).await {
        return error;
    }

    let update_result = user_service.update_user(&id, body.0).await;
    handle_update_result(update_result)
}

/// Change current user password
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
    user_service: web::Data<UserService>,
    form: web::Form<PasswordChangeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let change_password_result = user_service
        .change_user_password(&logged_user.id, form.0)
        .await;
    handle_action_result(change_password_result)
}

/// Promote a user
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
    user_service: web::Data<UserService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&user_service, &logged_user.id).await {
        return error;
    }

    let ItemId(id) = path.into_inner();
    let update_result = user_service.promote_user(&id).await;
    handle_update_result(update_result)
}

/// Demote a user
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
    user_service: web::Data<UserService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    if let Err(error) = require_admin(&user_service, &logged_user.id).await {
        return error;
    }

    let ItemId(id) = path.into_inner();
    let update_result = user_service.demote_user(&id).await;
    handle_update_result(update_result)
}

/// Delete a user
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
    user_service: web::Data<UserService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();

    if let Err(error) = require_admin_or_current_user(&user_service, &logged_user.id, &id).await {
        return error;
    }

    let delete_result = user_service.delete_user(&id).await;
    handle_delete_result(delete_result)
}
