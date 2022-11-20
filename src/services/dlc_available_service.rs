use chrono::NaiveDate;
use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{DLCAvailableDTO, PlatformAvailableDTO};
use crate::repository::dlc_available_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::{dlcs_service, platforms_service};

pub async fn get_platform_dlcs(
    pool: &PgPool,
    user_id: i32,
    platform_id: i32,
) -> Result<Vec<DLCAvailableDTO>, ApiErrors> {
    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let find_result =
        dlc_available_repository::find_all_dlcs_with_platform(pool, user_id, platform_id).await;
    handle_get_list_result(find_result)
}

pub async fn get_dlc_platforms(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
) -> Result<Vec<PlatformAvailableDTO>, ApiErrors> {
    dlcs_service::exists_dlc(pool, user_id, dlc_id).await?;

    let find_result =
        dlc_available_repository::find_all_platforms_with_dlc(pool, user_id, dlc_id).await;
    handle_get_list_result(find_result)
}

pub async fn create_dlc_available(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    platform_id: i32,
    available_date: NaiveDate,
) -> Result<(), ApiErrors> {
    dlcs_service::exists_dlc(pool, user_id, dlc_id).await?;
    platforms_service::exists_platform(pool, user_id, platform_id).await?;

    let exists_result =
        dlc_available_repository::exists_by_id(pool, user_id, dlc_id, platform_id).await;
    handle_already_exists_result::<DLCAvailableDTO>(exists_result)?;

    let create_result =
        dlc_available_repository::create(pool, user_id, dlc_id, platform_id, available_date).await;
    handle_action_result::<DLCAvailableDTO>(create_result)
}

pub async fn delete_dlc_available(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    platform_id: i32,
) -> Result<(), ApiErrors> {
    exists_dlc_available(pool, user_id, dlc_id, platform_id).await?;

    let delete_result =
        dlc_available_repository::delete_by_id(pool, user_id, dlc_id, platform_id).await;
    handle_action_result::<DLCAvailableDTO>(delete_result)
}

pub async fn exists_dlc_available(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    platform_id: i32,
) -> Result<(), ApiErrors> {
    let exists_result =
        dlc_available_repository::exists_by_id(pool, user_id, dlc_id, platform_id).await;
    handle_not_found_result::<DLCAvailableDTO>(exists_result)
}
