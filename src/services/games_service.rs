use sqlx::PgPool;

use crate::entities::GameSearch;
use crate::errors::{error_message_builder, ApiErrors};
use crate::models::{GameDTO, GamePageResult, NewGameDTO, SearchDTO};
use crate::providers::ImageClientProvider;
use crate::repository::game_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};
use super::game_image_service;

pub async fn get_game(pool: &PgPool, user_id: &str, game_id: &str) -> Result<GameDTO, ApiErrors> {
    let find_result = game_repository::find_by_id(pool, user_id, game_id).await;
    handle_get_result(find_result)
}

pub async fn search_games(
    pool: &PgPool,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GamePageResult, ApiErrors> {
    let search = handle_query_mapping::<GameDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_game(
    pool: &PgPool,
    user_id: &str,
    game: NewGameDTO,
) -> Result<GameDTO, ApiErrors> {
    create_merged(
        game,
        async move |created_game_id| get_game(pool, user_id, &created_game_id).await,
        async move |game_to_create| {
            let exists_result =
                game_repository::exists_with_unique(pool, user_id, &game_to_create).await;
            handle_already_exists_result::<GameDTO>(exists_result)?;

            let create_result = game_repository::create(pool, user_id, &game_to_create).await;
            handle_create_result::<String, GameDTO>(create_result)
        },
    )
    .await
}

pub async fn update_game(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    game: NewGameDTO,
) -> Result<(), ApiErrors> {
    update_merged(
        game,
        async move || get_game(pool, user_id, game_id).await,
        async move |game_to_update| {
            let exists_result = game_repository::exists_with_unique_except_id(
                pool,
                user_id,
                &game_to_update,
                game_id,
            )
            .await;
            handle_already_exists_result::<GameDTO>(exists_result)?;

            let update_result =
                game_repository::update_by_id(pool, user_id, game_id, &game_to_update).await;
            handle_update_result::<GameDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_game(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    game_id: &str,
) -> Result<(), ApiErrors> {
    let game = get_game(pool, user_id, game_id).await?;

    if let Some(cover_filename) = &game.cover_filename {
        let delete_cover_result =
            game_image_service::delete_game_cover(image_client_provider, cover_filename).await;
        if delete_cover_result.is_err() {
            log::warn!("Game deletion - Image client could not delete Game with image.")
        }
    }

    let delete_result = game_repository::delete_by_id(pool, user_id, game_id).await;
    handle_action_result::<GameDTO>(delete_result)
}

pub async fn set_game_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    game_id: &str,
    file_path: &str,
) -> Result<(), ApiErrors> {
    exists_game(pool, user_id, game_id).await?;
    let filename =
        game_image_service::set_game_cover(image_client_provider, user_id, game_id, file_path)
            .await?;
    set_game_cover_filename(pool, user_id, game_id, Some(filename)).await
}

pub async fn rename_game_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    game_id: &str,
    new_name: &str,
) -> Result<(), ApiErrors> {
    let old_filename = get_game_cover_filename(pool, user_id, game_id).await?;
    let new_filename = game_image_service::rename_game_cover(
        image_client_provider,
        user_id,
        game_id,
        &old_filename,
        new_name,
    )
    .await?;
    set_game_cover_filename(pool, user_id, game_id, Some(new_filename)).await
}

pub async fn delete_game_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    game_id: &str,
) -> Result<(), ApiErrors> {
    let filename = get_game_cover_filename(pool, user_id, game_id).await?;
    game_image_service::delete_game_cover(image_client_provider, &filename).await?;
    set_game_cover_filename(pool, user_id, game_id, Option::<String>::None).await
}

pub async fn exists_game(pool: &PgPool, user_id: &str, game_id: &str) -> Result<(), ApiErrors> {
    let exists_result = game_repository::exists_by_id(pool, user_id, game_id).await;
    handle_not_found_result::<GameDTO>(exists_result)
}

async fn get_game_cover_filename(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<String, ApiErrors> {
    let game = get_game(pool, user_id, game_id).await?;
    game.cover_filename.ok_or_else(|| {
        ApiErrors::InvalidParameter(error_message_builder::empty_param("Game cover"))
    })
}

async fn set_game_cover_filename(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    filename: Option<String>,
) -> Result<(), ApiErrors> {
    let update_result =
        game_repository::update_cover_filename_by_id(pool, user_id, game_id, filename).await;
    handle_action_result::<GameDTO>(update_result)
}
