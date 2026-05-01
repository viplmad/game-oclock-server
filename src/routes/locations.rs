use actix_web::{Responder, delete, get, post, put, web};

use crate::models::{
    AggregateResultDTO, AggregateSearchDTO, ErrorMessage, ItemId, ListSearchDTO, LocationAvailablePageResult, LocationDTO, LocationPageResult, LoggedUser, NewLocationDTO, QuicksearchQuery
};
use crate::services::{LocationService, MediaAvailableService};

use super::helpers::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

/// Get a location
#[utoipa::path(
    get,
    path = "/api/v1/locations/{id}",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 200, description = "Location obtained", body = LocationDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/locations/{id}")]
pub async fn get_location(
    location_service: web::Data<LocationService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = location_service.get_location(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Get all locations where a media is available
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/locations/list",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Locations obtained", body = LocationAvailablePageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/locations/list")]
pub async fn get_media_locations(
    media_available_service: web::Data<MediaAvailableService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let search_result = media_available_service
        .search_media_locations(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Aggregate all locations where a media is available
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/locations/aggregate",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = AggregateSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Locations aggregate obtained", body = AggregateResultDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/locations/aggregate")]
pub async fn aggregate_media_locations(
    media_available_service: web::Data<MediaAvailableService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<AggregateSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = media_available_service
        .aggregate_media_locations(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(get_result)
}

/// Get locations
#[utoipa::path(
    post,
    path = "/api/v1/locations/list",
    tag = "Locations",
    params(
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Locations obtained", body = LocationPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/locations/list")]
pub async fn get_locations(
    location_service: web::Data<LocationService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = location_service
        .search_locations(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Aggregate locations
#[utoipa::path(
    post,
    path = "/api/v1/locations/aggregate",
    tag = "Locations",
    params(
        QuicksearchQuery,
    ),
    request_body(content = AggregateSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Locations aggregate obtained", body = AggregateResultDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/locations/aggregate")]
pub async fn aggregate_locations(
    location_service: web::Data<LocationService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<AggregateSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let aggregate_result = location_service
        .aggregate_locations(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(aggregate_result)
}

/// Create a location
#[utoipa::path(
    post,
    path = "/api/v1/locations",
    tag = "Locations",
    request_body(content = NewLocationDTO, description = "Location to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Location created", body = LocationDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/locations")]
pub async fn create_location(
    location_service: web::Data<LocationService>,
    body: web::Json<NewLocationDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = location_service
        .create_location(&logged_user.id, body.0)
        .await;
    handle_create_result(create_result)
}

/// Update a location
#[utoipa::path(
    put,
    path = "/api/v1/locations/{id}",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    request_body(content = NewLocationDTO, description = "Location to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Location updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/locations/{id}")]
pub async fn update_location(
    location_service: web::Data<LocationService>,
    path: web::Path<ItemId>,
    body: web::Json<NewLocationDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = location_service
        .update_location(&logged_user.id, &id, body.0)
        .await;
    handle_update_result(update_result)
}

/// Delete a location
#[utoipa::path(
    delete,
    path = "/api/v1/locations/{id}",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 204, description = "Location deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/locations/{id}")]
pub async fn delete_location(
    location_service: web::Data<LocationService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = location_service.delete_location(&logged_user.id, &id).await;
    handle_delete_result(delete_result)
}
