use sqlx::PgPool;
use uuid::Uuid;

use super::query::device_query;
use crate::entities::{
    AggregateResult, Device, DeviceAggregateSearch, DeviceListSearch, PageResult,
};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{
    aggregate_all_search, execute, exists_some, fetch_all_search, fetch_optional,
};

#[derive(Clone)]
pub struct DeviceRepository {
    pool: PgPool,
}

impl DeviceRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl DeviceRepository {
    pub async fn find_by_id(
        &self,
        user_id: &Uuid,
        id: &Uuid,
    ) -> Result<Option<Device>, RepositoryError> {
        let query = device_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &Uuid,
        search: DeviceListSearch,
    ) -> Result<PageResult<Device>, SearchErrors> {
        let query = device_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all(
        &self,
        user_id: &Uuid,
        search: DeviceAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = device_query::aggregate_all_with_search(user_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn create(&self, device: &Device) -> Result<(), RepositoryError> {
        let query = device_query::insert(device);
        execute(&self.pool, query).await
    }

    pub async fn update(&self, device: &Device) -> Result<(), RepositoryError> {
        let query = device_query::update_by_id(device);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<(), RepositoryError> {
        let query = device_query::delete_by_id(user_id, id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, user_id: &Uuid, id: &Uuid) -> Result<bool, RepositoryError> {
        let query = device_query::exists_by_id(user_id, id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_name(
        &self,
        user_id: &Uuid,
        name: &str,
    ) -> Result<bool, RepositoryError> {
        let query = device_query::exists_by_name(user_id, name);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_name_except_id(
        &self,
        user_id: &Uuid,
        name: &str,
        excluded_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = device_query::exists_by_name_and_id_not(user_id, name, excluded_id);
        exists_some(&self.pool, query).await
    }
}
