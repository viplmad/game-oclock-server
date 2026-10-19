use sqlx::PgPool;
use uuid::Uuid;

use super::query::location_query;
use crate::entities::{
    AggregateResult, Location, LocationAggregateSearch, LocationListSearch, PageResult,
};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{
    aggregate_all_search, execute, exists_some, fetch_all_search, fetch_optional,
};

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
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<Location>, RepositoryError> {
        let query = location_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &Uuid,
        search: LocationListSearch,
    ) -> Result<PageResult<Location>, SearchErrors> {
        let query = location_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all(
        &self,
        user_id: &Uuid,
        search: LocationAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = location_query::aggregate_all_with_search(user_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn create(&self, location: &Location) -> Result<(), RepositoryError> {
        let query = location_query::insert(location);
        execute(&self.pool, query).await
    }

    pub async fn update(&self, location: &Location) -> Result<(), RepositoryError> {
        let query = location_query::update_by_id(location);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<(), RepositoryError> {
        let query = location_query::delete_by_id(user_id, id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = location_query::exists_by_id(user_id, id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_name(
        &self,
        user_id: &Uuid,
        name: &str,
    ) -> Result<bool, RepositoryError> {
        let query = location_query::exists_by_name(user_id, name);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_name_except_id(
        &self,
        user_id: &Uuid,
        name: &str,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = location_query::exists_by_name_and_id_not(user_id, name, excluded_id);
        exists_some(&self.pool, query).await
    }
}
