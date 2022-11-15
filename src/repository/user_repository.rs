use sqlx::PgPool;

use crate::query::user_query;
use crate::{entities::User, errors::RepositoryError};

use super::base::{execute_return_id, exists_id, fetch_optional};

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

pub async fn create(pool: &PgPool, user: &User, password: &str) -> Result<i32, RepositoryError> {
    let query = user_query::insert(user, password);
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

pub async fn exists_with_unique(pool: &PgPool, user: &User) -> Result<bool, RepositoryError> {
    let query = user_query::exists_by_username(&user.username);
    exists_id(pool, query).await
}
