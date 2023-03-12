use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{PlatformAvailableDTO, PlatformDTO};
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, extract_image_name, handle_image_client_provider};
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

pub fn populate_platforms_icon(provider: &ImageClientProvider, platforms: &mut Vec<PlatformDTO>) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for platform in platforms {
            if let Some(icon_filename) = &platform.icon_filename {
                platform.icon_url = Some(client.get_image_uri(PLATFORM_FOLDER, icon_filename));
            }
        }
    }
}

pub fn populate_platforms_available_icon(
    provider: &ImageClientProvider,
    platforms: &mut Vec<PlatformAvailableDTO>,
) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for platform in platforms {
            if let Some(icon_filename) = &platform.icon_filename {
                platform.icon_url = Some(client.get_image_uri(PLATFORM_FOLDER, icon_filename));
            }
        }
    }
}

pub async fn set_platform_icon(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    platform_id: i32,
    file_path: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let format_filename =
        build_platform_icon_filename(user_id, platform_id, Option::<String>::None);
    let filename = image_client
        .upload_image(file_path, PLATFORM_FOLDER, &format_filename)
        .await
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
    let old_name = extract_image_name(&old_filename)?;

    let format_filename =
        build_platform_icon_filename(user_id, platform_id, Some(String::from(new_name)));
    let filename = image_client
        .rename_image(PLATFORM_FOLDER, &old_name, &format_filename)
        .await
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
    let name = extract_image_name(&filename)?;

    image_client
        .delete_image(PLATFORM_FOLDER, &name)
        .await
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
