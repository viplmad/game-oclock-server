use std::fs::File;

use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::PlatformDTO;
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, handle_image_client_provider};
use super::platforms_service;

const PLATFORM_FOLDER: &str = "Platform";
const PLATFORM_ICON_SUFFIX: &str = "icon";

pub fn populate_platform_icon(provider: &ImageClientProvider, platform: &mut PlatformDTO) {
    if let Ok(client) = handle_image_client_provider(provider) {
        if let Some(icon_filename) = &platform.icon_filename {
            platform.icon_url = Some(client.get_image_uri(PLATFORM_FOLDER, icon_filename));
        }
    }
}

pub async fn set_platform_icon(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    platform_id: i32,
    file_result: Result<File, ApiErrors>,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;
    let file = file_result?;

    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let format_filename =
        build_platform_icon_filename(user_id, platform_id, Option::<String>::None);
    let filename = image_client
        .upload_image(file, PLATFORM_FOLDER, &format_filename)
        .map_err(|_| ApiErrors::UnknownError(String::from("Image upload error.")))?;

    platforms_service::set_platform_icon_filename(pool, user_id, platform_id, Some(filename)).await
}

pub async fn rename_platform_icon(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    platform_id: i32,
    new_name: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let old_filename =
        platforms_service::get_platform_icon_filename(pool, user_id, platform_id).await?;

    let format_filename =
        build_platform_icon_filename(user_id, platform_id, Some(String::from(new_name)));
    let filename = image_client
        .rename_image(PLATFORM_FOLDER, &old_filename, &format_filename)
        .map_err(|_| ApiErrors::UnknownError(String::from("Image rename error.")))?;

    platforms_service::set_platform_icon_filename(pool, user_id, platform_id, Some(filename)).await
}

pub async fn delete_platform_icon(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    platform_id: i32,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let filename =
        platforms_service::get_platform_icon_filename(pool, user_id, platform_id).await?;

    image_client
        .delete_image(PLATFORM_FOLDER, &filename)
        .map_err(|_| ApiErrors::UnknownError(String::from("Image delete error.")))?;

    platforms_service::set_platform_icon_filename(
        pool,
        user_id,
        platform_id,
        Option::<String>::None,
    )
    .await
}

fn build_platform_icon_filename(user_id: i32, platform_id: i32, name: Option<String>) -> String {
    build_image_filename(user_id, platform_id, PLATFORM_ICON_SUFFIX, name)
}