use sqlx::PgPool;
use uuid::Uuid;

use super::query::user_query;
use crate::entities::{PageResult, User, UserSearch};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{execute, exists_some, fetch_all_search, fetch_optional};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository {
    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, RepositoryError> {
        let query = user_query::select_by_id(id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_first_by_username(
        &self,
        username: &str,
    ) -> Result<Option<User>, RepositoryError> {
        let query = user_query::select_by_username(username);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all(&self, search: UserSearch) -> Result<PageResult<User>, SearchErrors> {
        let search_query = user_query::select_all_with_search(search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn create(&self, user: &User) -> Result<(), RepositoryError> {
        let query = user_query::insert(user);
        execute(&self.pool, query).await
    }

    pub async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        let query = user_query::update_by_id(user);
        execute(&self.pool, query).await
    }

    pub async fn update_password_by_id(
        &self,
        id: &Uuid,
        password: &str,
    ) -> Result<(), RepositoryError> {
        let query = user_query::update_password_by_id(id, password);
        execute(&self.pool, query).await
    }

    pub async fn update_admin_by_id(&self, id: &Uuid, admin: bool) -> Result<(), RepositoryError> {
        let query = user_query::update_admin_by_id(id, admin);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, id: &Uuid) -> Result<(), RepositoryError> {
        let query = user_query::delete_by_id(id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_id(id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_username(&self, username: &str) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_username(username);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_username_except_id(
        &self,
        username: &str,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_username_and_id_not(username, excluded_id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_with_admin_except_id(
        &self,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_admin_and_id_not(excluded_id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_id_and_admin(&self, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_admin_and_id(id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_with_admin(&self) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_admin();
        exists_some(&self.pool, query).await
    }
}
