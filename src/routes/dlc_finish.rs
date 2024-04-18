use actix_web::{delete, get, post, web, Responder};
use sqlx::PgPool;

use crate::models::{
    DateDTO, ItemId, LoggedUser, OptionalStartEndDateQuery, QuicksearchQuery, SearchDTO,
};
use crate::providers::ImageClientProvider;
use crate::services::{dlc_finishes_service, dlc_image_service, dlc_with_finish_service};

use super::base::{
    handle_action_result, handle_delete_result, handle_get_result, populate_get_page_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/dlcs/{id}/finishes",
    tag = "DLCFinish",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "Finishes obtained", body = [String], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/dlcs/{id}/finishes")]
async fn get_dlc_finishes(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = dlc_finishes_service::get_dlc_finishes(&pool, &logged_user.id, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/dlcs/{id}/finishes/first",
    tag = "DLCFinish",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "First finish obtained", body = String, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or finish not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/dlcs/{id}/finishes/first")]
async fn get_first_dlc_finish(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = dlc_finishes_service::get_first_dlc_finish(&pool, &logged_user.id, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/dlcs/finished/first",
    tag = "DLCFinish",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "DLCs obtained", body = DLCWithFinishPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/dlcs/finished/first")]
async fn get_first_finished_dlcs(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = dlc_with_finish_service::search_first_finished_dlcs(
        &pool,
        &logged_user.id,
        query.start_date,
        query.end_date,
        body.0,
        quick_query.0.q,
    )
    .await;
    populate_get_page_result(&mut get_result, |dlc| {
        dlc_image_service::populate_dlcs_with_finish_cover(&image_client_provider, dlc)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/dlcs/finished/last",
    tag = "DLCFinish",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "DLCs obtained", body = DLCWithFinishPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/dlcs/finished/last")]
async fn get_last_finished_dlcs(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut get_result = dlc_with_finish_service::search_last_finished_dlcs(
        &pool,
        &logged_user.id,
        query.start_date,
        query.end_date,
        body.0,
        quick_query.0.q,
    )
    .await;
    populate_get_page_result(&mut get_result, |dlc| {
        dlc_image_service::populate_dlcs_with_finish_cover(&image_client_provider, dlc)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/dlcs/{id}/finishes",
    tag = "DLCFinish",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    request_body(content = DateDTO, description = "DLC finish date to be added", content_type = "application/json"),
    responses(
        (status = 204, description = "DLC finish added"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/dlcs/{id}/finishes")]
async fn post_dlc_finish(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<DateDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result =
        dlc_finishes_service::create_dlc_finish(&pool, &logged_user.id, &id, body.date).await;
    handle_action_result(create_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/dlcs/{id}/finishes",
    tag = "DLCFinish",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    request_body(content = DateDTO, description = "DLC finish date to be deleted", content_type = "application/json"),
    responses(
        (status = 204, description = "DLC finish date deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/dlcs/{id}/finishes")]
async fn delete_dlc_finish(
    pool: web::Data<PgPool>,
    path: web::Path<ItemId>,
    body: web::Json<DateDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        dlc_finishes_service::delete_dlc_finish(&pool, &logged_user.id, &id, body.date).await;
    handle_delete_result(delete_result)
}
