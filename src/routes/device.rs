use actix_web::{delete, get, post, put, web, Responder};

use crate::models::{
    DeviceDTO, DevicePageResult, ErrorMessage, ItemId, LoggedUser, NewDeviceDTO, QuicksearchQuery,
    SearchDTO,
};
use crate::services::{DeviceService, GamePlayedDeviceService};

use super::helpers::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

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

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/devices",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Devices obtained", body = [DeviceDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/devices")]
pub async fn get_game_devices(
    game_played_device_service: web::Data<GamePlayedDeviceService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_played_device_service
        .get_game_played_devices(&logged_user.id, &id)
        .await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/list",
    tag = "Devices",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Devices obtained", body = DevicePageResult, content_type = "application/json"),
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
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = device_service
        .search_devices(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/devices",
    tag = "Devices",
    request_body(content = NewDeviceDTO, description = "Device to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Device created", body = DeviceDTO, content_type = "application/json"),
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
pub async fn post_device(
    device_service: web::Data<DeviceService>,
    body: web::Json<NewDeviceDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = device_service.create_device(&logged_user.id, body.0).await;
    handle_create_result(create_result)
}

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
pub async fn put_device(
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
