use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{
    FileTempPath, ItemId, LoggedUser, NewPlatformDTO, QuicksearchQuery, SearchDTO,
};
use crate::providers::ImageClientProvider;
use crate::services::{
    dlc_available_service, game_available_service, platform_image_service, platforms_service,
};

use super::base::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_multipart_result, handle_update_result, populate_get_page_result, populate_get_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/platforms/{id}",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    responses(
        (status = 200, description = "Platform obtained", body = PlatformDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/platforms/{id}")]
pub async fn get_platform(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = platforms_service::get_platform(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |platform| {
        platform_image_service::populate_platform_icon(&image_client_provider, platform)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/platforms",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Platforms obtained", body = [PlatformAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/platforms")]
pub async fn get_game_platforms(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result =
        game_available_service::get_game_platforms(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |platform| {
        platform_image_service::populate_platforms_available_icon(&image_client_provider, platform)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/dlcs/{id}/platforms",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "Platforms obtained", body = [PlatformAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/dlcs/{id}/platforms")]
pub async fn get_dlc_platforms(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result =
        dlc_available_service::get_dlc_platforms(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |platform| {
        platform_image_service::populate_platforms_available_icon(&image_client_provider, platform)
    });
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
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/platforms/list")]
pub async fn get_platforms(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut search_result =
        platforms_service::search_platforms(&pool, &logged_user.id, body.0, query.0.q).await;
    populate_get_page_result(&mut search_result, |platform| {
        platform_image_service::populate_platforms_icon(&image_client_provider, platform)
    });
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
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/platforms")]
pub async fn post_platform(
    pool: web::Data<PgPool>,
    body: web::Json<NewPlatformDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = platforms_service::create_platform(&pool, &logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/platforms/{id}/icon",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    request_body(content = Image, description = "Platform icon to be uploaded", content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Platform icon uploaded"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/platforms/{id}/icon")]
pub async fn post_platform_icon(
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

    let upload_result = platforms_service::set_platform_icon(
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
    path = "/api/v1/platforms/{id}",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    request_body(content = NewPlatformDTO, description = "Platform to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Platform updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/platforms/{id}")]
pub async fn put_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewPlatformDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result =
        platforms_service::update_platform(&pool, &logged_user.id, &id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/platforms/{id}/icon",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    request_body(content = String, description = "New platform icon name", content_type = "application/json"),
    responses(
        (status = 204, description = "Platform icon renamed"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/platforms/{id}/icon")]
pub async fn put_platform_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: web::Json<String>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = platforms_service::rename_platform_icon(
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
    path = "/api/v1/platforms/{id}",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    responses(
        (status = 204, description = "Platform deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/platforms/{id}")]
pub async fn delete_platform(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        platforms_service::delete_platform(&pool, &image_client_provider, &logged_user.id, &id)
            .await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/platforms/{id}/icon",
    tag = "Platforms",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    responses(
        (status = 204, description = "Platform icon deleted"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/platforms/{id}/icon")]
pub async fn delete_platform_icon(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = platforms_service::delete_platform_icon(
        &pool,
        &image_client_provider,
        &logged_user.id,
        &id,
    )
    .await;
    handle_action_result(delete_result)
}
