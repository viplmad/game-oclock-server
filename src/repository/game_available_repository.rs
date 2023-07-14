use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{GameWithDate, PlatformWithDate};
use crate::errors::RepositoryError;
use crate::query::game_available_query;

use super::base::{execute, exists_id, fetch_all};

pub async fn find_all_games_with_platform(
    pool: &PgPool,
    user_id: &str,
    platform_id: &str,
) -> Result<Vec<GameWithDate>, RepositoryError> {
    let query = game_available_query::select_all_games_by_platform_id_order_by_added_date(
        user_id,
        platform_id,
    );
    fetch_all(pool, query).await
}

pub async fn find_all_platforms_with_game(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<PlatformWithDate>, RepositoryError> {
    let query =
        game_available_query::select_all_platforms_by_game_id_order_by_added_date(user_id, game_id);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    platform_id: &str,
    added_date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = game_available_query::insert(user_id, game_id, platform_id, added_date);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    platform_id: &str,
) -> Result<(), RepositoryError> {
    let query = game_available_query::delete_by_id(user_id, game_id, platform_id);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    platform_id: &str,
) -> Result<bool, RepositoryError> {
    let query = game_available_query::exists_by_id(user_id, game_id, platform_id);
    exists_id(pool, query).await
}

pub async fn exists_platforms_with_game(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<bool, RepositoryError> {
    let query = game_available_query::exists_platforms_by_game_id(user_id, game_id);
    exists_id(pool, query).await
}
