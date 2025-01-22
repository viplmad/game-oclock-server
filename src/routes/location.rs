use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{
    FileTempPath, ItemId, LoggedUser, NewLocationDTO, QuicksearchQuery, SearchDTO,
};
use crate::providers::ImageClientProvider;
use crate::services::{game_available_service, location_image_service, locations_service};

use super::base::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_multipart_result, handle_update_result, populate_get_page_result, populate_get_result,
};

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
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = locations_service::get_location(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |location| {
        location_image_service::populate_location_icon(&image_client_provider, location)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/locations",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Locations obtained", body = [LocationAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/locations")]
pub async fn get_game_locations(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result =
        game_available_service::get_game_locations(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |location| {
        location_image_service::populate_locations_available_icon(&image_client_provider, location)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/locations/list",
    tag = "Locations",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
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
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut search_result =
        locations_service::search_locations(&pool, &logged_user.id, body.0, query.0.q).await;
    populate_get_page_result(&mut search_result, |location| {
        location_image_service::populate_locations_icon(&image_client_provider, location)
    });
    handle_get_result(search_result)
}

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
pub async fn post_location(
    pool: web::Data<PgPool>,
    body: web::Json<NewLocationDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = locations_service::create_location(&pool, &logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/locations/{id}/icon",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    request_body(content = Image, description = "Location icon to be uploaded", content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Location icon uploaded"),
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
#[post("/locations/{id}/icon")]
pub async fn post_location_icon(
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

    let upload_result = locations_service::set_location_icon(
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
pub async fn put_location(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewLocationDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result =
        locations_service::update_location(&pool, &logged_user.id, &id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/locations/{id}/icon",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    request_body(content = String, description = "New location icon name", content_type = "application/json"),
    responses(
        (status = 204, description = "Location icon renamed"),
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
#[put("/locations/{id}/icon")]
pub async fn put_location_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: web::Json<String>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = locations_service::rename_location_icon(
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
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        locations_service::delete_location(&pool, &image_client_provider, &logged_user.id, &id)
            .await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/locations/{id}/icon",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 204, description = "Location icon deleted"),
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
#[delete("/locations/{id}/icon")]
pub async fn delete_location_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = locations_service::delete_location_icon(
        &pool,
        &image_client_provider,
        &logged_user.id,
        &id,
    )
    .await;
    handle_action_result(delete_result)
}
