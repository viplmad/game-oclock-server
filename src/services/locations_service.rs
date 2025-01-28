use sqlx::PgPool;

use crate::entities::LocationSearch;
use crate::errors::ApiErrors;
use crate::models::{LocationDTO, LocationPageResult, NewLocationDTO, SearchDTO};
use crate::repository::location_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

pub async fn get_location(
    pool: &PgPool,
    user_id: &str,
    location_id: &str,
) -> Result<LocationDTO, ApiErrors> {
    let find_result = location_repository::find_by_id(pool, user_id, location_id).await;
    handle_get_result(find_result)
}

pub async fn search_locations(
    pool: &PgPool,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<LocationPageResult, ApiErrors> {
    let search = handle_query_mapping::<LocationDTO, LocationSearch>(search, quicksearch)?;
    let find_result = location_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_location(
    pool: &PgPool,
    user_id: &str,
    location: NewLocationDTO,
) -> Result<LocationDTO, ApiErrors> {
    create_merged(
        location,
        async move |created_location_id| get_location(pool, user_id, &created_location_id).await,
        async move |location_to_create| {
            let exists_result =
                location_repository::exists_with_unique(pool, user_id, &location_to_create).await;
            handle_already_exists_result::<LocationDTO>(exists_result)?;

            let create_result =
                location_repository::create(pool, user_id, &location_to_create).await;
            handle_create_result::<String, LocationDTO>(create_result)
        },
    )
    .await
}

pub async fn update_location(
    pool: &PgPool,
    user_id: &str,
    location_id: &str,
    location: NewLocationDTO,
) -> Result<(), ApiErrors> {
    update_merged(
        location,
        async move || get_location(pool, user_id, location_id).await,
        async move |location_to_update| {
            let exists_result = location_repository::exists_with_unique_except_id(
                pool,
                user_id,
                &location_to_update,
                location_id,
            )
            .await;
            handle_already_exists_result::<LocationDTO>(exists_result)?;

            let update_result =
                location_repository::update_by_id(pool, user_id, location_id, &location_to_update)
                    .await;
            handle_update_result::<LocationDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_location(
    pool: &PgPool,
    user_id: &str,
    location_id: &str,
) -> Result<(), ApiErrors> {
    let delete_result = location_repository::delete_by_id(pool, user_id, location_id).await;
    handle_action_result::<LocationDTO>(delete_result)
}

pub async fn exists_location(
    pool: &PgPool,
    user_id: &str,
    location_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = location_repository::exists_by_id(pool, user_id, location_id).await;
    handle_not_found_result::<LocationDTO>(exists_result)
}
