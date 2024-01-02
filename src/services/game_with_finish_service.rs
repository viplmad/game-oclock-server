use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{GameSearch, GameWithDate};
use crate::errors::ApiErrors;
use crate::models::{GameWithFinishDTO, GameWithFinishPageResult, SearchDTO};
use crate::repository::game_with_finish_repository;

use super::base::{
    check_optional_start_end, check_start_end, handle_get_list_paged_result, handle_query_mapping,
    handle_result,
};

pub async fn search_first_finished_games(
    pool: &PgPool,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithFinishPageResult, ApiErrors> {
    check_optional_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_finish_repository::search_first_by_date_between(
        pool, user_id, start_date, end_date, search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}

pub async fn search_last_finished_games(
    pool: &PgPool,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithFinishPageResult, ApiErrors> {
    check_optional_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_finish_repository::search_last_by_date_between(
        pool, user_id, start_date, end_date, search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}

pub(super) async fn find_game_with_finishes_between(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameWithDate>, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let find_result =
        game_with_finish_repository::find_all_by_date_between(pool, user_id, start_date, end_date)
            .await;
    handle_result::<Vec<GameWithDate>, GameWithFinishDTO>(find_result)
}
