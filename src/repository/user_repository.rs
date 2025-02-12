use sqlx::PgPool;

use super::query::user_query;
use crate::entities::{PageResult, User, UserSearch};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{execute, exists_id, fetch_all_search, fetch_optional};

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
    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError> {
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

    pub async fn create(&self, password: &str, user: &User) -> Result<String, RepositoryError> {
        let id = crate::uuid_utils::new_model_uuid();

        let query = user_query::insert(&id, password, user);
        execute(&self.pool, query).await.map(|_| id)
    }

    pub async fn update_by_id(&self, id: &str, user: &User) -> Result<(), RepositoryError> {
        let query = user_query::update_by_id(id, user);
        execute(&self.pool, query).await
    }

    pub async fn update_password(&self, id: &str, password: &str) -> Result<(), RepositoryError> {
        let query = user_query::update_password_by_id(id, password);
        execute(&self.pool, query).await
    }

    pub async fn update_admin(&self, id: &str, admin: bool) -> Result<(), RepositoryError> {
        let query = user_query::update_admin_by_id(id, admin);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<(), RepositoryError> {
        let query = user_query::delete_by_id(id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, id: &str) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_id(id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_unique(&self, user: &User) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_username(&user.username);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_unique_except_id(
        &self,
        user: &User,
        excluded_id: &str,
    ) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_username_and_id_not(&user.username, excluded_id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_admin_except_id(
        &self,
        excluded_id: &str,
    ) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_admin_and_id_not(excluded_id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_by_id_and_admin(&self, id: &str) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_admin_and_id(id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_admin(&self) -> Result<bool, RepositoryError> {
        let query = user_query::exists_by_admin();
        exists_id(&self.pool, query).await
    }
}
