use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{DLCAvailableDTO, DLCWithFinishDTO, DLCDTO};
use crate::providers::ImageClientProvider;

use super::base::{build_image_filename, extract_image_name, handle_image_client_provider};
use super::dlcs_service;

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

pub async fn set_dlc_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    dlc_id: i32,
    file_path: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    dlcs_service::exists_dlc(pool, user_id, dlc_id).await?;

    let format_filename = build_dlc_cover_filename(user_id, dlc_id, Option::<String>::None);
    let filename = image_client
        .upload_image(file_path, DLC_FOLDER, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image upload error.")))?;

    dlcs_service::set_dlc_cover_filename(pool, user_id, dlc_id, Some(filename)).await
}

pub async fn rename_dlc_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    dlc_id: i32,
    new_name: &str,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let old_filename = dlcs_service::get_dlc_cover_filename(pool, user_id, dlc_id).await?;
    let old_name = extract_image_name(&old_filename)?;

    let format_filename = build_dlc_cover_filename(user_id, dlc_id, Some(String::from(new_name)));
    let filename = image_client
        .rename_image(DLC_FOLDER, &old_name, &format_filename)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image rename error.")))?;

    dlcs_service::set_dlc_cover_filename(pool, user_id, dlc_id, Some(filename)).await
}

pub async fn delete_dlc_cover(
    pool: &PgPool,
    image_client_provider: &ImageClientProvider,
    user_id: i32,
    dlc_id: i32,
) -> Result<(), ApiErrors> {
    let image_client = handle_image_client_provider(image_client_provider)?;

    let filename = dlcs_service::get_dlc_cover_filename(pool, user_id, dlc_id).await?;
    let name = extract_image_name(&filename)?;

    image_client
        .delete_image(DLC_FOLDER, &name)
        .await
        .map_err(|_| ApiErrors::UnknownError(String::from("Image delete error.")))?;

    dlcs_service::set_dlc_cover_filename(pool, user_id, dlc_id, Option::<String>::None).await
}

fn build_dlc_cover_filename(user_id: i32, dlc_id: i32, name: Option<String>) -> String {
    build_image_filename(user_id, dlc_id, DLC_HEADER_SUFFIX, name)
}
