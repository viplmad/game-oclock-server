use actix_web::{Responder, delete, get, post, put, web};

use crate::models::{
    ErrorMessage, ItemId, LoggedUser, NewTagDTO, QuicksearchQuery, SearchDTO, TagDTO,
    TagMediaPageResult, TagPageResult,
};
use crate::services::{MediaTagService, TagService};

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

/// Get all tags from a media
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/tags/list",
    tag = "Tags",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Tags obtained", body = TagMediaPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/tags/list")]
pub async fn get_media_tags(
    media_tag_service: web::Data<MediaTagService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let search_result = media_tag_service
        .search_media_tags(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Count all tags from a media
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/tags/count",
    tag = "Tags",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Tags count obtained", body = u64, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/tags/count")]
pub async fn count_media_tags(
    media_tag_service: web::Data<MediaTagService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let count_result = media_tag_service
        .count_media_tags(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(count_result)
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

/// Count tags
#[utoipa::path(
    post,
    path = "/api/v1/tags/count",
    tag = "Tags",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Tags count obtained", body = u64, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/tags/count")]
pub async fn count_tags(
    tag_service: web::Data<TagService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let count_result = tag_service
        .count_tags(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(count_result)
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
pub async fn create_tag(
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
pub async fn update_tag(
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
