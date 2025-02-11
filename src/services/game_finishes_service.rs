use chrono::NaiveDate;

use crate::entities::{Finish, GameFinish};
use crate::errors::ApiErrors;
use crate::models::{FinishDTO, Merge, NewFinishDTO};
use crate::repository::{GameFinishRepository, GameRepository};

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_get_result_raw, handle_not_found_result, handle_result,
};
use super::games_service;

pub async fn get_first_game_finish(
    repository: &GameFinishRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<NaiveDate, ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let find_result = repository.find_first_by_game_id(user_id, game_id).await;
    handle_get_result_raw::<NaiveDate, FinishDTO>(find_result)
}

pub async fn get_game_finishes(
    repository: &GameFinishRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<FinishDTO>, ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let find_result = repository.find_all_by_game_id(user_id, game_id).await;
    handle_get_list_result::<Finish, FinishDTO>(find_result)
}

// For review
pub(super) async fn find_first_game_finishes_by_games(
    repository: &GameFinishRepository,
    user_id: &str,
    game_ids: Vec<String>,
) -> Result<Vec<GameFinish>, ApiErrors> {
    let find_result = repository
        .find_all_first_by_user_id_and_game_id_in(user_id, game_ids)
        .await;
    handle_result::<Vec<GameFinish>, FinishDTO>(find_result)
}

pub async fn create_game_finish(
    repository: &GameFinishRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
    finish: NewFinishDTO,
) -> Result<(), ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let exists_result = repository.exists_by_id(user_id, game_id, finish.date).await;
    handle_already_exists_result::<FinishDTO>(exists_result)?;

    let merged_new = FinishDTO::merge_with_default(finish);
    let finish_to_create = Finish::from(merged_new);
    let create_result = repository.create(user_id, game_id, &finish_to_create).await;
    handle_action_result::<FinishDTO>(create_result)
}

pub async fn delete_game_finish(
    repository: &GameFinishRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;
    exists_game_finish(repository, user_id, game_id, date).await?;

    let delete_result = repository.delete_by_id(user_id, game_id, date).await;
    handle_action_result::<FinishDTO>(delete_result)
}

pub async fn exists_game_finish(
    repository: &GameFinishRepository,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, game_id, date).await;
    handle_not_found_result::<FinishDTO>(exists_result)
}
