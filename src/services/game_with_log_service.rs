use chrono::NaiveDate;

use crate::entities::{GameSearch, GameWithLog};
use crate::errors::ApiErrors;
use crate::models::{GameWithLogDTO, GameWithLogPageResult, SearchDTO};
use crate::repository::GameWithLogRepository;

use super::helpers::{
    check_optional_start_end, check_start_end, handle_get_list_paged_result, handle_query_mapping,
    handle_result, optional_start_end_to_datetime, start_end_to_datetime,
};

#[derive(Clone)]
pub struct GameWithLogService {
    repository: GameWithLogRepository,
}

impl GameWithLogService {
    pub fn with(repository: GameWithLogRepository) -> Self {
        Self { repository }
    }
}

impl GameWithLogService {
    pub async fn search_first_played_games(
        &self,
        user_id: &str,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<GameWithLogPageResult, ApiErrors> {
        check_optional_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
        let search = handle_query_mapping::<GameWithLogDTO, GameSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_first_by_start_datetime_between(user_id, start_datetime, end_datetime, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn search_last_played_games(
        &self,
        user_id: &str,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<GameWithLogPageResult, ApiErrors> {
        check_optional_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
        let search = handle_query_mapping::<GameWithLogDTO, GameSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_last_by_start_datetime_between(user_id, start_datetime, end_datetime, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    // For review
    pub(super) async fn find_game_with_logs_between(
        &self,
        user_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<GameWithLog>, ApiErrors> {
        check_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
        let find_result = self
            .repository
            .find_all_by_start_datetime_between(user_id, start_datetime, end_datetime)
            .await;
        handle_result::<Vec<GameWithLog>, GameWithLogDTO>(find_result)
    }
}
