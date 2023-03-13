use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::entities::{GameSearch, GameWithLog, PageResult};
use crate::errors::{RepositoryError, SearchErrors};
use crate::query::game_log_query;

use super::base::{fetch_all, fetch_all_search};

pub async fn search_first_by_datetime_between(
    pool: &PgPool,
    user_id: &str,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
    search: GameSearch,
) -> Result<PageResult<GameWithLog>, SearchErrors> {
    let search_query = game_log_query::select_all_first_game_with_log_with_search_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(user_id, start_datetime, end_datetime, search)?;
    fetch_all_search(pool, search_query).await
}

pub async fn search_last_by_datetime_between(
    pool: &PgPool,
    user_id: &str,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
    search: GameSearch,
) -> Result<PageResult<GameWithLog>, SearchErrors> {
    let search_query = game_log_query::select_all_last_game_with_log_with_search_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(user_id, start_datetime, end_datetime, search)?;
    fetch_all_search(pool, search_query).await
}

pub async fn find_all_by_datetime_between(
    pool: &PgPool,
    user_id: &str,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> Result<Vec<GameWithLog>, RepositoryError> {
    let query = game_log_query::select_all_games_log_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(user_id, start_datetime, end_datetime);
    fetch_all(pool, query).await
}
