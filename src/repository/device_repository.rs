use sqlx::PgPool;

use super::query::device_query;
use crate::entities::{Device, DeviceSearch, PageResult};
use crate::errors::{RepositoryError, SearchErrors};

use super::base::{execute, exists_id, fetch_all_search, fetch_optional};

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
        user_id: &str,
        id: &str,
    ) -> Result<Option<Device>, RepositoryError> {
        let query = device_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &str,
        search: DeviceSearch,
    ) -> Result<PageResult<Device>, SearchErrors> {
        let search_query = device_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn create(&self, user_id: &str, device: &Device) -> Result<String, RepositoryError> {
        let id = crate::uuid_utils::new_model_uuid();

        let query = device_query::insert(user_id, &id, device);
        execute(&self.pool, query).await.map(|_| id)
    }

    pub async fn update_by_id(
        &self,
        user_id: &str,
        id: &str,
        device: &Device,
    ) -> Result<(), RepositoryError> {
        let query = device_query::update_by_id(user_id, id, device);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, user_id: &str, id: &str) -> Result<(), RepositoryError> {
        let query = device_query::delete_by_id(user_id, id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(&self, user_id: &str, id: &str) -> Result<bool, RepositoryError> {
        let query = device_query::exists_by_id(user_id, id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_unique(
        &self,
        user_id: &str,
        device: &Device,
    ) -> Result<bool, RepositoryError> {
        let query = device_query::exists_by_name(user_id, &device.name);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_unique_except_id(
        &self,
        user_id: &str,
        device: &Device,
        excluded_id: &str,
    ) -> Result<bool, RepositoryError> {
        let query = device_query::exists_by_name_and_id_not(user_id, &device.name, excluded_id);
        exists_id(&self.pool, query).await
    }
}
