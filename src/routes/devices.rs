use actix_web::{Responder, delete, get, post, put, web};

use crate::models::{
    AggregateResultDTO, AggregateSearchDTO, DeviceDTO, ErrorMessage, ItemId, ListSearchDTO,
    LoggedUser, NewDeviceDTO, PageResultDTO, QuicksearchQuery,
};
use crate::services::{DeviceService, MediaSessionDeviceService};

use super::helpers::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

/// Get a device
#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Device id"),
    ),
    responses(
        (status = 200, description = "Device obtained", body = DeviceDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Device not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/devices/{id}")]
pub async fn get_device(
    device_service: web::Data<DeviceService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = device_service.get_device(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Get all devices where a media has been in a session
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/devices/list",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Devices obtained", body = PageResultDTO<DeviceDTO>, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/devices/list")]
pub async fn get_media_devices(
    media_session_device_service: web::Data<MediaSessionDeviceService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let search_result = media_session_device_service
        .search_media_session_devices(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Aggregate all devices where a media has been in a session
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/devices/aggregate",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = AggregateSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Devices aggregate obtained", body = AggregateResultDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/devices/aggregate")]
pub async fn aggregate_media_devices(
    media_session_device_service: web::Data<MediaSessionDeviceService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<AggregateSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let aggregate_result = media_session_device_service
        .aggregate_media_session_devices(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(aggregate_result)
}

/// Get devices
#[utoipa::path(
    post,
    path = "/api/v1/devices/list",
    tag = "Devices",
    params(
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Devices obtained", body = PageResultDTO<DeviceDTO>, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/devices/list")]
pub async fn get_devices(
    device_service: web::Data<DeviceService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = device_service
        .search_devices(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Aggregate devices
#[utoipa::path(
    post,
    path = "/api/v1/devices/aggregate",
    tag = "Devices",
    params(
        QuicksearchQuery,
    ),
    request_body(content = AggregateSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Devices aggregate obtained", body = AggregateResultDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/devices/aggregate")]
pub async fn aggregate_devices(
    device_service: web::Data<DeviceService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<AggregateSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let aggregate_result = device_service
        .aggregate_devices(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(aggregate_result)
}

/// Create a device
#[utoipa::path(
    post,
    path = "/api/v1/devices",
    tag = "Devices",
    request_body(content = NewDeviceDTO, description = "Device to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Device created", body = String, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Device not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/devices")]
pub async fn create_device(
    device_service: web::Data<DeviceService>,
    body: web::Json<NewDeviceDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = device_service.create_device(&logged_user.id, body.0).await;
    handle_create_result(create_result)
}

/// Update a device
#[utoipa::path(
    put,
    path = "/api/v1/devices/{id}",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Device id"),
    ),
    request_body(content = NewDeviceDTO, description = "Device to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Device updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Device not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/devices/{id}")]
pub async fn update_device(
    device_service: web::Data<DeviceService>,
    path: web::Path<ItemId>,
    body: web::Json<NewDeviceDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = device_service
        .update_device(&logged_user.id, &id, body.0)
        .await;
    handle_update_result(update_result)
}

/// Delete a device
#[utoipa::path(
    delete,
    path = "/api/v1/devices/{id}",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Device id"),
    ),
    responses(
        (status = 204, description = "Device deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Device not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/devices/{id}")]
pub async fn delete_device(
    device_service: web::Data<DeviceService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = device_service.delete_device(&logged_user.id, &id).await;
    handle_delete_result(delete_result)
}
