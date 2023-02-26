use actix_web::{delete, get, post, web, Responder};
use sqlx::PgPool;

use crate::models::{
    DateDTO, ItemId, LoggedUser, OptionalStartEndDateQuery, QuicksearchQuery, SearchDTO,
};
use crate::providers::ImageClientProvider;
use crate::services::{game_finishes_service, game_image_service, game_with_finish_service};

use super::base::{
    handle_action_result, handle_delete_result, handle_get_result, populate_get_page_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/finishes",
    tag = "GameFinish",
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
    path = "/api/v1/games/{id}/finishes/first",
    tag = "GameFinish",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "First finish obtained", body = String, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or finish not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}/finishes/first")]
async fn get_first_game_finish(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_finishes_service::get_first_game_finish(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/finished/first",
    tag = "GameFinish",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Games obtained", body = GameWithFinishPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/games/finished/first")]
async fn get_first_finished_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = game_with_finish_service::search_first_finished_games(
        &pool,
        logged_user.id,
        query.start_date,
        query.end_date,
        body.0,
        quick_query.0.q,
    )
    .await;
    populate_get_page_result(&mut get_result, |game| {
        game_image_service::populate_games_with_finish_cover(&image_client_provider, game)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/finished/last",
    tag = "GameFinish",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Games obtained", body = GameWithFinishPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/games/finished/last")]
async fn get_last_finished_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = game_with_finish_service::search_last_finished_games(
        &pool,
        logged_user.id,
        query.start_date,
        query.end_date,
        body.0,
        quick_query.0.q,
    )
    .await;
    populate_get_page_result(&mut get_result, |game| {
        game_image_service::populate_games_with_finish_cover(&image_client_provider, game)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/finishes",
    tag = "GameFinish",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    request_body(content = DateDTO, description = "Game finish date to be added", content_type = "application/json"),
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
    body: web::Json<DateDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result =
        game_finishes_service::create_game_finish(&pool, logged_user.id, id, body.date).await;
    handle_action_result(create_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/finishes",
    tag = "GameFinish",
    params(
        ("id" = i32, Path, description = "Game id"),
    ),
    request_body(content = DateDTO, description = "Game finish date to be deleted", content_type = "application/json"),
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
    body: web::Json<DateDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        game_finishes_service::delete_game_finish(&pool, logged_user.id, id, body.date).await;
    handle_delete_result(delete_result)
}
