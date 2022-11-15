use chrono::NaiveDate;
use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::GameFinish;
use crate::repository::game_finish_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result_raw,
    handle_not_found_result,
};
use super::games_service;

pub async fn get_game_finishes(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<NaiveDate>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_finish_repository::find_all_by_game_id(pool, user_id, game_id).await;
    handle_get_list_result_raw::<NaiveDate, GameFinish>(find_result)
}

pub async fn create_game_finish(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let exists_result = game_finish_repository::exists_by_id(pool, user_id, game_id, date).await;
    handle_already_exists_result::<GameFinish>(exists_result)?;

    let create_result = game_finish_repository::create(pool, user_id, game_id, date).await;
    handle_action_result::<GameFinish>(create_result)
}

pub async fn delete_game_finish(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;
    exists_game_finish(pool, user_id, game_id, date).await?;

    let delete_result = game_finish_repository::delete_by_id(pool, user_id, game_id, date).await;
    handle_action_result::<GameFinish>(delete_result)
}

pub async fn exists_game_finish(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    let exists_result = game_finish_repository::exists_by_id(pool, user_id, game_id, date).await;
    handle_not_found_result::<GameFinish>(exists_result)
}
