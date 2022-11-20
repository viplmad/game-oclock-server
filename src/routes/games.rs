use actix_web::{delete, get, post, put, web, Responder};
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

use crate::models::{GameLogDTO, ItemId, ItemIdAndRelatedId, LoggedUser, NewGameDTO, QueryRequest};
use crate::services::{
    dlcs_service, game_available_service, game_finishes_service, game_logs_service,
    game_tags_service, games_service,
};

use super::base::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_update_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Game obtained", body = GameDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}")]
async fn get_game(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = games_service::get_game(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/finishes",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Finishes obtained", body = [String], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}/finishes")]
async fn get_game_finishes(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_finishes_service::get_game_finishes(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/logs",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Logs obtained", body = [GameLogDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}/logs")]
async fn get_game_logs(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_logs_service::get_game_logs(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/dlcs",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "DLCs obtained", body = [DLCDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
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
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = dlcs_service::get_game_dlcs(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/tags",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Tags obtained", body = [TagDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}/tags")]
async fn get_game_tags(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_tags_service::get_game_tags(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/platforms",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Platforms obtained", body = [PlatformAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}/platforms")]
async fn get_game_platforms(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_available_service::get_game_platforms(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games",
    tag = "Games",
    params(
        QueryRequest,
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games")]
async fn get_games(
    pool: web::Data<PgPool>,
    query: web::Query<QueryRequest>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = games_service::get_games(&pool, logged_user.id, query.0).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games",
    tag = "Games",
    request_body(content = NewGameDTO, description = "Game to be created", content_type = "application/json"),
    responses(
        (status = 201, description = "Game created", body = GameDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/games")]
async fn post_game(
    pool: web::Data<PgPool>,
    body: web::Json<NewGameDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = games_service::create_game(&pool, logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/finishes",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    request_body(content = String, description = "Game finish date to be added", content_type = "application/json"),
    responses(
        (status = 204, description = "Game finish added"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/games/{id}/finishes")]
async fn post_game_finish(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NaiveDate>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result =
        game_finishes_service::create_game_finish(&pool, logged_user.id, id, body.0).await;
    handle_action_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/logs",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    request_body(content = GameLogDTO, description = "Game log to be added", content_type = "application/json"),
    responses(
        (status = 204, description = "Game log added"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/games/{id}/logs")]
async fn post_game_log(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<GameLogDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result = game_logs_service::create_game_log(&pool, logged_user.id, id, body.0).await;
    handle_action_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/games/{id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    request_body(content = NewGameDTO, description = "Game to be updated", content_type = "application/json"),
    responses(
        (status = 200, description = "Game updated", body = GameDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}")]
async fn put_game(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewGameDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = games_service::update_game(&pool, logged_user.id, id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/dlcs/{other_id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
        ("other_id" = i32, Path, description = "DLC id")
    ),
    responses(
        (status = 204, description = "Game and DLC linked"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}/dlcs/{other_id}")]
async fn link_game_dlc(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, dlc_id) = path.into_inner();
    let update_result = dlcs_service::update_dlc_base_game(&pool, logged_user.id, dlc_id, id).await;
    handle_action_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/tags/{other_id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
        ("other_id" = i32, Path, description = "Tag id")
    ),
    responses(
        (status = 204, description = "Game and Tag linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}/tags/{other_id}")]
async fn link_game_tag(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let create_result = game_tags_service::create_game_tag(&pool, logged_user.id, id, tag_id).await;
    handle_action_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/platforms/{other_id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
        ("other_id" = i32, Path, description = "Platform id")
    ),
    request_body(content = String, description = "Available date", content_type = "application/json"),
    responses(
        (status = 204, description = "Game and Platform linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}/platforms/{other_id}")]
async fn link_game_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    body: web::Json<NaiveDate>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, platform_id) = path.into_inner();
    let create_result = game_available_service::create_game_available(
        &pool,
        logged_user.id,
        id,
        platform_id,
        body.0,
    )
    .await;
    handle_action_result(create_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 204, description = "Game deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}")]
async fn delete_game(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = games_service::delete_game(&pool, logged_user.id, id).await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/finishes",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    request_body(content = String, description = "Game finish date to be deleted", content_type = "application/json"),
    responses(
        (status = 204, description = "Game finish date deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/finishes")]
async fn delete_game_finish(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NaiveDate>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        game_finishes_service::delete_game_finish(&pool, logged_user.id, id, body.0).await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/logs",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    request_body(content = String, description = "Game log datetime to be deleted", content_type = "application/json"),
    responses(
        (status = 204, description = "Game log deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/logs")]
async fn delete_game_log(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NaiveDateTime>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = game_logs_service::delete_game_log(&pool, logged_user.id, id, body.0).await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/dlcs/{other_id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
        ("other_id" = i32, Path, description = "DLC id")
    ),
    responses(
        (status = 204, description = "Game and DLC unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/dlcs/{other_id}")]
async fn unlink_game_dlc(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, dlc_id) = path.into_inner();
    let update_result = dlcs_service::remove_dlc_base_game(&pool, logged_user.id, dlc_id, id).await;
    handle_action_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/tags/{other_id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
        ("other_id" = i32, Path, description = "Tag id")
    ),
    responses(
        (status = 204, description = "Game and Tag unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/tags/{other_id}")]
async fn unlink_game_tag(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let delete_result = game_tags_service::delete_game_tag(&pool, logged_user.id, id, tag_id).await;
    handle_action_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/platforms/{other_id}",
    tag = "Games",
    params(
        ("id" = i32, Path, description = "Game id"),
        ("other_id" = i32, Path, description = "Platform id")
    ),
    responses(
        (status = 204, description = "Game and Platform unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game and Platform relation not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/platforms/{other_id}")]
async fn unlink_game_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, platform_id) = path.into_inner();
    let delete_result =
        game_available_service::delete_game_available(&pool, logged_user.id, id, platform_id).await;
    handle_action_result(delete_result)
}
