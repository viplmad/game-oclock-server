use chrono::NaiveDate;
use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{GameAvailableDTO, PlatformAvailableDTO};
use crate::repository::game_available_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::{games_service, platforms_service};

pub async fn get_platform_games(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
) -> Result<Vec<GameAvailableDTO>, ApiErrors> {
    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let find_result =
        game_available_repository::find_all_games_with_platform(pool, user_id, platform_id).await;
    handle_get_list_result(find_result)
}

pub async fn get_game_platforms(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<PlatformAvailableDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result =
        game_available_repository::find_all_platforms_with_game(pool, user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn create_game_available(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    platform_id: i32,
    available_date: NaiveDate,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;
    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let exists_result =
        game_available_repository::exists_by_id(pool, user_id, game_id, platform_id).await;
    handle_already_exists_result::<GameAvailableDTO>(exists_result)?;

    let create_result =
        game_available_repository::create(pool, user_id, game_id, platform_id, available_date)
            .await;
    handle_action_result::<GameAvailableDTO>(create_result)
}

pub async fn delete_game_available(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    platform_id: i32,
) -> Result<(), ApiErrors> {
    exists_game_available(pool, user_id, game_id, platform_id).await?;

    let delete_result =
        game_available_repository::delete_by_id(pool, user_id, game_id, platform_id).await;
    handle_action_result::<GameAvailableDTO>(delete_result)
}

pub async fn exists_game_available(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    platform_id: i32,
) -> Result<(), ApiErrors> {
    let exists_result =
        game_available_repository::exists_by_id(pool, user_id, game_id, platform_id).await;
    handle_not_found_result::<GameAvailableDTO>(exists_result)
}
