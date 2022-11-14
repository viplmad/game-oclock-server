use sqlx::PgPool;

use crate::entities::Tag;
use crate::errors::RepositoryError;
use crate::query::tag_query;

use super::base::{execute, execute_return_id, exists_id, fetch_all, fetch_optional};

pub async fn find_by_id(
    pool: &PgPool,
    user_id: i32,
    tag_id: i32,
) -> Result<Option<Tag>, RepositoryError> {
    let query = tag_query::select_by_id(user_id, tag_id);
    fetch_optional(pool, query).await
}

pub async fn find_all(
    pool: &PgPool,
    user_id: i32,
    limit: u64,
) -> Result<Vec<Tag>, RepositoryError> {
    // TODO Replace limit with query/search
    let query = tag_query::select_all(user_id, limit);
    fetch_all(pool, query).await
}

pub async fn create(pool: &PgPool, user_id: i32, tag: &Tag) -> Result<i32, RepositoryError> {
    let query = tag_query::insert(user_id, tag);
    execute_return_id(pool, query).await
}

pub async fn update(
    pool: &PgPool,
    user_id: i32,
    tag_id: i32,
    tag: &Tag,
) -> Result<i32, RepositoryError> {
    let query = tag_query::update_by_id(user_id, tag_id, tag);
    execute_return_id(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, user_id: i32, tag_id: i32) -> Result<(), RepositoryError> {
    let query = tag_query::delete_by_id(user_id, tag_id);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: i32,
    tag_id: i32,
) -> Result<bool, RepositoryError> {
    let query = tag_query::exists_by_id(user_id, tag_id);
    exists_id(pool, query).await
}

pub async fn exists_with_unique(
    pool: &PgPool,
    user_id: i32,
    tag: &Tag,
) -> Result<bool, RepositoryError> {
    let query = tag_query::exists_by_name(user_id, &tag.name);
    exists_id(pool, query).await
}

pub async fn exists_with_unique_except_id(
    pool: &PgPool,
    user_id: i32,
    tag: &Tag,
    tag_id: i32,
) -> Result<bool, RepositoryError> {
    let query = tag_query::exists_by_name_and_id_not(user_id, &tag.name, tag_id);
    exists_id(pool, query).await
}
