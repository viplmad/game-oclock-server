use actix_web::{Responder, delete, get, post, put, web};

use crate::models::{
    ErrorMessage, ItemId, LoggedUser, NewTagDTO, QuicksearchQuery, SearchDTO, TagDTO, TagPageResult,
};
use crate::services::{GameTagService, TagService};

use super::helpers::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

/// Get a tag
#[utoipa::path(
    get,
    path = "/api/v1/tags/{id}",
    tag = "Tags",
    params(
        ("id" = String, Path, description = "Tag id"),
    ),
    responses(
        (status = 200, description = "Tag obtained", body = TagDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/tags/{id}")]
pub async fn get_tag(
    tag_service: web::Data<TagService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = tag_service.get_tag(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Get all tags from a game
#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/tags",
    tag = "Tags",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Tags obtained", body = [TagDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/tags")]
pub async fn get_game_tags(
    game_tag_service: web::Data<GameTagService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_tag_service.get_game_tags(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Search tags
#[utoipa::path(
    post,
    path = "/api/v1/tags/list",
    tag = "Tags",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Tags obtained", body = TagPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/tags/list")]
pub async fn get_tags(
    tag_service: web::Data<TagService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = tag_service
        .search_tags(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Create a tag
#[utoipa::path(
    post,
    path = "/api/v1/tags",
    tag = "Tags",
    request_body(content = NewTagDTO, description = "Tag to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Tag created", body = TagDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/tags")]
pub async fn post_tag(
    tag_service: web::Data<TagService>,
    body: web::Json<NewTagDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = tag_service.create_tag(&logged_user.id, body.0).await;
    handle_create_result(create_result)
}

/// Update a tag
#[utoipa::path(
    put,
    path = "/api/v1/tags/{id}",
    tag = "Tags",
    params(
        ("id" = String, Path, description = "Tag id"),
    ),
    request_body(content = NewTagDTO, description = "Tag to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Tag updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/tags/{id}")]
pub async fn put_tag(
    tag_service: web::Data<TagService>,
    path: web::Path<ItemId>,
    body: web::Json<NewTagDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = tag_service.update_tag(&logged_user.id, &id, body.0).await;
    handle_update_result(update_result)
}

/// Delete a tag
#[utoipa::path(
    delete,
    path = "/api/v1/tags/{id}",
    tag = "Tags",
    params(
        ("id" = String, Path, description = "Tag id"),
    ),
    responses(
        (status = 204, description = "Tag deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/tags/{id}")]
pub async fn delete_tag(
    tag_service: web::Data<TagService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = tag_service.delete_tag(&logged_user.id, &id).await;
    handle_delete_result(delete_result)
}
