use uuid::Uuid;

use crate::entities::{
    DeviceAggregateSearch, DeviceListSearch, MediaAggregateSearch, MediaListSearch,
};
use crate::errors::ApiErrors;
use crate::models::{
    AggregateResultDTO, AggregateSearchDTO, DeviceDTO, DevicePageResult, ListSearchDTO, MediaDTO,
    MediaPageResult,
};
use crate::repository::MediaSessionDeviceRepository;

use super::helpers::{
    handle_aggregate_search_mapping, handle_get_aggregate_result, handle_get_list_paged_result,
    handle_list_search_mapping,
};
use super::{DeviceService, MediaService};

#[derive(Clone)]
pub struct MediaSessionDeviceService {
    repository: MediaSessionDeviceRepository,
    media_service: MediaService,
    device_service: DeviceService,
}

impl MediaSessionDeviceService {
    pub fn with(
        repository: MediaSessionDeviceRepository,
        media_service: MediaService,
        device_service: DeviceService,
    ) -> Self {
        Self {
            repository,
            media_service,
            device_service,
        }
    }
}

impl MediaSessionDeviceService {
    pub async fn search_device_session_medias(
        &self,
        user_id: &Uuid,
        device_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<MediaPageResult, ApiErrors> {
        self.device_service
            .exists_device(user_id, device_id)
            .await?;

        let search = handle_list_search_mapping::<MediaDTO, MediaListSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_all_medias_with_session_device(user_id, device_id, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn aggregate_device_session_medias(
        &self,
        user_id: &Uuid,
        device_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        self.device_service
            .exists_device(user_id, device_id)
            .await?;

        let search =
            handle_aggregate_search_mapping::<MediaDTO, MediaAggregateSearch>(search, quicksearch)?;
        let aggregate_result = self
            .repository
            .aggregate_all_medias_with_session_device(user_id, device_id, search)
            .await;
        handle_get_aggregate_result::<MediaDTO>(aggregate_result)
    }

    pub async fn search_media_session_devices(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<DevicePageResult, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search =
            handle_list_search_mapping::<DeviceDTO, DeviceListSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_all_devices_with_session_media(user_id, media_id, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn aggregate_media_session_devices(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search = handle_aggregate_search_mapping::<DeviceDTO, DeviceAggregateSearch>(
            search,
            quicksearch,
        )?;
        let aggregate_result = self
            .repository
            .aggregate_all_devices_with_session_media(user_id, media_id, search)
            .await;
        handle_get_aggregate_result::<DeviceDTO>(aggregate_result)
    }
}
