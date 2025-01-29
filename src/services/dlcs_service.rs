use sqlx::PgPool;

use crate::errors::{error_message_builder, ApiErrors};
use crate::models::GameDTO;
use crate::repository::game_repository;

use super::base::{handle_action_result, handle_get_list_result};
use super::games_service;

pub async fn get_dlc_base_game(
    pool: &PgPool,
    user_id: &str,
    dlc_id: &str,
) -> Result<GameDTO, ApiErrors> {
    let dlc = games_service::get_game(pool, user_id, dlc_id).await?;
    let base_game_id = dlc.base_game_id.ok_or_else(|| {
        ApiErrors::InvalidParameter(error_message_builder::empty_param("DLC base game"))
    })?;
    games_service::get_game(pool, user_id, &base_game_id).await
}

pub async fn get_game_dlcs(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<GameDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_repository::find_all_by_base_game_id(pool, user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn set_dlc_base_game(
    pool: &PgPool,
    user_id: &str,
    dlc_id: &str,
    base_game_id: Option<String>,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, dlc_id).await?;

    // TODO
    // check dlc is not already in other base_game
    // check game to set as base_game is not a dlc

    if let Some(game_id) = &base_game_id {
        games_service::exists_game(pool, user_id, game_id).await?;
    }

    let update_result =
        game_repository::update_base_game_id(pool, user_id, dlc_id, base_game_id).await;
    handle_action_result::<GameDTO>(update_result)
}
