use std::fs::File;

use sqlx::PgPool;

use crate::clients::image_client::ImageClient;
use crate::errors::ApiErrors;
use crate::models::GameDTO;
use crate::providers::ImageClientProvider;

use super::games_service;

const IMAGE_FOLDER_GAME: &str = "Game";

pub fn add_game_cover(provider: &ImageClientProvider, game: &mut GameDTO) {
    if let Ok(client) = handle_image_client_provider(provider) {
        if let Some(cover_filename) = &game.cover_filename {
            game.cover_url = Some(client.get_image_uri(IMAGE_FOLDER_GAME, cover_filename));
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

    let initial_filename = build_header_filename(game_id);
    let filename = image_client
        .upload_image(file, IMAGE_FOLDER_GAME, &initial_filename)
        .map_err(|_| ApiErrors::UnknownError(String::from("Image upload error.")))?;

    games_service::set_game_cover(pool, user_id, game_id, &filename).await
}

fn handle_image_client_provider(
    provider: &ImageClientProvider,
) -> Result<&dyn ImageClient, ApiErrors> {
    if let Some(client) = provider.get_client() {
        Ok(client)
    } else {
        Err(ApiErrors::InvalidParameter(String::from(
            "Image client not set",
        )))
    }
}

fn build_header_filename(game_id: i32) -> String {
    format!("{game_id}-header")
}
