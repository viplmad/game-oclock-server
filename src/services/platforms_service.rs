use sqlx::PgPool;

use crate::entities::PlatformSearch;
use crate::errors::ApiErrors;
use crate::models::{NewPlatformDTO, PlatformDTO, PlatformSearchResult, SearchDTO};
use crate::repository::platform_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

pub async fn get_platform(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
) -> Result<PlatformDTO, ApiErrors> {
    let find_result = platform_repository::find_by_id(pool, user_id, platform_id).await;
    handle_get_result(find_result)
}

pub async fn search_platforms(
    pool: &PgPool,
    user_id: i32,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<PlatformSearchResult, ApiErrors> {
    let search = handle_query_mapping::<PlatformDTO, PlatformSearch>(search, quicksearch)?;
    let find_result = platform_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_platform(
    pool: &PgPool,
    user_id: i32,
    platform: NewPlatformDTO,
) -> Result<PlatformDTO, ApiErrors> {
    create_merged(
        platform,
        async move |created_platform_id| get_platform(pool, user_id, created_platform_id).await,
        async move |platform_to_create| {
            let exists_result =
                platform_repository::exists_with_unique(pool, user_id, &platform_to_create).await;
            handle_already_exists_result::<PlatformDTO>(exists_result)?;

            let create_result =
                platform_repository::create(pool, user_id, &platform_to_create).await;
            handle_create_result::<i32, PlatformDTO>(create_result)
        },
    )
    .await
}

pub async fn update_platform(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
    platform: NewPlatformDTO,
) -> Result<PlatformDTO, ApiErrors> {
    update_merged(
        platform,
        async move || get_platform(pool, user_id, platform_id).await,
        async move |platform_to_update| {
            let exists_result = platform_repository::exists_with_unique_except_id(
                pool,
                user_id,
                &platform_to_update,
                platform_id,
            )
            .await;
            handle_already_exists_result::<PlatformDTO>(exists_result)?;

            let update_result =
                platform_repository::update_by_id(pool, user_id, platform_id, &platform_to_update)
                    .await;
            handle_update_result::<i32, PlatformDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_platform(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
) -> Result<(), ApiErrors> {
    exists_platform(pool, user_id, platform_id).await?;

    let delete_result = platform_repository::delete_by_id(pool, user_id, platform_id).await;
    handle_action_result::<PlatformDTO>(delete_result)
}

pub async fn exists_platform(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
) -> Result<(), ApiErrors> {
    let exists_result = platform_repository::exists_by_id(pool, user_id, platform_id).await;
    handle_not_found_result::<PlatformDTO>(exists_result)
}
