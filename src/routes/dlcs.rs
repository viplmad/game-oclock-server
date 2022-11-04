use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{ItemId, LoggedUser, NewDLCDTO, QueryRequest};
use crate::services::dlcs_service;

use super::base::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/dlcs/{id}",
    tag = "DLCs",
    params(
        ("id" = i32, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "DLC obtained", body = DLCDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
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
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = dlcs_service::get_dlc(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/dlcs/{id}/base-game",
    tag = "DLCs",
    params(
        ("id" = i32, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "Game obtained", body = GameDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
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
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = dlcs_service::get_dlc_base_game(&pool, logged_user.id, id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/dlcs",
    tag = "DLCs",
    params(
        QueryRequest,
    ),
    responses(
        (status = 200, description = "DLCs obtained", body = [DLCDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/dlcs")]
async fn get_dlcs(
    pool: web::Data<PgPool>,
    query: web::Query<QueryRequest>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = dlcs_service::get_dlcs(&pool, logged_user.id, query.0).await;
    handle_get_result(get_result)
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
    let create_result = dlcs_service::create_dlc(&pool, logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/dlcs/{id}",
    tag = "DLCs",
    params(
        ("id" = i32, Path, description = "DLC id"),
    ),
    request_body(content = NewDLCDTO, description = "DLC to be updated", content_type = "application/json"),
    responses(
        (status = 200, description = "DLC updated", body = DLCDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
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
    let update_result = dlcs_service::update_dlc(&pool, logged_user.id, id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/dlcs/{id}",
    tag = "DLCs",
    params(
        ("id" = i32, Path, description = "DLC id"),
    ),
    responses(
        (status = 204, description = "DLC deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
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
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = dlcs_service::delete_dlc(&pool, logged_user.id, id).await;
    handle_delete_result(delete_result)
}
