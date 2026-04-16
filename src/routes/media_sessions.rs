use actix_web::{Responder, delete, get, post, web};

use crate::models::{
    AggregateSearchDTO, DateTimeDTO, ErrorMessage, ItemId, ListSearchDTO, LoggedUser,
    MediaSessionPageResult, MediasReviewDTO, NewSessionDTO, OptionalStartEndDateQuery,
    QuicksearchQuery, SessionDTO, SessionPageResult, StartEndDateQuery,
};
use crate::services::{MediaReviewService, MediaSessionService, MediaWithSessionService};

use super::helpers::{handle_action_result, handle_delete_result, handle_get_result};

/// Get all media sessions
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/sessions/list",
    tag = "MediaSessions",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Sessions obtained", body = SessionPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/sessions/list")]
pub async fn get_media_sessions(
    media_session_service: web::Data<MediaSessionService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let search_result = media_session_service
        .search_media_sessions(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Aggregate all media sessions
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/sessions/aggregate",
    tag = "MediaSessions",
    params(
        ("id" = String, Path, description = "Media id"),
        QuicksearchQuery,
    ),
    request_body(content = AggregateSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Sessions aggregate obtained", body = u64, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/sessions/aggregate")]
pub async fn aggregate_media_sessions(
    media_session_service: web::Data<MediaSessionService>,
    path: web::Path<ItemId>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<AggregateSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let aggregate_result = media_session_service
        .aggregate_media_sessions(&logged_user.id, &id, body.0, query.0.q)
        .await;
    handle_get_result(aggregate_result)
}

/// Get a review in a time frame
#[utoipa::path(
    post,
    path = "/api/v1/medias/sessions/review",
    tag = "MediaSessions",
    params(
        StartEndDateQuery,
    ),
    responses(
        (status = 200, description = "Session medias review obtained", body = MediasReviewDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/sessions/review")]
pub async fn get_session_medias_review(
    media_review_service: web::Data<MediaReviewService>,
    query: web::Query<StartEndDateQuery>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = media_review_service
        .get_session_medias_review(&logged_user.id, query.start_date, query.end_date)
        .await;
    handle_get_result(get_result)
}

/// Search first medias by session
#[utoipa::path(
    post,
    path = "/api/v1/medias/sessions/first",
    tag = "MediaSessions",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias obtained", body = MediaSessionPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/sessions/first")]
pub async fn get_first_session_medias(
    media_with_session_service: web::Data<MediaWithSessionService>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = media_with_session_service
        .search_first_session_medias(
            &logged_user.id,
            query.start_date,
            query.end_date,
            body.0,
            quick_query.0.q,
        )
        .await;
    handle_get_result(get_result)
}

/// Search last medias by session
#[utoipa::path(
    post,
    path = "/api/v1/medias/sessions/last",
    tag = "MediaSessions",
    params(
        OptionalStartEndDateQuery,
        QuicksearchQuery,
    ),
    request_body(content = ListSearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Medias obtained", body = MediaSessionPageResult, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/sessions/last")]
pub async fn get_last_session_medias(
    media_with_session_service: web::Data<MediaWithSessionService>,
    query: web::Query<OptionalStartEndDateQuery>,
    quick_query: web::Query<QuicksearchQuery>,
    body: web::Json<ListSearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let get_result = media_with_session_service
        .search_last_session_medias(
            &logged_user.id,
            query.start_date,
            query.end_date,
            body.0,
            quick_query.0.q,
        )
        .await;
    handle_get_result(get_result)
}

/// Get a media session
#[utoipa::path(
    get,
    path = "/api/v1/medias/{id}/sessions",
    tag = "MediaSessions",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    request_body(content = DateTimeDTO, description = "Media session datetime", content_type = "application/json"),
    responses(
        (status = 200, description = "Media session obtained", body = SessionDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/medias/{id}/sessions")]
pub async fn get_media_session(
    media_session_service: web::Data<MediaSessionService>,
    path: web::Path<ItemId>,
    body: web::Json<DateTimeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = media_session_service
        .get_media_session(&logged_user.id, &id, body.datetime)
        .await;
    handle_get_result(get_result)
}

/// Create a media session
#[utoipa::path(
    post,
    path = "/api/v1/medias/{id}/sessions",
    tag = "MediaSessions",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    request_body(content = NewSessionDTO, description = "Media session to be added", content_type = "application/json"),
    responses(
        (status = 204, description = "Media session added"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/medias/{id}/sessions")]
pub async fn create_media_session(
    media_session_service: web::Data<MediaSessionService>,
    path: web::Path<ItemId>,
    body: web::Json<NewSessionDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result = media_session_service
        .create_media_session(&logged_user.id, &id, body.0)
        .await;
    handle_action_result(create_result)
}

/// Delete a media session
#[utoipa::path(
    delete,
    path = "/api/v1/medias/{id}/sessions",
    tag = "MediaSessions",
    params(
        ("id" = String, Path, description = "Media id"),
    ),
    request_body(content = DateTimeDTO, description = "Media session datetime to be deleted", content_type = "application/json"),
    responses(
        (status = 204, description = "Media session deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Media not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/medias/{id}/sessions")]
pub async fn delete_media_session(
    media_session_service: web::Data<MediaSessionService>,
    path: web::Path<ItemId>,
    body: web::Json<DateTimeDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = media_session_service
        .delete_media_session(&logged_user.id, &id, body.datetime)
        .await;
    handle_delete_result(delete_result)
}
