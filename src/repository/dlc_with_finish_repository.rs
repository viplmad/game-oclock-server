use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{DLCSearch, DLCWithDate, SearchResult};
use crate::errors::RepositoryError;
use crate::query::dlc_finish_query;

use super::base::fetch_all_search;

pub async fn search_first_by_date_between(
    pool: &PgPool,
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: DLCSearch,
) -> Result<SearchResult<DLCWithDate>, RepositoryError> {
    let search_query =
    dlc_finish_query::select_all_first_dlc_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_asc(
        user_id, start_date, end_date, search,
    )?;
    fetch_all_search(pool, search_query).await
}

pub async fn search_last_by_date_between(
    pool: &PgPool,
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: DLCSearch,
) -> Result<SearchResult<DLCWithDate>, RepositoryError> {
    let search_query =
        dlc_finish_query::select_all_last_dlc_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_desc(
            user_id, start_date, end_date, search,
        )?;
    fetch_all_search(pool, search_query).await
}
