use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_available_query;
use crate::entities::{
    LocationListSearch, LocationWithAvailable, MediaAvailable, MediaListSearch,
    MediaWithStateWithAvailable, PageResult,
};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{aggregate_all_search, execute, exists_some, fetch_all_search};

#[derive(Clone)]
pub struct MediaAvailableRepository {
    pool: PgPool,
}

impl MediaAvailableRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MediaAvailableRepository {
    pub async fn search_all_medias_with_location(
        &self,
        user_id: &Uuid,
        location_id: &Uuid,
        search: MediaListSearch,
    ) -> Result<PageResult<MediaWithStateWithAvailable>, SearchErrors> {
        let search_query = media_available_query::select_all_medias_by_location_id_order_by_date(
            user_id,
            location_id,
            search,
        )?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all_medias_with_location(
        &self,
        user_id: &Uuid,
        location_id: &Uuid,
        search: MediaListSearch,
    ) -> Result<u64, SearchErrors> {
        let count_query = media_available_query::count_all_medias_by_location_id_order_by_date(
            user_id,
            location_id,
            search,
        )?;
        aggregate_all_search(&self.pool, count_query).await
    }

    pub async fn search_all_locations_with_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: LocationListSearch,
    ) -> Result<PageResult<LocationWithAvailable>, SearchErrors> {
        let search_query = media_available_query::select_all_locations_by_media_id_order_by_date(
            user_id, media_id, search,
        )?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all_locations_with_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: LocationListSearch,
    ) -> Result<u64, SearchErrors> {
        let count_query = media_available_query::count_all_locations_by_media_id_order_by_date(
            user_id, media_id, search,
        )?;
        aggregate_all_search(&self.pool, count_query).await
    }

    pub async fn create(&self, media_available: &MediaAvailable) -> Result<(), RepositoryError> {
        let query = media_available_query::insert(media_available);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        location_id: &Uuid,
    ) -> Result<(), RepositoryError> {
        let query = media_available_query::delete_by_id(user_id, media_id, location_id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        location_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = media_available_query::exists_by_id(user_id, media_id, location_id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_locations_with_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = media_available_query::exists_locations_by_media_id(user_id, media_id);
        exists_some(&self.pool, query).await
    }
}
