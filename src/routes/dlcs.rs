use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{
    DateDTO, ItemId, ItemIdAndRelatedId, LoggedUser, NewDLCDTO, QuicksearchQuery, SearchDTO,
};
use crate::providers::ImageClientProvider;
use crate::services::{dlc_available_service, dlc_image_service, dlcs_service, game_image_service};

use super::base::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_multipart_result, handle_update_result, populate_get_page_result, populate_get_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/dlcs/{id}",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "DLC obtained", body = DLCDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/dlcs/{id}")]
async fn get_dlc(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = dlcs_service::get_dlc(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |dlc| {
        dlc_image_service::populate_dlc_cover(&image_client_provider, dlc)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/dlcs/{id}/base-game",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "Game obtained", body = GameDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/dlcs/{id}/base-game")]
async fn get_dlc_base_game(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = dlcs_service::get_dlc_base_game(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |game| {
        game_image_service::populate_game_cover(&image_client_provider, game)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/dlcs",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "DLCs obtained", body = [DLCDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}/dlcs")]
async fn get_game_dlcs(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = dlcs_service::get_game_dlcs(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |dlcs| {
        dlc_image_service::populate_dlcs_cover(&image_client_provider, dlcs)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/platforms/{id}/dlcs",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    responses(
        (status = 200, description = "DLCs obtained", body = [DLCAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
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
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result =
        dlc_available_service::get_platform_dlcs(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |dlcs| {
        dlc_image_service::populate_dlcs_available_cover(&image_client_provider, dlcs)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/dlcs/list",
    tag = "DLCs",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "DLCs obtained", body = DLCPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/dlcs/list")]
async fn get_dlcs(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut search_result =
        dlcs_service::search_dlcs(&pool, &logged_user.id, body.0, query.0.q).await;
    populate_get_page_result(&mut search_result, |dlcs| {
        dlc_image_service::populate_dlcs_cover(&image_client_provider, dlcs)
    });
    handle_get_result(search_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/dlcs",
    tag = "DLCs",
    request_body(content = NewDLCDTO, description = "DLC to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "DLC created", body = DLCDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/dlcs")]
async fn post_dlc(
    pool: web::Data<PgPool>,
    body: web::Json<NewDLCDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = dlcs_service::create_dlc(&pool, &logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/dlcs/{id}/cover",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    request_body(content = Image, description = "DLC cover to be uploaded", content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "DLC cover uploaded"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/dlcs/{id}/cover")]
async fn post_dlc_cover(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: actix_multipart::Multipart,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();

    let file_path_result = crate::multipart_utils::get_multipart_file_path(body).await;
    let file_path = match handle_multipart_result(file_path_result) {
        Ok(res) => res,
        Err(err) => return err,
    };

    let upload_result = dlcs_service::set_dlc_cover(
        &pool,
        &image_client_provider,
        &logged_user.id,
        &id,
        &file_path,
    )
    .await;

    crate::multipart_utils::delete_temp_path(&file_path).await;

    handle_action_result(upload_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/dlcs/{id}",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    request_body(content = NewDLCDTO, description = "DLC to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "DLC updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/dlcs/{id}")]
async fn put_dlc(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewDLCDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = dlcs_service::update_dlc(&pool, &logged_user.id, &id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/dlcs/{id}/cover",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    request_body(content = String, description = "New dlc cover name", content_type = "application/json"),
    responses(
        (status = 204, description = "DLC cover renamed"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/dlcs/{id}/cover")]
async fn put_dlc_cover(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: web::Json<String>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = dlcs_service::rename_dlc_cover(
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
    put,
    path = "/api/v1/dlcs/{id}/base-game/{other_id}",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
        ("other_id" = String, Path, description = "Game id")
    ),
    responses(
        (status = 204, description = "DLC and Game linked"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/dlcs/{id}/base-game/{other_id}")]
async fn link_dlc_game(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, game_id) = path.into_inner();
    let update_result =
        dlcs_service::set_dlc_base_game(&pool, &logged_user.id, &id, Some(game_id)).await;
    handle_action_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/dlcs/{id}/platforms/{other_id}",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
        ("other_id" = String, Path, description = "Platform id")
    ),
    request_body(content = DateDTO, description = "Available date", content_type = "application/json"),
    responses(
        (status = 204, description = "DLC and Platform linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/dlcs/{id}/platforms/{other_id}")]
async fn link_dlc_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    body: web::Json<DateDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, platform_id) = path.into_inner();
    let create_result = dlc_available_service::create_dlc_available(
        &pool,
        &logged_user.id,
        &id,
        &platform_id,
        body.date,
    )
    .await;
    handle_action_result(create_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/dlcs/{id}",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 204, description = "DLC deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/dlcs/{id}")]
async fn delete_dlc(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        dlcs_service::delete_dlc(&pool, &image_client_provider, &logged_user.id, &id).await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/dlcs/{id}/cover",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 204, description = "DLC cover deleted"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/dlcs/{id}/cover")]
async fn delete_dlc_cover(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        dlcs_service::delete_dlc_cover(&pool, &image_client_provider, &logged_user.id, &id).await;
    handle_action_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/dlcs/{id}/base-game",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 204, description = "DLC and Game unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/dlcs/{id}/base-game")]
async fn unlink_dlc_game(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = dlcs_service::set_dlc_base_game(&pool, &logged_user.id, &id, None).await;
    handle_action_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/dlcs/{id}/platforms/{other_id}",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
        ("other_id" = String, Path, description = "Platform id")
    ),
    responses(
        (status = 204, description = "DLC and Platform unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC and Platform relation not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/dlcs/{id}/platforms/{other_id}")]
async fn unlink_dlc_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, platform_id) = path.into_inner();
    let delete_result =
        dlc_available_service::delete_dlc_available(&pool, &logged_user.id, &id, &platform_id)
            .await;
    handle_action_result(delete_result)
}
