use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{GameWithDate, PlatformWithDate};
use crate::errors::RepositoryError;
use crate::query::game_available_query;

use super::base::{execute, exists_id, fetch_all};

pub async fn find_all_games_with_platform(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
) -> Result<Vec<GameWithDate>, RepositoryError> {
    let query = game_available_query::select_all_games_by_platform_id(user_id, platform_id);
    fetch_all(pool, query).await
}

pub async fn find_all_platforms_with_game(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<PlatformWithDate>, RepositoryError> {
    let query = game_available_query::select_all_platforms_by_game_id(user_id, game_id);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    platform_id: i32,
    added_date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = game_available_query::insert(user_id, game_id, platform_id, added_date);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    platform_id: i32,
) -> Result<(), RepositoryError> {
    let query = game_available_query::delete_by_id(user_id, game_id, platform_id);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    platform_id: i32,
) -> Result<bool, RepositoryError> {
    let query = game_available_query::exists_by_id(user_id, game_id, platform_id);
    exists_id(pool, query).await
}
