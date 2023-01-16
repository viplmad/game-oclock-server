use std::fs::File;

use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::GameDTO;
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, handle_image_client_provider};
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

pub async fn set_game_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    game_id: i32,
    file_result: Result<File, ApiErrors>,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;
    let file = file_result?;

    games_service::exists_game(pool, user_id, game_id).await?;

    let format_filename = build_game_cover_filename(user_id, game_id, Option::<String>::None);
    let filename = image_client
        .upload_image(file, GAME_FOLDER, &format_filename)
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

    image_client
        .delete_image(GAME_FOLDER, &filename)
        .map_err(|_| ApiErrors::UnknownError(String::from("Image delete error.")))?;

    games_service::set_game_cover_filename(pool, user_id, game_id, Option::<String>::None).await
}

fn build_game_cover_filename(user_id: i32, game_id: i32, name: Option<String>) -> String {
    build_image_filename(user_id, game_id, GAME_HEADER_SUFFIX, name)
}
