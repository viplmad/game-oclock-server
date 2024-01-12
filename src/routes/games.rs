use actix_web::{delete, get, post, put, web, Responder};
use sqlx::PgPool;

use crate::models::{
    DateDTO, FileTempPath, ItemId, ItemIdAndRelatedId, LoggedUser, NewGameDTO, QuicksearchQuery,
    SearchDTO,
};
use crate::providers::ImageClientProvider;
use crate::services::{
    game_available_service, game_image_service, game_tags_service, games_service,
};

use super::base::{
    handle_action_result, handle_create_result, handle_delete_result, handle_get_result,
    handle_multipart_result, handle_update_result, populate_get_page_result, populate_get_result,
};

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
        ("bearer_token" = [])
    )
)]
#[get("/games/{id}")]
async fn get_game(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = games_service::get_game(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |game| {
        game_image_service::populate_game_cover(&image_client_provider, game)
    });
    handle_get_result(get_result)
}

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
        ("bearer_token" = [])
    )
)]
#[get("/tags/{id}/games")]
async fn get_tag_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result = game_tags_service::get_tag_games(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |games| {
        game_image_service::populate_games_cover(&image_client_provider, games)
    });
    handle_get_result(get_result)
}

#[utoipa::path(
    get,
    path = "/api/v1/platforms/{id}/games",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Platform id"),
    ),
    responses(
        (status = 200, description = "Games obtained", body = [GameAvailableDTO], content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[get("/platforms/{id}/games")]
async fn get_platform_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let mut get_result =
        game_available_service::get_platform_games(&pool, &logged_user.id, &id).await;
    populate_get_result(&mut get_result, |games| {
        game_image_service::populate_games_available_cover(&image_client_provider, games)
    });
    handle_get_result(get_result)
}

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
        ("bearer_token" = [])
    )
)]
#[post("/games/list")]
async fn get_games(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    query: web::Query<QuicksearchQuery>,
    body: web::Json<SearchDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let mut search_result =
        games_service::search_games(&pool, &logged_user.id, body.0, query.0.q).await;
    populate_get_page_result(&mut search_result, |games| {
        game_image_service::populate_games_cover(&image_client_provider, games)
    });
    handle_get_result(search_result)
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
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
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
    let create_result = games_service::create_game(&pool, &logged_user.id, body.0).await;
    handle_create_result(create_result)
}

#[utoipa::path(
    post,
    path = "/api/v1/games/{id}/cover",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = Image, description = "Game cover to be uploaded", content_type = "multipart/form-data"),
    responses(
        (status = 204, description = "Game cover uploaded"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[post("/games/{id}/cover")]
async fn post_game_cover(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: actix_multipart::Multipart,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();

    let file_path_result = crate::multipart_utils::get_multipart_file_path(body).await;
    let FileTempPath {
        directory_path,
        file_path,
    } = match handle_multipart_result(file_path_result) {
        Ok(res) => res,
        Err(err) => return err,
    };

    let upload_result = games_service::set_game_cover(
        &pool,
        &image_client_provider,
        &logged_user.id,
        &id,
        &file_path,
    )
    .await;

    crate::temp_file_utils::delete_temp_dir(&directory_path).await;

    handle_action_result(upload_result)
}

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
    let update_result = games_service::update_game(&pool, &logged_user.id, &id, body.0).await;
    handle_update_result(update_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/cover",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    request_body(content = String, description = "New game cover name", content_type = "application/json"),
    responses(
        (status = 204, description = "Game cover renamed"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}/cover")]
async fn put_game_cover(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    body: web::Json<String>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let update_result = games_service::rename_game_cover(
        &pool,
        &image_client_provider,
        &logged_user.id,
        &id,
        &body.0,
    )
    .await;
    handle_action_result(update_result)
}

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
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}/tags/{other_id}")]
async fn link_game_tag(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let create_result =
        game_tags_service::create_game_tag(&pool, &logged_user.id, &id, &tag_id).await;
    handle_action_result(create_result)
}

#[utoipa::path(
    put,
    path = "/api/v1/games/{id}/platforms/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Platform id"),
    ),
    request_body(content = DateDTO, description = "Available date", content_type = "application/json"),
    responses(
        (status = 204, description = "Game and Platform linked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game or Platform not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[put("/games/{id}/platforms/{other_id}")]
async fn link_game_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    body: web::Json<DateDTO>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, platform_id) = path.into_inner();
    let create_result = game_available_service::create_game_available(
        &pool,
        &logged_user.id,
        &id,
        &platform_id,
        body.date,
    )
    .await;
    handle_action_result(create_result)
}

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
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}")]
async fn delete_game(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        games_service::delete_game(&pool, &image_client_provider, &logged_user.id, &id).await;
    handle_delete_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/cover",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
    ),
    responses(
        (status = 204, description = "Game cover deleted"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/cover")]
async fn delete_game_cover(
    pool: web::Data<PgPool>,
    image_client_provider: web::Data<ImageClientProvider>,
    path: web::Path<ItemId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemId(id) = path.into_inner();
    let delete_result =
        games_service::delete_game_cover(&pool, &image_client_provider, &logged_user.id, &id).await;
    handle_action_result(delete_result)
}

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
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/tags/{other_id}")]
async fn unlink_game_tag(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, tag_id) = path.into_inner();
    let delete_result =
        game_tags_service::delete_game_tag(&pool, &logged_user.id, &id, &tag_id).await;
    handle_action_result(delete_result)
}

#[utoipa::path(
    delete,
    path = "/api/v1/games/{id}/platforms/{other_id}",
    tag = "Games",
    params(
        ("id" = String, Path, description = "Game id"),
        ("other_id" = String, Path, description = "Platform id"),
    ),
    responses(
        (status = 204, description = "Game and Platform unlinked"),
        (status = 400, description = "Bad request", body = ErrorMessage, content_type = "application/json"),
        (status = 401, description = "Unauthorized", body = ErrorMessage, content_type = "application/json"),
        (status = 403, description = "Forbidden", body = ErrorMessage, content_type = "application/json"),
        (status = 404, description = "Game and Platform relation not found", body = ErrorMessage, content_type = "application/json"),
        (status = 500, description = "Internal server error", body = ErrorMessage, content_type = "application/json"),
    ),
    security(
        ("bearer_token" = [])
    )
)]
#[delete("/games/{id}/platforms/{other_id}")]
async fn unlink_game_platform(
    pool: web::Data<PgPool>,
    path: web::Path<ItemIdAndRelatedId>,
    logged_user: LoggedUser,
) -> impl Responder {
    let ItemIdAndRelatedId(id, platform_id) = path.into_inner();
    let delete_result =
        game_available_service::delete_game_available(&pool, &logged_user.id, &id, &platform_id)
            .await;
    handle_action_result(delete_result)
}
