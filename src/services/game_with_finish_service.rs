use chrono::NaiveDate;

use crate::entities::{GameSearch, GameWithFinish};
use crate::errors::ApiErrors;
use crate::models::{GameWithFinishDTO, GameWithFinishPageResult, SearchDTO};
use crate::repository::GameWithFinishRepository;

use super::base::{
    check_optional_start_end, check_start_end, handle_get_list_paged_result, handle_query_mapping,
    handle_result,
};

pub async fn search_first_finished_games(
    repository: &GameWithFinishRepository,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithFinishPageResult, ApiErrors> {
    check_optional_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
    let find_result = repository
        .search_first_by_date_between(user_id, start_date, end_date, search)
        .await;
    handle_get_list_paged_result(find_result)
}

pub async fn search_last_finished_games(
    repository: &GameWithFinishRepository,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithFinishPageResult, ApiErrors> {
    check_optional_start_end(start_date, end_date)?;

    let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
    let find_result = repository
        .search_last_by_date_between(user_id, start_date, end_date, search)
        .await;
    handle_get_list_paged_result(find_result)
}

pub(super) async fn find_game_with_finishes_between(
    repository: &GameWithFinishRepository,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameWithFinish>, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let find_result = repository
        .find_all_by_date_between(user_id, start_date, end_date)
        .await;
    handle_result::<Vec<GameWithFinish>, GameWithFinishDTO>(find_result)
}
