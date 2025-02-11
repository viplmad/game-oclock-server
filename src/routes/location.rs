use actix_web::{delete, get, post, put, web, Responder};

use crate::models::{
    ErrorMessage, ItemId, LocationAvailableDTO, LocationDTO, LocationPageResult, LoggedUser,
    NewLocationDTO, QuicksearchQuery, SearchDTO,
};
use crate::repository::{GameAvailableRepository, GameRepository, LocationRepository};
use crate::services::{game_available_service, locations_service};

use super::base::{
    handle_create_result, handle_delete_result, handle_get_result, handle_update_result,
};

#[utoipa::path(
    get,
    path = "/api/v1/locations/{id}",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 200, description = "Location obtained", body = LocationDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/locations/{id}")]
pub async fn get_location(
    location_repository: web::Data<LocationRepository>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result =
        locations_service::get_location(&location_repository, &logged_user.id, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/locations",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Locations obtained", body = [LocationAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/locations")]
pub async fn get_game_locations(
    game_available_repository: web::Data<GameAvailableRepository>,
    game_repository: web::Data<GameRepository>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_available_service::get_game_locations(
        &game_available_repository,
        &game_repository,
        &logged_user.id,
        &id,
    )
    .await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/locations/list",
    tag = "Locations",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Locations obtained", body = LocationPageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/locations/list")]
pub async fn get_locations(
    location_repository: web::Data<LocationRepository>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = locations_service::search_locations(
        &location_repository,
        &logged_user.id,
        body.0,
        query.0.q,
    )
    .await;
    handle_get_result(search_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/locations",
    tag = "Locations",
    request_body(content = NewLocationDTO, description = "Location to be createad", content_type = "application/json"),
    responses(
        (status = 201, description = "Location created", body = LocationDTO, content_type = "application/json"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/locations")]
pub async fn post_location(
    location_repository: web::Data<LocationRepository>,
    body: web::Json<NewLocationDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result =
        locations_service::create_location(&location_repository, &logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/locations/{id}",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    request_body(content = NewLocationDTO, description = "Location to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Location updated"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/locations/{id}")]
pub async fn put_location(
    location_repository: web::Data<LocationRepository>,
    path: web::Path<ItemId>,
    body: web::Json<NewLocationDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result =
        locations_service::update_location(&location_repository, &logged_user.id, &id, body.0)
            .await;
    handle_update_result(update_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/locations/{id}",
    tag = "Locations",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 204, description = "Location deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/locations/{id}")]
pub async fn delete_location(
    location_repository: web::Data<LocationRepository>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        locations_service::delete_location(&location_repository, &logged_user.id, &id).await;
    handle_delete_result(delete_result)
}
