use uuid::Uuid;

use crate::entities::{Device, DeviceListSearch};
use crate::errors::ApiErrors;
use crate::models::{DeviceDTO, DevicePageResult, ListSearchDTO, NewDeviceDTO};
use crate::repository::DeviceRepository;

use super::helpers::{
    create_merged, handle_action_result, handle_already_exists_result, handle_get_aggregate_result,
    handle_get_list_paged_result, handle_get_result, handle_list_search_mapping,
    handle_not_found_result, handle_update_result, update_merged,
};

#[derive(Clone)]
pub struct DeviceService {
    repository: DeviceRepository,
}

impl DeviceService {
    pub fn with(repository: DeviceRepository) -> Self {
        Self { repository }
    }
}

impl DeviceService {
    pub async fn get_device(&self, user_id: &Uuid, id: &Uuid) -> Result<DeviceDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, id).await;
        handle_get_result(find_result)
    }

    pub async fn search_devices(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<DevicePageResult, ApiErrors> {
        let search =
            handle_list_search_mapping::<DeviceDTO, DeviceListSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn count_devices(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<u64, ApiErrors> {
        let search =
            handle_list_search_mapping::<DeviceDTO, DeviceListSearch>(search, quicksearch)?;
        let count_result = self.repository.count_all(user_id, search).await;
        handle_get_aggregate_result::<DeviceDTO>(count_result)
    }

    pub async fn create_device(
        &self,
        user_id: &Uuid,
        device: NewDeviceDTO,
    ) -> Result<DeviceDTO, ApiErrors> {
        let new_id = crate::uuid_utils::new_model_uuid();
        create_merged(
            device,
            async move || self.get_device(user_id, &new_id).await,
            async move |mut device_to_create: Device| {
                let exists_result = self
                    .repository
                    .exists_by_name(user_id, &device_to_create.name)
                    .await;
                handle_already_exists_result::<DeviceDTO>(exists_result)?;

                device_to_create.user_id = user_id.clone();
                device_to_create.id = new_id.clone();
                device_to_create.added_datetime = crate::date_utils::now();
                device_to_create.updated_datetime = crate::date_utils::now();
                let create_result = self.repository.create(&device_to_create).await;
                handle_action_result::<DeviceDTO>(create_result)
            },
        )
        .await
    }

    pub async fn update_device(
        &self,
        user_id: &Uuid,
        id: &Uuid,
        device: NewDeviceDTO,
    ) -> Result<(), ApiErrors> {
        update_merged(
            device,
            async move || self.get_device(user_id, id).await,
            async move |mut device_to_update: Device| {
                let exists_result = self
                    .repository
                    .exists_by_name_except_id(user_id, &device_to_update.name, id)
                    .await;
                handle_already_exists_result::<DeviceDTO>(exists_result)?;

                device_to_update.user_id = user_id.clone();
                device_to_update.id = id.clone();
                device_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update(&device_to_update).await;
                handle_update_result::<DeviceDTO>(update_result)
            },
        )
        .await
    }

    pub async fn delete_device(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let delete_result = self.repository.delete_by_id(user_id, id).await;
        handle_action_result::<DeviceDTO>(delete_result)
    }

    pub async fn exists_device(&self, user_id: &Uuid, id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, id).await;
        handle_not_found_result::<DeviceDTO>(exists_result)
    }
}
