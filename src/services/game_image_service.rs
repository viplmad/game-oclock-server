use crate::errors::ApiErrors;
use crate::models::{
    GameAvailableDTO, GameDTO, GamePlayedReviewDTO, GameWithFinishDTO, GameWithLogDTO,
    GameWithLogsDTO,
};
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, extract_image_name, handle_image_client_provider};

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

pub fn populate_games_played_review_cover(
    provider: &ImageClientProvider,
    games: &mut Vec<GamePlayedReviewDTO>,
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
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    game_id: &str,
    file_path: &str,
) -> Result<String, ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let format_filename = build_game_cover_filename(user_id, game_id, Option::<String>::None);
    image_client
        .upload_image(file_path, GAME_FOLDER, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image upload error.")))
}

pub async fn rename_game_cover(
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    game_id: &str,
    old_filename: &str,
    new_name: &str,
) -> Result<String, ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let old_name = extract_image_name(old_filename)?;

    let format_filename = build_game_cover_filename(user_id, game_id, Some(String::from(new_name)));
    image_client
        .rename_image(GAME_FOLDER, &old_name, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image rename error.")))
}

pub async fn delete_game_cover(
    image_client_provider: &ImageClientProvider,
    filename: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let name = extract_image_name(filename)?;

    image_client
        .delete_image(GAME_FOLDER, &name)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image delete error.")))
}

fn build_game_cover_filename(user_id: &str, game_id: &str, name: Option<String>) -> String {
    build_image_filename(user_id, game_id, GAME_HEADER_SUFFIX, name)
}
