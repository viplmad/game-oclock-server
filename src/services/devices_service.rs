use crate::entities::DeviceSearch;
use crate::errors::ApiErrors;
use crate::models::{DeviceDTO, DevicePageResult, NewDeviceDTO, SearchDTO};
use crate::repository::DeviceRepository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

pub async fn get_device(
    repository: &DeviceRepository,
    user_id: &str,
    device_id: &str,
) -> Result<DeviceDTO, ApiErrors> {
    let find_result = repository.find_by_id(user_id, device_id).await;
    handle_get_result(find_result)
}

pub async fn search_devices(
    repository: &DeviceRepository,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<DevicePageResult, ApiErrors> {
    let search = handle_query_mapping::<DeviceDTO, DeviceSearch>(search, quicksearch)?;
    let find_result = repository.search_all(user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_device(
    repository: &DeviceRepository,
    user_id: &str,
    device: NewDeviceDTO,
) -> Result<DeviceDTO, ApiErrors> {
    create_merged(
        device,
        async move |created_device_id| get_device(repository, user_id, &created_device_id).await,
        async move |device_to_create| {
            let exists_result = repository
                .exists_with_unique(user_id, &device_to_create)
                .await;
            handle_already_exists_result::<DeviceDTO>(exists_result)?;

            let create_result = repository.create(user_id, &device_to_create).await;
            handle_create_result::<String, DeviceDTO>(create_result)
        },
    )
    .await
}

pub async fn update_device(
    repository: &DeviceRepository,
    user_id: &str,
    device_id: &str,
    device: NewDeviceDTO,
) -> Result<(), ApiErrors> {
    update_merged(
        device,
        async move || get_device(repository, user_id, device_id).await,
        async move |device_to_update| {
            let exists_result = repository
                .exists_with_unique_except_id(user_id, &device_to_update, device_id)
                .await;
            handle_already_exists_result::<DeviceDTO>(exists_result)?;

            let update_result = repository
                .update_by_id(user_id, device_id, &device_to_update)
                .await;
            handle_update_result::<DeviceDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_device(
    repository: &DeviceRepository,
    user_id: &str,
    device_id: &str,
) -> Result<(), ApiErrors> {
    let delete_result = repository.delete_by_id(user_id, device_id).await;
    handle_action_result::<DeviceDTO>(delete_result)
}

pub async fn exists_device(
    repository: &DeviceRepository,
    user_id: &str,
    device_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, device_id).await;
    handle_not_found_result::<DeviceDTO>(exists_result)
}
