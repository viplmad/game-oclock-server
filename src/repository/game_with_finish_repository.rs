use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{GameSearch, GameWithDate, PageResult};
use crate::errors::SearchErrors;
use crate::query::game_finish_query;

use super::base::fetch_all_search;

pub async fn search_first_by_date_between(
    pool: &PgPool,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: GameSearch,
) -> Result<PageResult<GameWithDate>, SearchErrors> {
    let search_query =
    game_finish_query::select_all_first_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_asc(
        user_id, start_date, end_date, search,
    )?;
    fetch_all_search(pool, search_query).await
}

pub async fn search_last_by_date_between(
    pool: &PgPool,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: GameSearch,
) -> Result<PageResult<GameWithDate>, SearchErrors> {
    let search_query =
        game_finish_query::select_all_last_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_desc(
            user_id, start_date, end_date, search,
        )?;
    fetch_all_search(pool, search_query).await
}
