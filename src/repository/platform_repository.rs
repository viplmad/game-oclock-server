use sqlx::PgPool;

use crate::entities::{PageResult, Platform, PlatformSearch};
use crate::errors::{RepositoryError, SearchErrors};
use crate::query::platform_query;

use super::base::{execute, execute_return_id, exists_id, fetch_all_search, fetch_optional};

pub async fn find_by_id(
    pool: &PgPool,
    user_id: i32,
    id: i32,
) -> Result<Option<Platform>, RepositoryError> {
    let query = platform_query::select_by_id(user_id, id);
    fetch_optional(pool, query).await
}

pub async fn search_all(
    pool: &PgPool,
    user_id: i32,
    search: PlatformSearch,
) -> Result<PageResult<Platform>, SearchErrors> {
    let search_query = platform_query::select_all_with_search(user_id, search)?;
    fetch_all_search(pool, search_query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    platform: &Platform,
) -> Result<i32, RepositoryError> {
    let query = platform_query::insert(user_id, platform);
    execute_return_id(pool, query).await
}

pub async fn update_by_id(
    pool: &PgPool,
    user_id: i32,
    id: i32,
    platform: &Platform,
) -> Result<i32, RepositoryError> {
    let query = platform_query::update_by_id(user_id, id, platform);
    execute_return_id(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, user_id: i32, id: i32) -> Result<(), RepositoryError> {
    let query = platform_query::delete_by_id(user_id, id);
    execute(pool, query).await
}

pub async fn exists_by_id(pool: &PgPool, user_id: i32, id: i32) -> Result<bool, RepositoryError> {
    let query = platform_query::exists_by_id(user_id, id);
    exists_id(pool, query).await
}

pub async fn exists_with_unique(
    pool: &PgPool,
    user_id: i32,
    platform: &Platform,
) -> Result<bool, RepositoryError> {
    let query = platform_query::exists_by_name(user_id, &platform.name);
    exists_id(pool, query).await
}

pub async fn exists_with_unique_except_id(
    pool: &PgPool,
    user_id: i32,
    platform: &Platform,
    excluded_id: i32,
) -> Result<bool, RepositoryError> {
    let query = platform_query::exists_by_name_and_id_not(user_id, &platform.name, excluded_id);
    exists_id(pool, query).await
}
