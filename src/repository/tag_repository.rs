use sqlx::PgPool;

use crate::entities::{PageResult, Tag, TagSearch};
use crate::errors::{RepositoryError, SearchErrors};
use crate::query::tag_query;

use super::base::{execute, execute_return_id, exists_id, fetch_all_search, fetch_optional};

pub async fn find_by_id(
    pool: &PgPool,
    user_id: i32,
    id: i32,
) -> Result<Option<Tag>, RepositoryError> {
    let query = tag_query::select_by_id(user_id, id);
    fetch_optional(pool, query).await
}

pub async fn search_all(
    pool: &PgPool,
    user_id: i32,
    search: TagSearch,
) -> Result<PageResult<Tag>, SearchErrors> {
    let search_query = tag_query::select_all_with_query(user_id, search)?;
    fetch_all_search(pool, search_query).await
}

pub async fn create(pool: &PgPool, user_id: i32, tag: &Tag) -> Result<i32, RepositoryError> {
    let query = tag_query::insert(user_id, tag);
    execute_return_id(pool, query).await
}

pub async fn update_by_id(
    pool: &PgPool,
    user_id: i32,
    id: i32,
    tag: &Tag,
) -> Result<i32, RepositoryError> {
    let query = tag_query::update_by_id(user_id, id, tag);
    execute_return_id(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, user_id: i32, id: i32) -> Result<(), RepositoryError> {
    let query = tag_query::delete_by_id(user_id, id);
    execute(pool, query).await
}

pub async fn exists_by_id(pool: &PgPool, user_id: i32, id: i32) -> Result<bool, RepositoryError> {
    let query = tag_query::exists_by_id(user_id, id);
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
    excluded_id: i32,
) -> Result<bool, RepositoryError> {
    let query = tag_query::exists_by_name_and_id_not(user_id, &tag.name, excluded_id);
    exists_id(pool, query).await
}
