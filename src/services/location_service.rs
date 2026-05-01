use uuid::Uuid;

use crate::entities::{Location, LocationAggregateSearch, LocationListSearch};
use crate::errors::ApiErrors;
use crate::models::{
    AggregateResultDTO, AggregateSearchDTO, ListSearchDTO, LocationDTO, LocationPageResult,
    NewLocationDTO,
};
use crate::repository::LocationRepository;

use super::helpers::{
    create_merged, handle_action_result, handle_aggregate_search_mapping,
    handle_already_exists_result, handle_get_aggregate_result, handle_get_list_paged_result,
    handle_get_result, handle_list_search_mapping, handle_not_found_result, handle_update_result,
    update_merged,
};

#[derive(Clone)]
pub struct LocationService {
    repository: LocationRepository,
}

impl LocationService {
    pub fn with(repository: LocationRepository) -> Self {
        Self { repository }
    }
}

impl LocationService {
    pub async fn get_location(&self, user_id: &Uuid, id: &Uuid) -> Result<LocationDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, id).await;
        handle_get_result(find_result)
    }

    pub async fn search_locations(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<LocationPageResult, ApiErrors> {
        let search =
            handle_list_search_mapping::<LocationDTO, LocationListSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn aggregate_locations(
        &self,
        user_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        let search = handle_aggregate_search_mapping::<LocationDTO, LocationAggregateSearch>(
            search,
            quicksearch,
        )?;
        let aggregate_result = self.repository.aggregate_all(user_id, search).await;
        handle_get_aggregate_result::<LocationDTO>(aggregate_result)
    }

    pub async fn create_location(
        &self,
        user_id: &Uuid,
        location: NewLocationDTO,
    ) -> Result<LocationDTO, ApiErrors> {
        let new_id = crate::uuid_utils::new_model_uuid();
        create_merged(
            location,
            async move || self.get_location(user_id, &new_id).await,
            async move |mut location_to_create: Location| {
                let exists_result = self
                    .repository
                    .exists_by_name(user_id, &location_to_create.name)
                    .await;
                handle_already_exists_result::<LocationDTO>(exists_result)?;

                location_to_create.user_id = user_id.clone();
                location_to_create.id = new_id.clone();
                location_to_create.added_datetime = crate::date_utils::now();
                location_to_create.updated_datetime = crate::date_utils::now();
                let create_result = self.repository.create(&location_to_create).await;
                handle_action_result::<LocationDTO>(create_result)
            },
        )
        .await
    }

    pub async fn update_location(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        location: NewLocationDTO,
    ) -> Result<(), ApiErrors> {
        update_merged(
            location,
            async move || self.get_location(user_id, id).await,
            async move |mut location_to_update: Location| {
                let exists_result = self
                    .repository
                    .exists_by_name_except_id(user_id, &location_to_update.name, id)
                    .await;
                handle_already_exists_result::<LocationDTO>(exists_result)?;

                location_to_update.user_id = user_id.clone();
                location_to_update.id = id.clone();
                location_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update(&location_to_update).await;
                handle_update_result::<LocationDTO>(update_result)
            },
        )
        .await
    }

    pub async fn delete_location(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let delete_result = self.repository.delete_by_id(user_id, id).await;
        handle_action_result::<LocationDTO>(delete_result)
    }

    pub async fn exists_location(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, id).await;
        handle_not_found_result::<LocationDTO>(exists_result)
    }
}
