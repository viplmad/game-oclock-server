use actix_web::{get, post, web, Responder};
use sqlx::PgPool;

use crate::models::{ItemId, LoggedUser, StartEndDateQuery};
use crate::services::game_streaks_service;

use super::base::handle_get_result;

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/streaks",
    tag = "GameStreaks",
    params(
        StartEndDateQuery,
    ),
    responses(
        (status = 200, description = "Game streaks obtained", body = [GameStreakDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}/streaks")]
async fn get_game_streaks(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    query: web::Query<StartEndDateQuery>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_streaks_service::get_game_streaks(
        &pool,
        &logged_user.id,
        &id,
        query.start_date,
        query.end_date,
    )
    .await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/streaks",
    tag = "GameStreaks",
    params(
        StartEndDateQuery,
    ),
    responses(
        (status = 200, description = "Streaks obtained", body = [GameStreakDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/games/streaks")]
async fn get_streaks(
    pool: web::Data<PgPool>,
    query: web::Query<StartEndDateQuery>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result =
        game_streaks_service::get_streaks(&pool, &logged_user.id, query.start_date, query.end_date)
            .await;
    handle_get_result(get_result)
}
