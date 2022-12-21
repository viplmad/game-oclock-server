use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::GameSearch;
use crate::errors::ApiErrors;
use crate::models::{GameWithFinishDTO, GameWithFinishSearchResult, SearchDTO};
use crate::repository::game_with_finish_repository;

use super::base::{handle_get_list_paged_result, handle_query_mapping};

pub async fn search_first_finished_games(
    pool: &PgPool,
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithFinishSearchResult, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_finish_repository::search_first_by_date_between(
        pool, user_id, start_date, end_date, search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}

pub async fn search_last_finished_games(
    pool: &PgPool,
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithFinishSearchResult, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_finish_repository::search_last_by_date_between(
        pool, user_id, start_date, end_date, search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}

fn check_start_end(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<(), ApiErrors> {
    if start_date.is_none() && end_date.is_none() {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Start date and end date cannot be empty",
        )));
    }
    if start_date.is_some_and(|start| end_date.is_some_and(|end| start > end)) {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Start date must be previous than end date",
        )));
    }
    Ok(())
}
