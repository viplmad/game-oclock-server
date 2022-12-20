use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{GameWithDate, GameWithFinishSearch, SearchResult};
use crate::errors::RepositoryError;
use crate::query::game_finish_query;

use super::base::fetch_all_search;

pub async fn search_all_by_date_between(
    pool: &PgPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    search: GameWithFinishSearch,
) -> Result<SearchResult<GameWithDate>, RepositoryError> {
    let search_query =
        game_finish_query::search_all_games_finish_with_search_by_date_gte_and_date_lte_order_by_date_desc(
            user_id, start_date, end_date, search
        )?;
    fetch_all_search(pool, search_query).await
}
