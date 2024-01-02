use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::GameFinish;
use crate::errors::ApiErrors;
use crate::models::GameFinishDTO;
use crate::repository::game_finish_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result_raw,
    handle_get_result_raw, handle_not_found_result, handle_result,
};
use super::games_service;

pub async fn get_first_game_finish(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<NaiveDate, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_finish_repository::find_first_by_game_id(pool, user_id, game_id).await;
    handle_get_result_raw::<NaiveDate, GameFinishDTO>(find_result)
}

pub async fn get_game_finishes(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<NaiveDate>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_finish_repository::find_all_by_game_id(pool, user_id, game_id).await;
    handle_get_list_result_raw::<NaiveDate, GameFinishDTO>(find_result)
}

pub(super) async fn find_first_game_finishes_by_games(
    pool: &PgPool,
    user_id: &str,
    game_ids: Vec<String>,
) -> Result<Vec<GameFinish>, ApiErrors> {
    let find_result =
        game_finish_repository::find_all_first_by_user_id_and_game_id_in(pool, user_id, game_ids)
            .await;
    handle_result::<Vec<GameFinish>, GameFinishDTO>(find_result)
}

pub async fn create_game_finish(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let exists_result = game_finish_repository::exists_by_id(pool, user_id, game_id, date).await;
    handle_already_exists_result::<GameFinishDTO>(exists_result)?;

    let create_result = game_finish_repository::create(pool, user_id, game_id, date).await;
    handle_action_result::<GameFinishDTO>(create_result)
}

pub async fn delete_game_finish(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;
    exists_game_finish(pool, user_id, game_id, date).await?;

    let delete_result = game_finish_repository::delete_by_id(pool, user_id, game_id, date).await;
    handle_action_result::<GameFinishDTO>(delete_result)
}

pub async fn exists_game_finish(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    let exists_result = game_finish_repository::exists_by_id(pool, user_id, game_id, date).await;
    handle_not_found_result::<GameFinishDTO>(exists_result)
}
