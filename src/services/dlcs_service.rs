use crate::errors::{error_message_builder, ApiErrors};
use crate::models::GameDTO;
use crate::repository::GameRepository;

use super::base::{handle_action_result, handle_get_list_result};
use super::games_service;

pub async fn get_dlc_base_game(
    repository: &GameRepository,
    user_id: &str,
    dlc_id: &str,
) -> Result<GameDTO, ApiErrors> {
    let dlc = games_service::get_game(repository, user_id, dlc_id).await?;
    let base_game_id = dlc.base_game_id.ok_or_else(|| {
        ApiErrors::InvalidParameter(error_message_builder::empty_param("DLC base game"))
    })?;
    games_service::get_game(repository, user_id, &base_game_id).await
}

pub async fn get_game_dlcs(
    repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<GameDTO>, ApiErrors> {
    games_service::exists_game(repository, user_id, game_id).await?;

    let find_result = repository.find_all_by_base_game_id(user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn set_dlc_base_game(
    repository: &GameRepository,
    user_id: &str,
    dlc_id: &str,
    base_game_id: Option<String>,
) -> Result<(), ApiErrors> {
    games_service::exists_game(repository, user_id, dlc_id).await?;

    // TODO
    // check dlc is not already in other base_game
    // check game to set as base_game is not a dlc

    if let Some(game_id) = &base_game_id {
        games_service::exists_game(repository, user_id, game_id).await?;
    }

    let update_result = repository
        .update_base_game_id(user_id, dlc_id, base_game_id)
        .await;
    handle_action_result::<GameDTO>(update_result)
}
