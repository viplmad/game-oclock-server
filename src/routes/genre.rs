use actix_web::{delete, get, post, put, web, Responder};

use crate::models::{
    ErrorMessage, GenreDTO, GenrePageResult, ItemId, LoggedUser, NewGenreDTO, QuicksearchQuery,
    SearchDTO,
};
use crate::services::{GameGenreService, GenreService};

use super::base::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/genres/{id}",
    tag = "Genres",
    params(
        ("id" = String, Path, description = "Genre id"),
    ),
    responses(
        (status = 200, description = "Genre obtained", body = GenreDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Genre not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/genres/{id}")]
pub async fn get_genre(
    genre_service: web::Data<GenreService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = genre_service.get_genre(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/genres",
    tag = "Genres",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Genres obtained", body = [GenreDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/genres")]
pub async fn get_game_genres(
    game_genre_service: web::Data<GameGenreService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_genre_service
        .get_game_genres(&logged_user.id, &id)
        .await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/genres/list",
    tag = "Genres",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Genres obtained", body = GenrePageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/genres/list")]
pub async fn get_genres(
    genre_service: web::Data<GenreService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = genre_service
        .search_genres(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/genres",
    tag = "Genres",
    request_body(content = NewGenreDTO, description = "Genre to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Genre created", body = GenreDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Genre not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/genres")]
pub async fn post_genre(
    genre_service: web::Data<GenreService>,
    body: web::Json<NewGenreDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = genre_service.create_genre(&logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/genres/{id}",
    tag = "Genres",
    params(
        ("id" = String, Path, description = "Genre id"),
    ),
    request_body(content = NewGenreDTO, description = "Genre to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Genre updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Genre not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/genres/{id}")]
pub async fn put_genre(
    genre_service: web::Data<GenreService>,
    path: web::Path<ItemId>,
    body: web::Json<NewGenreDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = genre_service
        .update_genre(&logged_user.id, &id, body.0)
        .await;
    handle_update_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/genres/{id}",
    tag = "Genres",
    params(
        ("id" = String, Path, description = "Genre id"),
    ),
    responses(
        (status = 204, description = "Genre deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Genre not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/genres/{id}")]
pub async fn delete_genre(
    genre_service: web::Data<GenreService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = genre_service.delete_genre(&logged_user.id, &id).await;
    handle_delete_result(delete_result)
}
