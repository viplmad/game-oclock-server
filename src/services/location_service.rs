use crate::entities::{Location, LocationSearch};
use crate::errors::ApiErrors;
use crate::models::{LocationDTO, LocationPageResult, NewLocationDTO, SearchDTO};
use crate::repository::LocationRepository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
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
    pub async fn get_location(
        &self,
        user_id: &str,
        location_id: &str,
    ) -> Result<LocationDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, location_id).await;
        handle_get_result(find_result)
    }

    pub async fn search_locations(
        &self,
        user_id: &str,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<LocationPageResult, ApiErrors> {
        let search = handle_query_mapping::<LocationDTO, LocationSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn create_location(
        &self,
        user_id: &str,
        location: NewLocationDTO,
    ) -> Result<LocationDTO, ApiErrors> {
        create_merged(
            location,
            async move |created_location_id| self.get_location(user_id, &created_location_id).await,
            async move |mut location_to_create: Location| {
                let exists_result = self
                    .repository
                    .exists_by_name(user_id, &location_to_create.name)
                    .await;
                handle_already_exists_result::<LocationDTO>(exists_result)?;

                location_to_create.user_id = crate::uuid_utils::parse_uuid(user_id);
                location_to_create.id = crate::uuid_utils::new_model_uuid_real();
                location_to_create.added_datetime = crate::date_utils::now();
                location_to_create.updated_datetime = crate::date_utils::now();
                let create_result = self.repository.create(&location_to_create).await;
                handle_action_result::<LocationDTO>(create_result)?;

                Ok(location_to_create.id.to_string()) //TODO
            },
        )
        .await
    }

    pub async fn update_location(
        &self,
        user_id: &str,
        location_id: &str,
        location: NewLocationDTO,
    ) -> Result<(), ApiErrors> {
        update_merged(
            location,
            async move || self.get_location(user_id, location_id).await,
            async move |mut location_to_update: Location| {
                let exists_result = self
                    .repository
                    .exists_by_name_except_id(user_id, &location_to_update.name, location_id)
                    .await;
                handle_already_exists_result::<LocationDTO>(exists_result)?;

                location_to_update.user_id = crate::uuid_utils::parse_uuid(user_id);
                location_to_update.id = crate::uuid_utils::parse_uuid(location_id);
                location_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update(&location_to_update).await;
                handle_update_result::<LocationDTO>(update_result)
            },
        )
        .await
    }

    pub async fn delete_location(&self, user_id: &str, location_id: &str) -> Result<(), ApiErrors> {
        let delete_result = self.repository.delete_by_id(user_id, location_id).await;
        handle_action_result::<LocationDTO>(delete_result)
    }

    pub async fn exists_location(&self, user_id: &str, location_id: &str) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, location_id).await;
        handle_not_found_result::<LocationDTO>(exists_result)
    }
}
