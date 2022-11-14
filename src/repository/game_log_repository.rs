use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::{errors::RepositoryError, entities::GameLog};
use crate::query::game_log_query;

use super::base::{execute, exists_id, fetch_all};

pub async fn find_all_by_game_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<GameLog>, RepositoryError> {
    let query = game_log_query::select_all_by_user_id_and_game_id(user_id, game_id);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    log: &GameLog,
) -> Result<(), RepositoryError> {
    let query = game_log_query::insert(user_id, game_id, log);
    execute(pool, query).await
}

pub async fn delete(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    datetime: NaiveDateTime,
) -> Result<(), RepositoryError> {
    let query = game_log_query::delete_by_id(user_id, game_id, datetime);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    datetime: NaiveDateTime,
) -> Result<bool, RepositoryError> {
    let query = game_log_query::exists_by_id(user_id, game_id, datetime);
    exists_id(pool, query).await
}
