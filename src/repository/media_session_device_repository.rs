use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_session_device_query;
use crate::entities::{
    AggregateResult, Device, DeviceAggregateSearch, DeviceListSearch, MediaAggregateSearch,
    MediaListSearch, MediaWithState, PageResult,
};
use crate::errors::SearchErrors;

use super::helpers::{aggregate_all_search, fetch_all_search};

#[derive(Clone)]
pub struct MediaSessionDeviceRepository {
    pool: PgPool,
}

impl MediaSessionDeviceRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MediaSessionDeviceRepository {
    pub async fn search_all_medias_with_session_device(
        &self,
        user_id: &Uuid,
        device_id: &Uuid,
        search: MediaListSearch,
    ) -> Result<PageResult<MediaWithState>, SearchErrors> {
        let query = media_session_device_query::select_all_medias_by_device_id_order_by_date(
            user_id, device_id, search,
        )?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all_medias_with_session_device(
        &self,
        user_id: &Uuid,
        device_id: &Uuid,
        search: MediaAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = media_session_device_query::aggregate_all_medias_by_device_id_order_by_date(
            user_id, device_id, search,
        )?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn search_all_devices_with_session_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: DeviceListSearch,
    ) -> Result<PageResult<Device>, SearchErrors> {
        let query = media_session_device_query::select_all_devices_by_media_id_order_by_date(
            user_id, media_id, search,
        )?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all_devices_with_session_media(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: DeviceAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = media_session_device_query::aggregate_all_devices_by_media_id_order_by_date(
            user_id, media_id, search,
        )?;
        aggregate_all_search(&self.pool, query).await
    }
}
