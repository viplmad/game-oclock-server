use sqlx::PgPool;

use crate::entities::{PageResult, User, UserSearch};
use crate::errors::{RepositoryError, SearchErrors};
use crate::query::user_query;

use super::base::{execute, execute_return_id, exists_id, fetch_all_search, fetch_optional};

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, RepositoryError> {
    let query = user_query::select_by_id(id);
    fetch_optional(pool, query).await
}

pub async fn find_first_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, RepositoryError> {
    let query = user_query::select_by_username(username);
    fetch_optional(pool, query).await
}

pub async fn search_all(
    pool: &PgPool,
    search: UserSearch,
) -> Result<PageResult<User>, SearchErrors> {
    let search_query = user_query::select_all_with_search(search)?;
    fetch_all_search(pool, search_query).await
}

pub async fn create(pool: &PgPool, user: &User, password: &str) -> Result<i32, RepositoryError> {
    let query = user_query::insert(user, password);
    execute_return_id(pool, query).await
}

pub async fn update_by_id(pool: &PgPool, id: i32, user: &User) -> Result<i32, RepositoryError> {
    let query = user_query::update_by_id(id, user);
    execute_return_id(pool, query).await
}

pub async fn update_password(
    pool: &PgPool,
    id: i32,
    password: &str,
) -> Result<i32, RepositoryError> {
    let query = user_query::update_password_by_id(id, password);
    execute_return_id(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<(), RepositoryError> {
    let query = user_query::delete_by_id(id);
    execute(pool, query).await
}

pub async fn exists_by_id(pool: &PgPool, id: i32) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_id(id);
    exists_id(pool, query).await
}

pub async fn exists_with_unique(pool: &PgPool, user: &User) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_username(&user.username);
    exists_id(pool, query).await
}

pub async fn exists_with_unique_except_id(
    pool: &PgPool,
    user: &User,
    excluded_id: i32,
) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_username_and_id_not(&user.username, excluded_id);
    exists_id(pool, query).await
}
