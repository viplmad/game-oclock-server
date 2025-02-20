use actix_web::{Responder, delete, get, post, web};

use crate::models::{ErrorMessage, ItemId, LinkDTO, LoggedUser, NewLinkDTO};
use crate::services::GameLinkService;

use super::helpers::{handle_action_result, handle_delete_result, handle_get_result};

#[utoipa::path(
    get,
    path = "/api/v1/games/{id}/links",
    tag = "GameLink",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 200, description = "Links obtained", body = [LinkDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[get("/games/{id}/links")]
pub async fn get_game_links(
    game_link_service: web::Data<GameLinkService>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let get_result = game_link_service.get_game_links(&logged_user.id, &id).await;
    handle_get_result(get_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/links",
    tag = "GameLink",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = NewLinkDTO, description = "Game link to be added", content_type = "application/json"),
    responses(
        (status = 204, description = "Game link added"),
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
#[post("/games/{id}/links")]
pub async fn post_game_link(
    game_link_service: web::Data<GameLinkService>,
    path: web::Path<ItemId>,
    body: web::Json<NewLinkDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let create_result = game_link_service
        .create_game_link(&logged_user.id, &id, body.0)
        .await;
    handle_action_result(create_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/links",
    tag = "GameLink",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = String, description = "Game link to be deleted", content_type = "application/json"),
    responses(
        (status = 204, description = "Game link deleted"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("OAuth2" = [])
    )
)]
#[delete("/games/{id}/links")]
pub async fn delete_game_link(
    game_link_service: web::Data<GameLinkService>,
    path: web::Path<ItemId>,
    body: web::Json<String>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result = game_link_service
        .delete_game_link(&logged_user.id, &id, &body.0)
        .await;
    handle_delete_result(delete_result)
}
