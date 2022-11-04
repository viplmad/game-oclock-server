use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{ItemId, ItemIdAndRelatedId, LoggedUser, NewGameDTO, QueryRequest};
use crate::services::{dlcs_service, games_service};

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
        (status = 204, description = "DLC linked"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}/dlcs/{other_id}")]
async fn put_game_dlc(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, dlc_id) = path.into_inner();
    let update_result = dlcs_service::update_dlc_base_game(&pool, logged_user.id, dlc_id, id).await;
    handle_action_result(update_result)
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
