use actix_web::{Responder, delete, get, post, web};

use crate::models::{
    DateTimeDTO, ErrorMessage, FinishDTO, GameWithFinishPageResult, GamesFinishedReviewDTO, ItemId,
    LoggedUser, NewFinishDTO, OptionalStartEndDateQuery, QuicksearchQuery, SearchDTO,
    StartEndDateQuery,
};
use crate::services::{GameFinishService, GameReviewService, GameWithFinishService};

use super::helpers::{handle_action_result, handle_delete_result, handle_get_result};

/// Get all game finishes
#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/finishes",
    tag = "GameFinishes",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Finishes obtained", body = [FinishDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/finishes")]
pub async fn get_game_finishes(
    game_finish_service: web::Data<GameFinishService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_finish_service
        .get_game_finishes(&logged_user.id, &id)
        .await;
    handle_get_result(get_result)
}

/// Get date of first time a game was finished
#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/finishes/first",
    tag = "GameFinishes",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "First finish obtained", body = String, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or finish not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/finishes/first")]
pub async fn get_first_game_finish(
    game_finish_service: web::Data<GameFinishService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_finish_service
        .get_first_game_finish(&logged_user.id, &id)
        .await;
    handle_get_result(get_result)
}

/// Get a finish review in a time frame
#[utoipa::path(
    post,
    path = "/api/v1/games/finished/review",
    tag = "GameFinishes",
    params(
        StartEndDateQuery,
    ),
    responses(
        (status = 200, description = "Finished games review obtained", body = GamesFinishedReviewDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/finished/review")]
pub async fn get_finished_games_review(
    game_review_service: web::Data<GameReviewService>,
    query: web::Query<StartEndDateQuery>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = game_review_service
        .get_finished_games_review(&logged_user.id, query.start_date, query.end_date)
        .await;
    handle_get_result(get_result)
}

/// Search first finished games
#[utoipa::path(
    post,
    path = "/api/v1/games/finished/first",
    tag = "GameFinishes",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Games obtained", body = GameWithFinishPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/finished/first")]
pub async fn get_first_finished_games(
    game_with_finish_service: web::Data<GameWithFinishService>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = game_with_finish_service
        .search_first_finished_games(
            &logged_user.id,
            query.start_date,
            query.end_date,
            body.0,
            quick_query.0.q,
        )
        .await;
    handle_get_result(get_result)
}

/// Search last finished games
#[utoipa::path(
    post,
    path = "/api/v1/games/finished/last",
    tag = "GameFinishes",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Games obtained", body = GameWithFinishPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/finished/last")]
pub async fn get_last_finished_games(
    game_with_finish_service: web::Data<GameWithFinishService>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = game_with_finish_service
        .search_last_finished_games(
            &logged_user.id,
            query.start_date,
            query.end_date,
            body.0,
            quick_query.0.q,
        )
        .await;
    handle_get_result(get_result)
}

/// Create a game finish
#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/finishes",
    tag = "GameFinishes",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = NewFinishDTO, description = "Game finish to be added", content_type = "application/json"),
    responses(
        (status = 204, description = "Game finish added"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/{id}/finishes")]
pub async fn post_game_finish(
    game_finish_service: web::Data<GameFinishService>,
    path: web::Path<ItemId>,
    body: web::Json<NewFinishDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result = game_finish_service
        .create_game_finish(&logged_user.id, &id, body.0)
        .await;
    handle_action_result(create_result)
}

/// Delete a game finish
#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/finishes",
    tag = "GameFinishes",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = DateTimeDTO, description = "Game finish date to be deleted", content_type = "application/json"),
    responses(
        (status = 204, description = "Game finish deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/games/{id}/finishes")]
pub async fn delete_game_finish(
    game_finish_service: web::Data<GameFinishService>,
    path: web::Path<ItemId>,
    body: web::Json<DateTimeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = game_finish_service
        .delete_game_finish(&logged_user.id, &id, body.datetime)
        .await;
    handle_delete_result(delete_result)
}
