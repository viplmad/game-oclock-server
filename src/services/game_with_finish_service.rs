use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::GameWithFinishSearch;
use crate::errors::ApiErrors;
use crate::models::{GameDTO, GameWithFinishSearchResult, SearchDTO};
use crate::repository::game_with_finish_repository;

use super::base::{handle_get_list_paged_result, handle_query_mapping};

pub async fn search_finished_games(
    pool: &PgPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithFinishSearchResult, ApiErrors> {
    let search = handle_query_mapping::<GameDTO, GameWithFinishSearch>(search, quicksearch)?;
    let find_result = game_with_finish_repository::search_all_by_date_between(
        pool, user_id, start_date, end_date, search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}
