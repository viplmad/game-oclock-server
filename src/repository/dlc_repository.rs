use sqlx::PgPool;

use crate::entities::{DLCIden, Query, DLC};
use crate::errors::RepositoryError;
use crate::query::dlc_query;

use super::base::{execute, execute_return_id, exists_id, fetch_all, fetch_optional};

pub async fn find_by_id(
    pool: &PgPool,
    user_id: i32,
    id: i32,
) -> Result<Option<DLC>, RepositoryError> {
    let query = dlc_query::select_by_id(user_id, id);
    fetch_optional(pool, query).await
}

pub async fn find_all_by_base_game_id(
    pool: &PgPool,
    user_id: i32,
    base_game_id: i32,
) -> Result<Vec<DLC>, RepositoryError> {
    let query = dlc_query::select_all_by_base_game_id(user_id, base_game_id);
    fetch_all(pool, query).await
}

pub async fn find_all(
    pool: &PgPool,
    user_id: i32,
    query: Query<DLCIden>,
) -> Result<Vec<DLC>, RepositoryError> {
    let query = dlc_query::select_all_by_query(user_id, query);
    fetch_all(pool, query).await
}

pub async fn create(pool: &PgPool, user_id: i32, dlc: &DLC) -> Result<i32, RepositoryError> {
    let query = dlc_query::insert(user_id, dlc);
    execute_return_id(pool, query).await
}

pub async fn update_by_id(
    pool: &PgPool,
    user_id: i32,
    id: i32,
    dlc: &DLC,
) -> Result<i32, RepositoryError> {
    let query = dlc_query::update_by_id(user_id, id, dlc);
    execute_return_id(pool, query).await
}

pub async fn update_base_game_id(
    pool: &PgPool,
    user_id: i32,
    id: i32,
    base_game_id: Option<i32>,
) -> Result<(), RepositoryError> {
    let query = dlc_query::update_base_game_id_by_id(user_id, id, base_game_id);
    execute(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, user_id: i32, id: i32) -> Result<(), RepositoryError> {
    let query = dlc_query::delete_by_id(user_id, id);
    execute(pool, query).await
}

pub async fn exists_by_id(pool: &PgPool, user_id: i32, id: i32) -> Result<bool, RepositoryError> {
    let query = dlc_query::exists_by_id(user_id, id);
    exists_id(pool, query).await
}

pub async fn exists_with_unique(
    pool: &PgPool,
    user_id: i32,
    dlc: &DLC,
) -> Result<bool, RepositoryError> {
    let query = dlc_query::exists_by_name(user_id, &dlc.name);
    exists_id(pool, query).await
}

pub async fn exists_with_unique_except_id(
    pool: &PgPool,
    user_id: i32,
    dlc: &DLC,
    excluded_id: i32,
) -> Result<bool, RepositoryError> {
    let query = dlc_query::exists_by_name_and_id_not(user_id, &dlc.name, excluded_id);
    exists_id(pool, query).await
}
