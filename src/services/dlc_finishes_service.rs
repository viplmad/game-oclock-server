use chrono::NaiveDate;
use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::DLCFinish;
use crate::repository::dlc_finish_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result_raw,
    handle_not_found_result,
};
use super::dlcs_service;

pub async fn get_dlc_finishes(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
) -> Result<Vec<NaiveDate>, ApiErrors> {
    dlcs_service::exists_dlc(pool, user_id, dlc_id).await?;

    let find_result = dlc_finish_repository::find_all_by_dlc_id(pool, user_id, dlc_id).await;
    handle_get_list_result_raw::<NaiveDate, DLCFinish>(find_result)
}

pub async fn create_dlc_finish(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    dlcs_service::exists_dlc(pool, user_id, dlc_id).await?;

    let exists_result = dlc_finish_repository::exists_by_id(pool, user_id, dlc_id, date).await;
    handle_already_exists_result::<DLCFinish>(exists_result)?;

    let create_result = dlc_finish_repository::create(pool, user_id, dlc_id, date).await;
    handle_action_result::<DLCFinish>(create_result)
}

pub async fn delete_dlc_finish(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    dlcs_service::exists_dlc(pool, user_id, dlc_id).await?;
    exists_dlc_finish(pool, user_id, dlc_id, date).await?;

    let delete_result = dlc_finish_repository::delete(pool, user_id, dlc_id, date).await;
    handle_action_result::<DLCFinish>(delete_result)
}

pub async fn exists_dlc_finish(
    pool: &PgPool,
    user_id: i32,
    dlc_id: i32,
    date: NaiveDate,
) -> Result<(), ApiErrors> {
    let exists_result = dlc_finish_repository::exists_by_id(pool, user_id, dlc_id, date).await;
    handle_not_found_result::<DLCFinish>(exists_result)
}
