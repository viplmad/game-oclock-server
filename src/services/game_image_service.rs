use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{
    GameAvailableDTO, GameDTO, GameWithFinishDTO, GameWithLogDTO, GameWithLogsDTO,
};
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, extract_image_name, handle_image_client_provider};
use super::games_service;

const GAME_FOLDER: &str = "Game";
const GAME_HEADER_SUFFIX: &str = "header";

pub fn populate_game_cover(provider: &ImageClientProvider, game: &mut GameDTO) {
    if let Ok(client) = handle_image_client_provider(provider) {
        if let Some(cover_filename) = &game.cover_filename {
            game.cover_url = Some(client.get_image_uri(GAME_FOLDER, cover_filename));
        }
    }
}

pub fn populate_games_cover(provider: &ImageClientProvider, games: &mut Vec<GameDTO>) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for game in games {
            if let Some(cover_filename) = &game.cover_filename {
                game.cover_url = Some(client.get_image_uri(GAME_FOLDER, cover_filename));
            }
        }
    }
}

pub fn populate_games_available_cover(
    provider: &ImageClientProvider,
    games: &mut Vec<GameAvailableDTO>,
) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for game in games {
            if let Some(cover_filename) = &game.cover_filename {
                game.cover_url = Some(client.get_image_uri(GAME_FOLDER, cover_filename));
            }
        }
    }
}

pub fn populate_games_with_finish_cover(
    provider: &ImageClientProvider,
    games: &mut Vec<GameWithFinishDTO>,
) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for game in games {
            if let Some(cover_filename) = &game.cover_filename {
                game.cover_url = Some(client.get_image_uri(GAME_FOLDER, cover_filename));
            }
        }
    }
}

pub fn populate_games_with_log_cover(
    provider: &ImageClientProvider,
    games: &mut Vec<GameWithLogDTO>,
) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for game in games {
            if let Some(cover_filename) = &game.cover_filename {
                game.cover_url = Some(client.get_image_uri(GAME_FOLDER, cover_filename));
            }
        }
    }
}

pub fn populate_games_with_logs_cover(
    provider: &ImageClientProvider,
    games: &mut Vec<GameWithLogsDTO>,
) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for game in games {
            if let Some(cover_filename) = &game.cover_filename {
                game.cover_url = Some(client.get_image_uri(GAME_FOLDER, cover_filename));
            }
        }
    }
}

pub async fn set_game_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    game_id: i32,
    file_path: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    games_service::exists_game(pool, user_id, game_id).await?;

    let format_filename = build_game_cover_filename(user_id, game_id, Option::<String>::None);
    let filename = image_client
        .upload_image(file_path, GAME_FOLDER, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image upload error.")))?;

    games_service::set_game_cover_filename(pool, user_id, game_id, Some(filename)).await
}

pub async fn rename_game_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    game_id: i32,
    new_name: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let old_filename = games_service::get_game_cover_filename(pool, user_id, game_id).await?;

    let format_filename = build_game_cover_filename(user_id, game_id, Some(String::from(new_name)));
    let filename = image_client
        .rename_image(GAME_FOLDER, &old_filename, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image rename error.")))?;

    games_service::set_game_cover_filename(pool, user_id, game_id, Some(filename)).await
}

pub async fn delete_game_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    game_id: i32,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let filename = games_service::get_game_cover_filename(pool, user_id, game_id).await?;
    let name = extract_image_name(&filename);

    image_client
        .delete_image(GAME_FOLDER, name)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image delete error.")))?;

    games_service::set_game_cover_filename(pool, user_id, game_id, Option::<String>::None).await
}

fn build_game_cover_filename(user_id: i32, game_id: i32, name: Option<String>) -> String {
    build_image_filename(user_id, game_id, GAME_HEADER_SUFFIX, name)
}
