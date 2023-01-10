use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::DLCSearch;
use crate::errors::ApiErrors;
use crate::models::{DLCWithFinishDTO, DLCWithFinishPageResult, SearchDTO};
use crate::repository::dlc_with_finish_repository;

use super::base::{check_start_end, handle_get_list_paged_result, handle_query_mapping};

pub async fn search_first_finished_dlcs(
    pool: &PgPool,
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<DLCWithFinishPageResult, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<DLCWithFinishDTO, DLCSearch>(search, quicksearch)?;
    let find_result = dlc_with_finish_repository::search_first_by_date_between(
        pool, user_id, start_date, end_date, search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}

pub async fn search_last_finished_dlcs(
    pool: &PgPool,
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<DLCWithFinishPageResult, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<DLCWithFinishDTO, DLCSearch>(search, quicksearch)?;
    let find_result = dlc_with_finish_repository::search_last_by_date_between(
        pool, user_id, start_date, end_date, search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}
