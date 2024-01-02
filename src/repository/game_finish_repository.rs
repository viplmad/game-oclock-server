use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::GameFinish;
use crate::errors::RepositoryError;
use crate::query::game_finish_query;

use super::base::{execute, execute_return_single, exists_id, fetch_all, fetch_all_single};

pub async fn find_first_by_game_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Option<NaiveDate>, RepositoryError> {
    let query = game_finish_query::select_min_date_by_user_id_and_game_id(user_id, game_id);
    execute_return_single(pool, query).await
}

pub async fn find_all_by_game_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<NaiveDate>, RepositoryError> {
    let query = game_finish_query::select_all_by_user_id_and_game_id(user_id, game_id);
    fetch_all_single(pool, query).await
}

pub async fn find_all_first_by_user_id_and_game_id_in(
    pool: &PgPool,
    user_id: &str,
    game_ids: Vec<String>,
) -> Result<Vec<GameFinish>, RepositoryError> {
    if game_ids.is_empty() {
        return Ok(vec![]);
    }

    let query = game_finish_query::select_all_first_by_user_id_and_game_id_in(user_id, game_ids);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = game_finish_query::insert(user_id, game_id, date);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = game_finish_query::delete_by_id(user_id, game_id, date);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    date: NaiveDate,
) -> Result<bool, RepositoryError> {
    let query = game_finish_query::exists_by_id(user_id, game_id, date);
    exists_id(pool, query).await
}
