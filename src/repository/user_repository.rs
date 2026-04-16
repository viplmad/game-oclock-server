use sqlx::PgPool;
use uuid::Uuid;

use super::query::user_query;
use crate::entities::{PageResult, User, UserListSearch};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{
    aggregate_all_search, execute, exists_some, fetch_all_search, fetch_optional,
};

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

    pub async fn search_all(
        &self,
        search: UserListSearch,
    ) -> Result<PageResult<User>, SearchErrors> {
        let search_query = user_query::select_all_with_search(search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all(&self, search: UserListSearch) -> Result<u64, SearchErrors> {
        let count_query = user_query::count_all_with_search(search)?;
        aggregate_all_search(&self.pool, count_query).await
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

    pub async fn update_admin_by_id(&self, id: &Uuid, role: &str) -> Result<(), RepositoryError> {
        let query = user_query::update_role_by_id(id, role);
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

    pub async fn exists_with_role_except_id(
        &self,
        excluded_id: &Uuid,
        role: &str,
    ) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_role_and_id_not(excluded_id, role);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_id_and_role(
        &self,
        id: &Uuid,
        role: &str,
    ) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_role_and_id(id, role);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_with_role(&self, role: &str) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_role(role);
        exists_some(&self.pool, query).await
    }
}
