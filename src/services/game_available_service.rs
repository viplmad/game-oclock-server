use chrono::NaiveDate;

use crate::errors::ApiErrors;
use crate::models::{GameAvailableDTO, GameStatus, LocationAvailableDTO, NewGameDTO};
use crate::repository::{GameAvailableRepository, GameRepository, LocationRepository};

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result, handle_result,
};
use super::{games_service, locations_service};

pub async fn get_location_games(
    repository: &GameAvailableRepository,
    location_repository: &LocationRepository,
    user_id: &str,
    location_id: &str,
) -> Result<Vec<GameAvailableDTO>, ApiErrors> {
    locations_service::exists_location(location_repository, user_id, location_id).await?;

    let find_result = repository
        .find_all_games_with_location(user_id, location_id)
        .await;
    handle_get_list_result(find_result)
}

pub async fn get_game_locations(
    repository: &GameAvailableRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<LocationAvailableDTO>, ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let find_result = repository
        .find_all_locations_with_game(user_id, game_id)
        .await;
    handle_get_list_result(find_result)
}

pub async fn create_game_available(
    repository: &GameAvailableRepository,
    game_repository: &GameRepository,
    location_repository: &LocationRepository,
    user_id: &str,
    game_id: &str,
    location_id: &str,
    available_date: NaiveDate,
) -> Result<(), ApiErrors> {
    let game = games_service::get_game(game_repository, user_id, game_id).await?;
    locations_service::exists_location(location_repository, user_id, location_id).await?;

    let exists_result = repository.exists_by_id(user_id, game_id, location_id).await;
    handle_already_exists_result::<GameAvailableDTO>(exists_result)?;

    if game.status == GameStatus::Wishlist {
        games_service::update_game(
            game_repository,
            repository,
            user_id,
            game_id,
            NewGameDTO {
                status: Some(GameStatus::NextUp),
                title: None,
                edition: None,
                release_date: None,
                rating: None,
                notes: None,
            },
        )
        .await?
    }

    let create_result = repository
        .create(user_id, game_id, location_id, available_date)
        .await;
    handle_action_result::<GameAvailableDTO>(create_result)
}

pub async fn delete_game_available(
    repository: &GameAvailableRepository,
    user_id: &str,
    game_id: &str,
    location_id: &str,
) -> Result<(), ApiErrors> {
    exists_game_available(repository, user_id, game_id, location_id).await?;

    let delete_result = repository.delete_by_id(user_id, game_id, location_id).await;
    handle_action_result::<GameAvailableDTO>(delete_result)
}

pub async fn exists_game_available(
    repository: &GameAvailableRepository,
    user_id: &str,
    game_id: &str,
    location_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, game_id, location_id).await;
    handle_not_found_result::<GameAvailableDTO>(exists_result)
}

pub async fn exists_no_game_available(
    repository: &GameAvailableRepository,
    user_id: &str,
    game_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository
        .exists_locations_with_game(user_id, game_id)
        .await;
    let exists = handle_result::<bool, GameAvailableDTO>(exists_result)?;
    match exists {
        true => Err(ApiErrors::AlreadyExists(String::from(
            "Game has locations available",
        ))),
        false => Ok(()),
    }
}
