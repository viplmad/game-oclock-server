use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{ItemId, LoggedUser, NewTagDTO, SearchDTO};
use crate::services::{game_tags_service, tags_service};

use super::base::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/tags/{id}",
    tag = "Tags",
    params(
        ("id" = i32, Path, description = "Tag id"),
    ),
    responses(
        (status = 200, description = "Tag obtained", body = TagDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/tags/{id}")]
async fn get_tag(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = tags_service::get_tag(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/tags/{id}/games",
    tag = "Tags",
    params(
        ("id" = i32, Path, description = "Tag id"),
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/tags/{id}/games")]
async fn get_tag_games(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_tags_service::get_tag_games(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/tags",
    tag = "Tags",
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Tags obtained", body = [TagDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/tags")]
async fn get_tags(
    pool: web::Data<PgPool>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = tags_service::get_tags(&pool, logged_user.id, body.0).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/tags",
    tag = "Tags",
    request_body(content = NewTagDTO, description = "Tag to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Tag created", body = TagDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/tags")]
async fn post_tag(
    pool: web::Data<PgPool>,
    body: web::Json<NewTagDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = tags_service::create_tag(&pool, logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/tags/{id}",
    tag = "Tags",
    params(
        ("id" = i32, Path, description = "Tag id"),
    ),
    request_body(content = NewTagDTO, description = "Tag to be updated", content_type = "application/json"),
    responses(
        (status = 200, description = "Tag updated", body = TagDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/tags/{id}")]
async fn put_tag(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewTagDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = tags_service::update_tag(&pool, logged_user.id, id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/tags/{id}",
    tag = "Tags",
    params(
        ("id" = i32, Path, description = "Tag id"),
    ),
    responses(
        (status = 204, description = "Tag deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/tags/{id}")]
async fn delete_tag(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = tags_service::delete_tag(&pool, logged_user.id, id).await;
    handle_delete_result(delete_result)
}
