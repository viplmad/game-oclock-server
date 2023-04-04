use crate::errors::ApiErrors;
use crate::models::{PlatformAvailableDTO, PlatformDTO};
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, extract_image_name, handle_image_client_provider};

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

pub(super) async fn set_platform_icon(
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    platform_id: &str,
    file_path: &str,
) -> Result<String, ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let format_filename =
        build_platform_icon_filename(user_id, platform_id, Option::<String>::None);
    image_client
        .upload_image(file_path, PLATFORM_FOLDER, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image upload error.")))
}

pub(super) async fn rename_platform_icon(
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    platform_id: &str,
    old_filename: &str,
    new_name: &str,
) -> Result<String, ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let old_name = extract_image_name(old_filename)?;

    let format_filename =
        build_platform_icon_filename(user_id, platform_id, Some(String::from(new_name)));
    image_client
        .rename_image(PLATFORM_FOLDER, &old_name, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image rename error.")))
}

pub(super) async fn delete_platform_icon(
    image_client_provider: &ImageClientProvider,
    filename: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let name = extract_image_name(filename)?;

    image_client
        .delete_image(PLATFORM_FOLDER, &name)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image delete error.")))
}

fn build_platform_icon_filename(user_id: &str, platform_id: &str, name: Option<String>) -> String {
    build_image_filename(user_id, platform_id, PLATFORM_ICON_SUFFIX, name)
}
