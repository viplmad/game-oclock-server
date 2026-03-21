use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entities::{LocationSearch, MediaAvailable, MediaSearch};
use crate::errors::ApiErrors;
use crate::models::{
    LocationAvailablePageResult, LocationDTO, MediaAvailableDTO, MediaAvailablePageResult,
    MediaDTO, SearchDTO,
};
use crate::repository::MediaAvailableRepository;

use super::helpers::{
    handle_action_result, handle_already_exists_result, handle_get_count_result,
    handle_get_list_paged_result, handle_not_found_result, handle_query_mapping,
};
use super::{LocationService, MediaService};

#[derive(Clone)]
pub struct MediaAvailableService {
    repository: MediaAvailableRepository,
    media_service: MediaService,
    location_service: LocationService,
}

impl MediaAvailableService {
    pub fn with(
        repository: MediaAvailableRepository,
        media_service: MediaService,
        location_service: LocationService,
    ) -> Self {
        Self {
            repository,
            media_service,
            location_service,
        }
    }
}

impl MediaAvailableService {
    pub async fn search_location_medias(
        &self,
        user_id: &Uuid,
        location_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<MediaAvailablePageResult, ApiErrors> {
        self.location_service
            .exists_location(user_id, location_id)
            .await?;

        let search = handle_query_mapping::<MediaDTO, MediaSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_all_medias_with_location(user_id, location_id, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn count_location_medias(
        &self,
        user_id: &Uuid,
        location_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<u64, ApiErrors> {
        self.location_service
            .exists_location(user_id, location_id)
            .await?;

        let search = handle_query_mapping::<MediaDTO, MediaSearch>(search, quicksearch)?;
        let count_result = self
            .repository
            .count_all_medias_with_location(user_id, location_id, search)
            .await;
        handle_get_count_result::<MediaDTO>(count_result)
    }

    pub async fn search_media_locations(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<LocationAvailablePageResult, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search = handle_query_mapping::<LocationDTO, LocationSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_all_locations_with_media(user_id, media_id, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn count_media_locations(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<u64, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search = handle_query_mapping::<LocationDTO, LocationSearch>(search, quicksearch)?;
        let count_result = self
            .repository
            .count_all_locations_with_media(user_id, media_id, search)
            .await;
        handle_get_count_result::<LocationDTO>(count_result)
    }

    pub async fn create_media_available(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        location_id: &Uuid,
        available_date: DateTime<Utc>,
    ) -> Result<(), ApiErrors> {
        self.media_service.exists_media(media_id).await?;
        self.location_service
            .exists_location(user_id, location_id)
            .await?;

        let exists_result = self
            .repository
            .exists_by_id(user_id, media_id, location_id)
            .await;
        handle_already_exists_result::<MediaAvailableDTO>(exists_result)?;

        let create_result = self
            .repository
            .create(&MediaAvailable {
                user_id: user_id.clone(),
                media_id: media_id.clone(),
                location_id: location_id.clone(),
                date: available_date,
                added_datetime: crate::date_utils::now(),
                updated_datetime: crate::date_utils::now(),
            })
            .await;
        handle_action_result::<MediaAvailableDTO>(create_result)
    }

    pub async fn delete_media_available(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        location_id: &Uuid,
    ) -> Result<(), ApiErrors> {
        self.exists_media_available(user_id, media_id, location_id)
            .await?;

        let delete_result = self
            .repository
            .delete_by_id(user_id, media_id, location_id)
            .await;
        handle_action_result::<MediaAvailableDTO>(delete_result)
    }

    pub async fn exists_media_available(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        location_id: &Uuid,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id(user_id, media_id, location_id)
            .await;
        handle_not_found_result::<MediaAvailableDTO>(exists_result)
    }
}
