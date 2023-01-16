use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{ItemId, LoggedUser, NewPlatformDTO, QuicksearchQuery, SearchDTO};
use crate::providers::ImageClientProvider;
use crate::routes::base::handle_action_result;
use crate::services::{
    dlc_available_service, game_available_service, platform_image_service, platforms_service,
};

use super::base::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/platforms/{id}",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    responses(
        (status = 200, description = "Platform obtained", body = PlatformDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/platforms/{id}")]
async fn get_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = platforms_service::get_platform(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/platforms/{id}/games",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/platforms/{id}/games")]
async fn get_platform_games(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_available_service::get_platform_games(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/platforms/{id}/dlcs",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    responses(
        (status = 200, description = "DLCs obtained", body = [DLCAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/platforms/{id}/dlcs")]
async fn get_platform_dlcs(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = dlc_available_service::get_platform_dlcs(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/platforms/list",
    tag = "Platforms",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Platforms obtained", body = PlatformPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/platforms/list")]
async fn get_platforms(
    pool: web::Data<PgPool>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result =
        platforms_service::search_platforms(&pool, logged_user.id, body.0, query.0.q).await;
    handle_get_result(search_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/platforms",
    tag = "Platforms",
    request_body(content = NewPlatformDTO, description = "Platform to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Platform created", body = PlatformDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/platforms")]
async fn post_platform(
    pool: web::Data<PgPool>,
    body: web::Json<NewPlatformDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = platforms_service::create_platform(&pool, logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/platforms/{id}/icon",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    request_body(content = Image, description = "Platform icon to be uploaded", content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Platform icon uploaded"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/platforms/{id}/icon")]
async fn post_platform_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: actix_multipart::Multipart,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let file_result = crate::multipart_utils::get_multipart_file(body).await;
    let upload_result = platform_image_service::set_platform_icon(
        &pool,
        &image_client_provider,
        logged_user.id,
        id,
        file_result,
    )
    .await;
    handle_action_result(upload_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/platforms/{id}",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    request_body(content = NewPlatformDTO, description = "Platform to be updated", content_type = "application/json"),
    responses(
        (status = 200, description = "Platform updated", body = PlatformDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/platforms/{id}")]
async fn put_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewPlatformDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = platforms_service::update_platform(&pool, logged_user.id, id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/platforms/{id}/icon",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    request_body(content = String, description = "New platform filename", content_type = "application/json"),
    responses(
        (status = 204, description = "Platform icon renamed"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/platforms/{id}/icon")]
async fn put_platform_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: web::Json<String>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = platform_image_service::rename_platform_icon(
        &pool,
        &image_client_provider,
        logged_user.id,
        id,
        &body.0,
    )
    .await;
    handle_action_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/platforms/{id}",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    responses(
        (status = 204, description = "Platform deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/platform/{id}")]
async fn delete_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = platforms_service::delete_platform(&pool, logged_user.id, id).await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/platforms/{id}/icon",
    tag = "Platforms",
    params(
        ("id" = i32, Path, description = "Platform id"),
    ),
    responses(
        (status = 204, description = "Platform icon deleted"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/platforms/{id}/icon")]
async fn delete_platform_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = platform_image_service::delete_platform_icon(
        &pool,
        &image_client_provider,
        logged_user.id,
        id,
    )
    .await;
    handle_action_result(delete_result)
}
