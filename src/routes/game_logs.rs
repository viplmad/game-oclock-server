use actix_web::{delete, get, post, web, Responder};
use sqlx::PgPool;

use crate::models::{
    DateTimeDTO, ItemId, LoggedUser, NewGameLogDTO, OptionalStartEndDateQuery, QuicksearchQuery,
    SearchDTO, StartEndDateQuery,
};
use crate::providers::ImageClientProvider;
use crate::services::{
    game_image_service, game_logs_service, game_review_service, game_with_logs_service,
};

use super::base::{
    handle_action_result, handle_delete_result, handle_get_result, populate_get_page_result,
    populate_get_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/logs",
    tag = "GameLogs",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Logs obtained", body = [GameLogDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/logs")]
pub async fn get_game_logs(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_logs_service::get_game_logs(&pool, &logged_user.id, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/logs/total",
    tag = "GameLogs",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Total logs time obtained", body = String, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/logs/total")]
pub async fn get_total_game_logs(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_logs_service::get_sum_game_logs(&pool, &logged_user.id, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/played/review",
    tag = "GameLogs",
    params(
        StartEndDateQuery,
    ),
    responses(
        (status = 200, description = "Played games review obtained", body = GamesPlayedReviewDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/played/review")]
pub async fn get_played_games_review(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<StartEndDateQuery>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = game_review_service::get_played_games_review(
        &pool,
        &logged_user.id,
        query.start_date,
        query.end_date,
    )
    .await;
    populate_get_result(&mut get_result, |review| {
        game_image_service::populate_games_played_review_cover(
            &image_client_provider,
            &mut review.games,
        )
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/played",
    tag = "GameLogs",
    params(
        StartEndDateQuery,
    ),
    responses(
        (status = 200, description = "Game with logs obtained", body = [GameWithLogsDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/played")]
pub async fn get_played_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<StartEndDateQuery>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = game_with_logs_service::get_game_with_logs(
        &pool,
        &logged_user.id,
        query.start_date,
        query.end_date,
    )
    .await;
    populate_get_result(&mut get_result, |game| {
        game_image_service::populate_games_with_logs_cover(&image_client_provider, game)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/played/first",
    tag = "GameLogs",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Games obtained", body = GameWithLogPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/played/first")]
pub async fn get_first_played_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = game_with_logs_service::search_first_played_games(
        &pool,
        &logged_user.id,
        query.start_date,
        query.end_date,
        body.0,
        quick_query.0.q,
    )
    .await;
    populate_get_page_result(&mut get_result, |game| {
        game_image_service::populate_games_with_log_cover(&image_client_provider, game)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/played/last",
    tag = "GameLogs",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Games obtained", body = GameWithLogPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/played/last")]
pub async fn get_last_played_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = game_with_logs_service::search_last_played_games(
        &pool,
        &logged_user.id,
        query.start_date,
        query.end_date,
        body.0,
        quick_query.0.q,
    )
    .await;
    populate_get_page_result(&mut get_result, |game| {
        game_image_service::populate_games_with_log_cover(&image_client_provider, game)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/logs",
    tag = "GameLogs",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = NewGameLogDTO, description = "Game log to be added", content_type = "application/json"),
    responses(
        (status = 204, description = "Game log added"),
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
#[post("/games/{id}/logs")]
pub async fn post_game_log(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<NewGameLogDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result =
        game_logs_service::create_game_log(&pool, &logged_user.id, &id, body.0).await;
    handle_action_result(create_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/logs",
    tag = "GameLogs",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = DateTimeDTO, description = "Game log datetime to be deleted", content_type = "application/json"),
    responses(
        (status = 204, description = "Game log deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/games/{id}/logs")]
pub async fn delete_game_log(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<DateTimeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        game_logs_service::delete_game_log(&pool, &logged_user.id, &id, body.datetime).await;
    handle_delete_result(delete_result)
}
