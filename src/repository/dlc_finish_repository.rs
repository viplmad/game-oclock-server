use chrono::NaiveDate;
use sqlx::PgPool;

use crate::errors::RepositoryError;
use crate::query::dlc_finish_query;

use super::base::{execute, exists_id, fetch_all_single};

pub async fn find_all_by_dlc_id(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
) -> Result<Vec<NaiveDate>, RepositoryError> {
    let query = dlc_finish_query::select_all_by_user_id_and_dlc_id(user_id, dlc_id);
    fetch_all_single(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = dlc_finish_query::insert(user_id, dlc_id, date);
    execute(pool, query).await
}

pub async fn delete(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = dlc_finish_query::delete_by_id(user_id, dlc_id, date);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    date: NaiveDate,
) -> Result<bool, RepositoryError> {
    let query = dlc_finish_query::exists_by_id(user_id, dlc_id, date);
    exists_id(pool, query).await
}