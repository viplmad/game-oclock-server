use chrono::NaiveDate;
use uuid::Uuid;

use crate::entities::{GameSearch, GameWithFinish};
use crate::errors::ApiErrors;
use crate::models::{GameWithFinishDTO, GameWithFinishPageResult, SearchDTO};
use crate::repository::GameWithFinishRepository;

use super::helpers::{
    check_optional_start_end, check_start_end, handle_get_list_paged_result, handle_query_mapping,
    handle_result, optional_start_end_to_datetime, start_end_to_datetime,
};

#[derive(Clone)]
pub struct GameWithFinishService {
    repository: GameWithFinishRepository,
}

impl GameWithFinishService {
    pub fn with(repository: GameWithFinishRepository) -> Self {
        Self { repository }
    }
}

impl GameWithFinishService {
    pub async fn search_first_finished_games(
        &self,
        user_id: &Uuid,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<GameWithFinishPageResult, ApiErrors> {
        check_optional_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
        let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_first_by_date_between(user_id, start_datetime, end_datetime, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn search_last_finished_games(
        &self,
        user_id: &Uuid,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<GameWithFinishPageResult, ApiErrors> {
        check_optional_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
        let search = handle_query_mapping::<GameWithFinishDTO, GameSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_last_by_date_between(user_id, start_datetime, end_datetime, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    // For review
    pub(super) async fn find_game_with_finishes_between(
        &self,
        user_id: &Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<GameWithFinish>, ApiErrors> {
        check_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
        let find_result = self
            .repository
            .find_all_by_date_between(user_id, start_datetime, end_datetime)
            .await;
        handle_result::<Vec<GameWithFinish>, GameWithFinishDTO>(find_result)
    }
}
