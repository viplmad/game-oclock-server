use actix_web::{Responder, delete, get, post, put, web};

use crate::models::{
    DateTimeDTO, ErrorMessage, ExternalQuicksearchQuery, ItemId, ItemIdAndRelatedId, ListSearchDTO,
    LoggedUser, Media2DTO, MediaAvailablePageResult, MediaDTO, MediaPageResult, MediaTagPageResult,
    NewMediaDTO, OrderDTO, QuicksearchQuery,
};
use crate::services::{
    MediaAvailableService, MediaService, MediaSessionDeviceService, MediaTagService,
};

use super::helpers::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_update_result,
};

/// Get a media
#[utoipa::path(
    get,
    path = "/api/v1/medias/{id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    responses(
        (status = 200, description = "Media obtained", body = MediaDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/medias/{id}")]
pub async fn get_media(
    media_service: web::Data<MediaService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = media_service.get_media(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Get all medias with specified tag
#[utoipa::path(
    post,
    path = "/api/v1/tags/{id}/medias/list",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Tag id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias obtained", body = MediaTagPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/tags/{id}/medias/list")]
pub async fn get_tag_medias(
    media_tag_service: web::Data<MediaTagService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let search_result = media_tag_service
        .search_tag_medias(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Count all medias with specified tag
#[utoipa::path(
    post,
    path = "/api/v1/tags/{id}/medias/count",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Tag id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias count obtained", body = u64, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/tags/{id}/medias/count")]
pub async fn count_tag_medias(
    media_tag_service: web::Data<MediaTagService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let count_result = media_tag_service
        .count_tag_medias(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(count_result)
}

/// Get all medias avaiable in a location
#[utoipa::path(
    post,
    path = "/api/v1/locations/{id}/medias/list",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Location id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias obtained", body = MediaAvailablePageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/locations/{id}/medias/list")]
pub async fn get_location_medias(
    media_available_service: web::Data<MediaAvailableService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let search_result = media_available_service
        .search_location_medias(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Count all medias avaiable in a location
#[utoipa::path(
    post,
    path = "/api/v1/locations/{id}/medias/count",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Location id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias count obtained", body = u64, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/locations/{id}/medias/count")]
pub async fn count_location_medias(
    media_available_service: web::Data<MediaAvailableService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let count_result = media_available_service
        .search_location_medias(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(count_result)
}

/// Get medias where a session has been on a specified device
#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/medias/list",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Device id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias obtained", body = MediaPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Device not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/devices/{id}/medias/list")]
pub async fn get_device_medias(
    media_session_device_service: web::Data<MediaSessionDeviceService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let search_result = media_session_device_service
        .search_device_session_medias(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Count medias where a session has been on a specified device
#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/medias/count",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Device id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias count obtained", body = u64, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Device not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/devices/{id}/medias/count")]
pub async fn count_device_medias(
    media_session_device_service: web::Data<MediaSessionDeviceService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let count_result = media_session_device_service
        .count_device_session_medias(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(count_result)
}

/// Search medias
#[utoipa::path(
    post,
    path = "/api/v1/medias/list",
    tag = "Medias",
    params(
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias obtained", body = MediaPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/list")]
pub async fn get_medias(
    media_service: web::Data<MediaService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = media_service
        .search_medias(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Count medias
#[utoipa::path(
    post,
    path = "/api/v1/medias/count",
    tag = "Medias",
    params(
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias count obtained", body = u64, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/count")]
pub async fn count_medias(
    media_service: web::Data<MediaService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let count_result = media_service
        .search_medias(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(count_result)
}

/// Create a media
#[utoipa::path(
    post,
    path = "/api/v1/medias",
    tag = "Medias",
    request_body(content = NewMediaDTO, description = "Media to be created", content_type = "application/json"),
    responses(
        (status = 201, description = "Media created", body = MediaDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias")]
pub async fn create_media(
    media_service: web::Data<MediaService>,
    body: web::Json<NewMediaDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = media_service.create_media(&logged_user.id, body.0).await;
    handle_create_result(create_result)
}

/// Update a media
#[utoipa::path(
    put,
    path = "/api/v1/medias/{id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    request_body(content = NewMediaDTO, description = "Media to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Media updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/medias/{id}")]
pub async fn update_media(
    media_service: web::Data<MediaService>,
    path: web::Path<ItemId>,
    body: web::Json<NewMediaDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = media_service
        .update_media(&logged_user.id, &id, body.0)
        .await;
    handle_update_result(update_result)
}

/// Add a tag to a media
#[utoipa::path(
    put,
    path = "/api/v1/medias/{id}/tags/{other_id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
        ("other_id" = String, Path, description = "Tag id"),
    ),
    request_body(content = OrderDTO, description = "Order", content_type = "application/json"),
    responses(
        (status = 204, description = "Media and Tag linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media or Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/medias/{id}/tags/{other_id}")]
pub async fn link_media_tag(
    media_tag_service: web::Data<MediaTagService>,
    path: web::Path<ItemIdAndRelatedId>,
    body: web::Json<OrderDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let create_result = media_tag_service
        .create_media_tag(&logged_user.id, &id, &tag_id, body.order)
        .await;
    handle_action_result(create_result)
}

/// Add a location as available to a media
#[utoipa::path(
    put,
    path = "/api/v1/medias/{id}/locations/{other_id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
        ("other_id" = String, Path, description = "Location id"),
    ),
    request_body(content = DateTimeDTO, description = "Available date", content_type = "application/json"),
    responses(
        (status = 204, description = "Media and Location linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media or Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/medias/{id}/locations/{other_id}")]
pub async fn link_media_location(
    media_available_service: web::Data<MediaAvailableService>,
    path: web::Path<ItemIdAndRelatedId>,
    body: web::Json<DateTimeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, location_id) = path.into_inner();
    let create_result = media_available_service
        .create_media_available(&logged_user.id, &id, &location_id, body.datetime)
        .await;
    handle_action_result(create_result)
}

/// Delete a media
#[utoipa::path(
    delete,
    path = "/api/v1/medias/{id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    responses(
        (status = 204, description = "Media deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/medias/{id}")]
pub async fn delete_media(
    media_service: web::Data<MediaService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = media_service.delete_media(&logged_user.id, &id).await;
    handle_delete_result(delete_result)
}

/// Remove tag from a media
#[utoipa::path(
    delete,
    path = "/api/v1/medias/{id}/tags/{other_id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
        ("other_id" = String, Path, description = "Tag id"),
    ),
    responses(
        (status = 204, description = "Media and Tag unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media or Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/medias/{id}/tags/{other_id}")]
pub async fn unlink_media_tag(
    media_tag_service: web::Data<MediaTagService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let delete_result = media_tag_service
        .delete_media_tag(&logged_user.id, &id, &tag_id)
        .await;
    handle_action_result(delete_result)
}

/// Remove a location as available from a media
#[utoipa::path(
    delete,
    path = "/api/v1/medias/{id}/locations/{other_id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
        ("other_id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 204, description = "Media and Location unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media and Location relation not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/medias/{id}/locations/{other_id}")]
pub async fn unlink_media_location(
    media_available_service: web::Data<MediaAvailableService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, location_id) = path.into_inner();
    let delete_result = media_available_service
        .delete_media_available(&logged_user.id, &id, &location_id)
        .await;
    handle_action_result(delete_result)
}

/// Add a media as parent of another
#[utoipa::path(
    put,
    path = "/api/v1/medias/{id}/parent/{other_id}",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
        ("other_id" = String, Path, description = "Parent Media id"),
    ),
    responses(
        (status = 204, description = "Medias linked"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/medias/{id}/parent/{other_id}")]
pub async fn link_parent_media(
    media_service: web::Data<MediaService>,
    path: web::Path<ItemIdAndRelatedId>,
) -> impl Responder {
    let ItemIdAndRelatedId(id, media_id) = path.into_inner();
    let update_result = media_service.set_media_parent(&id, Some(media_id)).await;
    handle_action_result(update_result)
}

/// Remove a media parent
#[utoipa::path(
    delete,
    path = "/api/v1/medias/{id}/parent",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    responses(
        (status = 204, description = "Medias unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/medias/{id}/parent")]
pub async fn unlink_parent_media(
    media_service: web::Data<MediaService>,
    path: web::Path<ItemId>,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = media_service.set_media_parent(&id, None).await;
    handle_action_result(update_result)
}

/// Sync a media
#[utoipa::path(
    put,
    path = "/api/v1/medias/{id}/sync",
    tag = "Medias",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    responses(
        (status = 204, description = "Media synced"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/medias/{id}/sync")]
pub async fn sync_media(
    media_service: web::Data<MediaService>,
    path: web::Path<ItemId>,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = media_service.sync_media(&id).await;
    handle_update_result(update_result)
}

// Search external medias
#[utoipa::path(
    post,
    path = "/api/v1/medias/search",
    tag = "Medias",
    params(
        ExternalQuicksearchQuery,
    ),
    responses(
        (status = 200, description = "Medias count obtained", body = [Media2DTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/search")]
pub async fn search_external_medias(
    media_service: web::Data<MediaService>,
    query: web::Query<ExternalQuicksearchQuery>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = media_service
        .search_external_medias(&logged_user.id, &query.0.source, &query.0.q)
        .await;
    handle_get_result(search_result)
}
