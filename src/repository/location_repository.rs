use sqlx::PgPool;

use crate::entities::{Location, LocationSearch, PageResult};
use crate::errors::{RepositoryError, SearchErrors};
use crate::query::location_query;

use super::base::{execute, exists_id, fetch_all_search, fetch_optional};

pub async fn find_by_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
) -> Result<Option<Location>, RepositoryError> {
    let query = location_query::select_by_id(user_id, id);
    fetch_optional(pool, query).await
}

pub async fn search_all(
    pool: &PgPool,
    user_id: &str,
    search: LocationSearch,
) -> Result<PageResult<Location>, SearchErrors> {
    let search_query = location_query::select_all_with_search(user_id, search)?;
    fetch_all_search(pool, search_query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: &str,
    location: &Location,
) -> Result<String, RepositoryError> {
    let id = crate::uuid_utils::new_model_uuid();

    let query = location_query::insert(user_id, &id, location);
    execute(pool, query).await.map(|_| id)
}

pub async fn update_by_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
    location: &Location,
) -> Result<(), RepositoryError> {
    let query = location_query::update_by_id(user_id, id, location);
    execute(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, user_id: &str, id: &str) -> Result<(), RepositoryError> {
    let query = location_query::delete_by_id(user_id, id);
    execute(pool, query).await
}

pub async fn exists_by_id(pool: &PgPool, user_id: &str, id: &str) -> Result<bool, RepositoryError> {
    let query = location_query::exists_by_id(user_id, id);
    exists_id(pool, query).await
}

pub async fn exists_with_unique(
    pool: &PgPool,
    user_id: &str,
    location: &Location,
) -> Result<bool, RepositoryError> {
    let query = location_query::exists_by_name(user_id, &location.name);
    exists_id(pool, query).await
}

pub async fn exists_with_unique_except_id(
    pool: &PgPool,
    user_id: &str,
    location: &Location,
    excluded_id: &str,
) -> Result<bool, RepositoryError> {
    let query = location_query::exists_by_name_and_id_not(user_id, &location.name, excluded_id);
    exists_id(pool, query).await
}
