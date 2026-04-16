use chrono::NaiveDate;
use uuid::Uuid;

use crate::entities::{MediaListSearch, MediaWithStateWithSession};
use crate::errors::ApiErrors;
use crate::models::{ListSearchDTO, MediaSessionDTO, MediaSessionPageResult};
use crate::repository::MediaWithSessionRepository;

use super::helpers::{
    check_optional_start_end, check_start_end, handle_get_list_paged_result,
    handle_list_search_mapping, handle_result, optional_start_end_to_datetime,
    start_end_to_datetime,
};

#[derive(Clone)]
pub struct MediaWithSessionService {
    repository: MediaWithSessionRepository,
}

impl MediaWithSessionService {
    pub fn with(repository: MediaWithSessionRepository) -> Self {
        Self { repository }
    }
}

impl MediaWithSessionService {
    pub async fn search_first_session_medias(
        &self,
        user_id: &Uuid,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<MediaSessionPageResult, ApiErrors> {
        check_optional_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
        let search =
            handle_list_search_mapping::<MediaSessionDTO, MediaListSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_first_by_start_datetime_between(user_id, start_datetime, end_datetime, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn search_last_session_medias(
        &self,
        user_id: &Uuid,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<MediaSessionPageResult, ApiErrors> {
        check_optional_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
        let search =
            handle_list_search_mapping::<MediaSessionDTO, MediaListSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_last_by_start_datetime_between(user_id, start_datetime, end_datetime, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    // For review
    pub(super) async fn find_media_with_sessions_between(
        &self,
        user_id: &Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<MediaWithStateWithSession>, ApiErrors> {
        check_start_end(start_date, end_date)?;

        let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
        let find_result = self
            .repository
            .find_all_by_start_datetime_between(user_id, start_datetime, end_datetime)
            .await;
        handle_result::<Vec<MediaWithStateWithSession>, MediaSessionDTO>(find_result)
    }
}
