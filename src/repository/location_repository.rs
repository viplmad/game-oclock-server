use sqlx::PgPool;

use super::query::location_query;
use crate::entities::{Location, LocationSearch, PageResult};
use crate::errors::{RepositoryError, SearchErrors};

use super::base::{execute, exists_id, fetch_all_search, fetch_optional};

#[derive(Clone)]
pub struct LocationRepository {
    pool: PgPool,
}

impl LocationRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl LocationRepository {
    pub async fn find_by_id(
        &self,
        user_id: &str,
        id: &str,
    ) -> Result<Option<Location>, RepositoryError> {
        let query = location_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &str,
        search: LocationSearch,
    ) -> Result<PageResult<Location>, SearchErrors> {
        let search_query = location_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn create(&self, location: &Location) -> Result<(), RepositoryError> {
        let query = location_query::insert(location);
        execute(&self.pool, query).await
    }

    pub async fn update(&self, location: &Location) -> Result<(), RepositoryError> {
        let query = location_query::update_by_id(location);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, user_id: &str, id: &str) -> Result<(), RepositoryError> {
        let query = location_query::delete_by_id(user_id, id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, user_id: &str, id: &str) -> Result<bool, RepositoryError> {
        let query = location_query::exists_by_id(user_id, id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_by_name(&self, user_id: &str, name: &str) -> Result<bool, RepositoryError> {
        let query = location_query::exists_by_name(user_id, name);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_by_name_except_id(
        &self,
        user_id: &str,
        name: &str,
        excluded_id: &str,
    ) -> Result<bool, RepositoryError> {
        let query = location_query::exists_by_name_and_id_not(user_id, name, excluded_id);
        exists_id(&self.pool, query).await
    }
}
