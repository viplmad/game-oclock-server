use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{FileTempPath, ItemId, LoggedUser, NewDeviceDTO, QuicksearchQuery, SearchDTO};
use crate::providers::ImageClientProvider;
use crate::services::{device_image_service, devices_service, game_available_service};

use super::base::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_multipart_result, handle_update_result, populate_get_page_result, populate_get_result,
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
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = devices_service::get_device(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |device| {
        device_image_service::populate_device_icon(&image_client_provider, device)
    });
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
        (status = 200, description = "Devices obtained", body = [DeviceAvailableDTO], content_type = "application/json"),
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
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result =
        game_available_service::get_game_devices(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |device| {
        device_image_service::populate_devices_available_icon(&image_client_provider, device)
    });
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
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut search_result =
        devices_service::search_devices(&pool, &logged_user.id, body.0, query.0.q).await;
    populate_get_page_result(&mut search_result, |device| {
        device_image_service::populate_devices_icon(&image_client_provider, device)
    });
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
    pool: web::Data<PgPool>,
    body: web::Json<NewDeviceDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = devices_service::create_device(&pool, &logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/devices/{id}/icon",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Device id"),
    ),
    request_body(content = Image, description = "Device icon to be uploaded", content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Device icon uploaded"),
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
#[post("/devices/{id}/icon")]
pub async fn post_device_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: actix_multipart::Multipart,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();

    let file_path_result = crate::multipart_utils::get_multipart_file_path(body).await;
    let FileTempPath {
        directory_path,
        file_path,
    } = match handle_multipart_result(file_path_result) {
        Ok(res) => res,
        Err(err) => return err,
    };

    let upload_result = devices_service::set_device_icon(
        &pool,
        &image_client_provider,
        &logged_user.id,
        &id,
        &file_path,
    )
    .await;

    crate::temp_file_utils::delete_temp_dir(&directory_path).await;

    handle_action_result(upload_result)
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
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewDeviceDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = devices_service::update_device(&pool, &logged_user.id, &id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/devices/{id}/icon",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Device id"),
    ),
    request_body(content = String, description = "New device icon name", content_type = "application/json"),
    responses(
        (status = 204, description = "Device icon renamed"),
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
#[put("/devices/{id}/icon")]
pub async fn put_device_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: web::Json<String>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = devices_service::rename_device_icon(
        &pool,
        &image_client_provider,
        &logged_user.id,
        &id,
        &body.0,
    )
    .await;
    handle_action_result(update_result)
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
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        devices_service::delete_device(&pool, &image_client_provider, &logged_user.id, &id).await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/devices/{id}/icon",
    tag = "Devices",
    params(
        ("id" = String, Path, description = "Device id"),
    ),
    responses(
        (status = 204, description = "Device icon deleted"),
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
#[delete("/devices/{id}/icon")]
pub async fn delete_device_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        devices_service::delete_device_icon(&pool, &image_client_provider, &logged_user.id, &id)
            .await;
    handle_action_result(delete_result)
}
