use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{DLCWithDate, PlatformAvailable};
use crate::errors::RepositoryError;
use crate::query::dlc_available_query;

use super::base::{execute, exists_id, fetch_all};

pub async fn find_all_dlcs_with_platform(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
) -> Result<Vec<DLCWithDate>, RepositoryError> {
    let query = dlc_available_query::select_all_dlcs_by_platform_id(user_id, platform_id);
    fetch_all(pool, query).await
}

pub async fn find_all_platforms_with_dlc(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
) -> Result<Vec<PlatformAvailable>, RepositoryError> {
    let query = dlc_available_query::select_all_platforms_by_dlc_id(user_id, dlc_id);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    platform_id: i32,
    added_date: NaiveDate,
) -> Result<(), RepositoryError> {
    let query = dlc_available_query::insert(user_id, dlc_id, platform_id, added_date);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    platform_id: i32,
) -> Result<(), RepositoryError> {
    let query = dlc_available_query::delete_by_id(user_id, dlc_id, platform_id);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    platform_id: i32,
) -> Result<bool, RepositoryError> {
    let query = dlc_available_query::exists_by_id(user_id, dlc_id, platform_id);
    exists_id(pool, query).await
}
