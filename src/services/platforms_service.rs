use sqlx::PgPool;

use crate::entities::PlatformSearch;
use crate::errors::{error_message_builder, ApiErrors};
use crate::models::{NewPlatformDTO, PlatformDTO, PlatformPageResult, SearchDTO};
use crate::providers::ImageClientProvider;
use crate::repository::platform_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};
use super::platform_image_service;

pub async fn get_platform(
    pool: &PgPool,
    user_id: &str,
    platform_id: &str,
) -> Result<PlatformDTO, ApiErrors> {
    let find_result = platform_repository::find_by_id(pool, user_id, platform_id).await;
    handle_get_result(find_result)
}

pub async fn search_platforms(
    pool: &PgPool,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<PlatformPageResult, ApiErrors> {
    let search = handle_query_mapping::<PlatformDTO, PlatformSearch>(search, quicksearch)?;
    let find_result = platform_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_platform(
    pool: &PgPool,
    user_id: &str,
    platform: NewPlatformDTO,
) -> Result<PlatformDTO, ApiErrors> {
    create_merged(
        platform,
        async move |created_platform_id| get_platform(pool, user_id, &created_platform_id).await,
        async move |platform_to_create| {
            let exists_result =
                platform_repository::exists_with_unique(pool, user_id, &platform_to_create).await;
            handle_already_exists_result::<PlatformDTO>(exists_result)?;

            let create_result =
                platform_repository::create(pool, user_id, &platform_to_create).await;
            handle_create_result::<String, PlatformDTO>(create_result)
        },
    )
    .await
}

pub async fn update_platform(
    pool: &PgPool,
    user_id: &str,
    platform_id: &str,
    platform: NewPlatformDTO,
) -> Result<(), ApiErrors> {
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
            handle_update_result::<PlatformDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_platform(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    platform_id: &str,
) -> Result<(), ApiErrors> {
    let platform = get_platform(pool, user_id, platform_id).await?;

    if let Some(icon_filename) = &platform.icon_filename {
        let delete_icon_result =
            platform_image_service::delete_platform_icon(image_client_provider, icon_filename)
                .await;
        if delete_icon_result.is_err() {
            log::warn!("Platform deletion - Image client could not delete Platform with image.")
        }
    }

    let delete_result = platform_repository::delete_by_id(pool, user_id, platform_id).await;
    handle_action_result::<PlatformDTO>(delete_result)
}

pub async fn set_platform_icon(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    platform_id: &str,
    file_path: &str,
) -> Result<(), ApiErrors> {
    exists_platform(pool, user_id, platform_id).await?;
    let filename = platform_image_service::set_platform_icon(
        image_client_provider,
        user_id,
        platform_id,
        file_path,
    )
    .await?;
    set_platform_icon_filename(pool, user_id, platform_id, Some(filename)).await
}

pub async fn rename_platform_icon(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    platform_id: &str,
    new_name: &str,
) -> Result<(), ApiErrors> {
    let old_filename = get_platform_icon_filename(pool, user_id, platform_id).await?;
    let new_filename = platform_image_service::rename_platform_icon(
        image_client_provider,
        user_id,
        platform_id,
        &old_filename,
        new_name,
    )
    .await?;
    set_platform_icon_filename(pool, user_id, platform_id, Some(new_filename)).await
}

pub async fn delete_platform_icon(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    platform_id: &str,
) -> Result<(), ApiErrors> {
    let filename = get_platform_icon_filename(pool, user_id, platform_id).await?;
    platform_image_service::delete_platform_icon(image_client_provider, &filename).await?;
    set_platform_icon_filename(pool, user_id, platform_id, Option::<String>::None).await
}

pub async fn exists_platform(
    pool: &PgPool,
    user_id: &str,
    platform_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = platform_repository::exists_by_id(pool, user_id, platform_id).await;
    handle_not_found_result::<PlatformDTO>(exists_result)
}

async fn get_platform_icon_filename(
    pool: &PgPool,
    user_id: &str,
    platform_id: &str,
) -> Result<String, ApiErrors> {
    let platform = get_platform(pool, user_id, platform_id).await?;
    platform.icon_filename.ok_or_else(|| {
        ApiErrors::InvalidParameter(error_message_builder::empty_param("Platform icon"))
    })
}

async fn set_platform_icon_filename(
    pool: &PgPool,
    user_id: &str,
    platform_id: &str,
    filename: Option<String>,
) -> Result<(), ApiErrors> {
    let update_result =
        platform_repository::update_icon_filename_by_id(pool, user_id, platform_id, filename).await;
    handle_action_result::<PlatformDTO>(update_result)
}
