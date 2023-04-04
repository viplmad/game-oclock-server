use crate::errors::ApiErrors;
use crate::models::{DLCAvailableDTO, DLCWithFinishDTO, DLCDTO};
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, extract_image_name, handle_image_client_provider};

const DLC_FOLDER: &str = "DLC";
const DLC_HEADER_SUFFIX: &str = "header";

pub fn populate_dlc_cover(provider: &ImageClientProvider, dlc: &mut DLCDTO) {
    if let Ok(client) = handle_image_client_provider(provider) {
        if let Some(cover_filename) = &dlc.cover_filename {
            dlc.cover_url = Some(client.get_image_uri(DLC_FOLDER, cover_filename));
        }
    }
}

pub fn populate_dlcs_cover(provider: &ImageClientProvider, dlcs: &mut Vec<DLCDTO>) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for dlc in dlcs {
            if let Some(cover_filename) = &dlc.cover_filename {
                dlc.cover_url = Some(client.get_image_uri(DLC_FOLDER, cover_filename));
            }
        }
    }
}

pub fn populate_dlcs_available_cover(
    provider: &ImageClientProvider,
    dlcs: &mut Vec<DLCAvailableDTO>,
) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for dlc in dlcs {
            if let Some(cover_filename) = &dlc.cover_filename {
                dlc.cover_url = Some(client.get_image_uri(DLC_FOLDER, cover_filename));
            }
        }
    }
}

pub fn populate_dlcs_with_finish_cover(
    provider: &ImageClientProvider,
    dlcs: &mut Vec<DLCWithFinishDTO>,
) {
    if let Ok(client) = handle_image_client_provider(provider) {
        for dlc in dlcs {
            if let Some(cover_filename) = &dlc.cover_filename {
                dlc.cover_url = Some(client.get_image_uri(DLC_FOLDER, cover_filename));
            }
        }
    }
}

pub(super) async fn set_dlc_cover(
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    dlc_id: &str,
    file_path: &str,
) -> Result<String, ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let format_filename = build_dlc_cover_filename(user_id, dlc_id, Option::<String>::None);
    image_client
        .upload_image(file_path, DLC_FOLDER, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image upload error.")))
}

pub(super) async fn rename_dlc_cover(
    image_client_provider: &ImageClientProvider,
    user_id: &str,
    dlc_id: &str,
    old_filename: &str,
    new_name: &str,
) -> Result<String, ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let old_name = extract_image_name(old_filename)?;

    let format_filename = build_dlc_cover_filename(user_id, dlc_id, Some(String::from(new_name)));
    image_client
        .rename_image(DLC_FOLDER, &old_name, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image rename error.")))
}

pub(super) async fn delete_dlc_cover(
    image_client_provider: &ImageClientProvider,
    filename: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let name = extract_image_name(filename)?;

    image_client
        .delete_image(DLC_FOLDER, &name)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image delete error.")))
}

fn build_dlc_cover_filename(user_id: &str, dlc_id: &str, name: Option<String>) -> String {
    build_image_filename(user_id, dlc_id, DLC_HEADER_SUFFIX, name)
}
