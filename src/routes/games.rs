use actix_web::{Responder, delete, get, post, put, web};

use crate::models::{
    DateDTO, ErrorMessage, GameAvailableDTO, GameDTO, GamePageResult, ItemId, ItemIdAndRelatedId,
    LoggedUser, NewGameDTO, QuicksearchQuery, SearchDTO,
};
use crate::services::{
    GameAvailableService, GameGenreService, GamePlayedDeviceService, GameService, GameTagService,
};

use super::helpers::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_update_result,
};

/// Get a game
#[utoipa::path(
    get,
    path = "/api/v1/games/{id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Game obtained", body = GameDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}")]
pub async fn get_game(
    game_service: web::Data<GameService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_service.get_game(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Get all games with specified tag
#[utoipa::path(
    get,
    path = "/api/v1/tags/{id}/games",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Tag id"),
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/tags/{id}/games")]
pub async fn get_tag_games(
    game_tag_service: web::Data<GameTagService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_tag_service.get_tag_games(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Get all games avaiable in a location
#[utoipa::path(
    get,
    path = "/api/v1/locations/{id}/games",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/locations/{id}/games")]
pub async fn get_location_games(
    game_available_service: web::Data<GameAvailableService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_available_service
        .get_location_games(&logged_user.id, &id)
        .await;
    handle_get_result(get_result)
}

/// Get all games with specified genre
#[utoipa::path(
    get,
    path = "/api/v1/genres/{id}/games",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Genre id"),
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Genre not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/genres/{id}/games")]
pub async fn get_genre_games(
    game_genre_service: web::Data<GameGenreService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_genre_service
        .get_genre_games(&logged_user.id, &id)
        .await;
    handle_get_result(get_result)
}

/// Get games that have been played on specified device
#[utoipa::path(
    get,
    path = "/api/v1/devices/{id}/games",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Device id"),
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Device not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/devices/{id}/games")]
pub async fn get_device_games(
    game_played_device_service: web::Data<GamePlayedDeviceService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_played_device_service
        .get_device_played_games(&logged_user.id, &id)
        .await;
    handle_get_result(get_result)
}

/// Search games
#[utoipa::path(
    post,
    path = "/api/v1/games/list",
    tag = "Games",
    params(
        QuicksearchQuery,
    ),
    request_body(content = SearchDTO, description = "Query", content_type = "application/json"),
    responses(
        (status = 200, description = "Games obtained", body = GamePageResult, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[post("/games/list")]
pub async fn get_games(
    game_service: web::Data<GameService>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let search_result = game_service
        .search_games(&logged_user.id, body.0, query.0.q)
        .await;
    handle_get_result(search_result)
}

/// Create a game
#[utoipa::path(
    post,
    path = "/api/v1/games",
    tag = "Games",
    request_body(content = NewGameDTO, description = "Game to be created", content_type = "application/json"),
    responses(
        (status = 201, description = "Game created", body = GameDTO, content_type = "application/json"),
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
#[post("/games")]
pub async fn post_game(
    game_service: web::Data<GameService>,
    body: web::Json<NewGameDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let create_result = game_service.create_game(&logged_user.id, body.0).await;
    handle_create_result(create_result)
}

/// Update a game
#[utoipa::path(
    put,
    path = "/api/v1/games/{id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = NewGameDTO, description = "Game to be updated", content_type = "application/json"),
    responses(
        (status = 204, description = "Game updated"),
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
#[put("/games/{id}")]
pub async fn put_game(
    game_service: web::Data<GameService>,
    game_available_service: web::Data<GameAvailableService>,
    path: web::Path<ItemId>,
    body: web::Json<NewGameDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = game_service
        .update_game(&game_available_service, &logged_user.id, &id, body.0)
        .await;
    handle_update_result(update_result)
}

/// Add a tag to a game
#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/tags/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Tag id"),
    ),
    responses(
        (status = 204, description = "Game and Tag linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/games/{id}/tags/{other_id}")]
pub async fn link_game_tag(
    game_tag_service: web::Data<GameTagService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let create_result = game_tag_service
        .create_game_tag(&logged_user.id, &id, &tag_id)
        .await;
    handle_action_result(create_result)
}

/// Add a location as available to a game
#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/locations/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Location id"),
    ),
    request_body(content = DateDTO, description = "Available date", content_type = "application/json"),
    responses(
        (status = 204, description = "Game and Location linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Location not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/games/{id}/locations/{other_id}")]
pub async fn link_game_location(
    game_available_service: web::Data<GameAvailableService>,
    path: web::Path<ItemIdAndRelatedId>,
    body: web::Json<DateDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, location_id) = path.into_inner();
    let create_result = game_available_service
        .create_game_available(&logged_user.id, &id, &location_id, body.date)
        .await;
    handle_action_result(create_result)
}

/// Add genre to a game
#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/genres/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Genre id"),
    ),
    responses(
        (status = 204, description = "Game and Genre linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Genre not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/games/{id}/genres/{other_id}")]
pub async fn link_game_genre(
    game_genre_service: web::Data<GameGenreService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, genre_id) = path.into_inner();
    let create_result = game_genre_service
        .create_game_genre(&logged_user.id, &id, &genre_id)
        .await;
    handle_action_result(create_result)
}

/// Delete a game
#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 204, description = "Game deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/games/{id}")]
pub async fn delete_game(
    game_service: web::Data<GameService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = game_service.delete_game(&logged_user.id, &id).await;
    handle_delete_result(delete_result)
}

/// Remove tag from a game
#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/tags/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Tag id"),
    ),
    responses(
        (status = 204, description = "Game and Tag unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Tag not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/games/{id}/tags/{other_id}")]
pub async fn unlink_game_tag(
    game_tag_service: web::Data<GameTagService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let delete_result = game_tag_service
        .delete_game_tag(&logged_user.id, &id, &tag_id)
        .await;
    handle_action_result(delete_result)
}

/// Remove a location as available from a game
#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/locations/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Location id"),
    ),
    responses(
        (status = 204, description = "Game and Location unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game and Location relation not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/games/{id}/locations/{other_id}")]
pub async fn unlink_game_location(
    game_available_service: web::Data<GameAvailableService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, location_id) = path.into_inner();
    let delete_result = game_available_service
        .delete_game_available(&logged_user.id, &id, &location_id)
        .await;
    handle_action_result(delete_result)
}

/// Remove genre froma a game
#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/genres/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Genre id"),
    ),
    responses(
        (status = 204, description = "Game and Genre unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game and Genre relation not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/games/{id}/genres/{other_id}")]
pub async fn unlink_game_genre(
    game_genre_service: web::Data<GameGenreService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, genre_id) = path.into_inner();
    let delete_result = game_genre_service
        .delete_game_genre(&logged_user.id, &id, &genre_id)
        .await;
    handle_action_result(delete_result)
}

/// Get all DLCs of a game
#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/dlcs",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "DLCs obtained", body = [GameDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/dlcs")]
pub async fn get_game_dlcs(
    game_service: web::Data<GameService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_service.get_game_dlcs(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Get game base game
#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/base-game",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 200, description = "Game obtained", body = GameDTO, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/base-game")]
pub async fn get_dlc_base_game(
    game_service: web::Data<GameService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_service.get_game_base_game(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

/// Add a game as base game of another
#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/base-game/{other_id}",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
        ("other_id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 204, description = "DLC and Game linked"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "DLC or Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[put("/games/{id}/base-game/{other_id}")]
pub async fn link_dlc_game(
    game_service: web::Data<GameService>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, game_id) = path.into_inner();
    let update_result = game_service
        .set_game_base_game(&logged_user.id, &id, Some(game_id))
        .await;
    handle_action_result(update_result)
}

/// Remove a game base game
#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/base-game",
    tag = "DLCs",
    params(
        ("id" = String, Path, description = "DLC id"),
    ),
    responses(
        (status = 204, description = "DLC and Game unlinked"),
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
#[delete("/games/{id}/base-game")]
pub async fn unlink_dlc_game(
    game_service: web::Data<GameService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = game_service
        .set_game_base_game(&logged_user.id, &id, None)
        .await;
    handle_action_result(update_result)
}
