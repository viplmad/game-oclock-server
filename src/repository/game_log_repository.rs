use chrono::NaiveDateTime;
use sqlx::{postgres::types::PgInterval, PgPool};

use crate::entities::{GameLog, GameLogWithTime};
use crate::errors::RepositoryError;
use crate::query::game_log_query;

use super::base::{execute, execute_return_single, exists_id, fetch_all};

pub async fn find_sum_time_by_game_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<PgInterval, RepositoryError> {
    let query = game_log_query::select_sum_time_by_user_id_and_game_id(user_id, game_id);
    execute_return_single(pool, query).await
}

pub async fn find_all_by_game_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<GameLogWithTime>, RepositoryError> {
    let query = game_log_query::select_all_by_user_id_and_game_id(user_id, game_id);
    fetch_all(pool, query).await
}

pub async fn find_all_first_by_user_id_and_game_id_in(
    pool: &PgPool,
    user_id: &str,
    game_ids: Vec<String>,
) -> Result<Vec<GameLogWithTime>, RepositoryError> {
    if game_ids.is_empty() {
        return Ok(vec![]);
    }

    let query = game_log_query::select_all_first_by_user_id_and_game_id_in(user_id, game_ids);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    log: &GameLog,
) -> Result<(), RepositoryError> {
    let query = game_log_query::insert(user_id, game_id, log);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    datetime: NaiveDateTime,
) -> Result<(), RepositoryError> {
    let query = game_log_query::delete_by_id(user_id, game_id, datetime);
    execute(pool, query).await
}

pub async fn exists_gap(
    pool: &PgPool,
    user_id: &str,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> Result<bool, RepositoryError> {
    let query = game_log_query::exists_by_start_datetime_lt_or_end_datetime_gt(
        user_id,
        end_datetime,
        start_datetime,
    );
    exists_id(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    datetime: NaiveDateTime,
) -> Result<bool, RepositoryError> {
    let query = game_log_query::exists_by_id(user_id, game_id, datetime);
    exists_id(pool, query).await
}
