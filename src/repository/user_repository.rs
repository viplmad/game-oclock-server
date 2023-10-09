use sqlx::PgPool;

use crate::entities::{PageResult, User, UserSearch};
use crate::errors::{RepositoryError, SearchErrors};
use crate::query::user_query;

use super::base::{execute, exists_id, fetch_all_search, fetch_optional};

pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<User>, RepositoryError> {
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

pub async fn create(pool: &PgPool, password: &str, user: &User) -> Result<String, RepositoryError> {
    let id = crate::uuid_utils::new_model_uuid();

    let query = user_query::insert(&id, password, user);
    execute(pool, query).await.map(|_| id)
}

pub async fn update_by_id(pool: &PgPool, id: &str, user: &User) -> Result<(), RepositoryError> {
    let query = user_query::update_by_id(id, user);
    execute(pool, query).await
}

pub async fn update_password(
    pool: &PgPool,
    id: &str,
    password: &str,
) -> Result<(), RepositoryError> {
    let query = user_query::update_password_by_id(id, password);
    execute(pool, query).await
}

pub async fn update_admin(pool: &PgPool, id: &str, admin: bool) -> Result<(), RepositoryError> {
    let query = user_query::update_admin_by_id(id, admin);
    execute(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, id: &str) -> Result<(), RepositoryError> {
    let query = user_query::delete_by_id(id);
    execute(pool, query).await
}

pub async fn exists_by_id(pool: &PgPool, id: &str) -> Result<bool, RepositoryError> {
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
    excluded_id: &str,
) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_username_and_id_not(&user.username, excluded_id);
    exists_id(pool, query).await
}

pub async fn exists_with_admin_except_id(
    pool: &PgPool,
    excluded_id: &str,
) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_admin_and_id_not(excluded_id);
    exists_id(pool, query).await
}

pub async fn exists_by_id_and_admin(pool: &PgPool, id: &str) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_admin_and_id(id);
    exists_id(pool, query).await
}

pub async fn exists_with_admin(pool: &PgPool) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_admin();
    exists_id(pool, query).await
}
