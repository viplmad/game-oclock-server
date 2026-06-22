use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

use crate::entities::{
    MediaSession, SessionAggregateGroupSearch, SessionAggregateSearch, SessionListSearch,
};
use crate::errors::ApiErrors;
use crate::models::{
    AggregateGroupResultDTO, AggregateGroupSearchDTO, AggregateResultDTO, AggregateSearchDTO,
    FetchMode, ListSearchDTO, Merge, NewSessionDTO, SessionDTO, SessionPageResult,
    SessionStreakPageResult,
};
use crate::repository::MediaSessionRepository;

use super::helpers::{
    handle_action_result, handle_aggregate_group_search_mapping, handle_aggregate_search_mapping,
    handle_already_exists_result, handle_get_aggregate_group_result, handle_get_aggregate_result,
    handle_get_list_paged_result, handle_get_result, handle_list_search_mapping,
    handle_not_found_result,
};
use super::{DeviceService, MediaService, StoredResponseService};

#[derive(Clone)]
pub struct MediaSessionService {
    repository: MediaSessionRepository,
    media_service: MediaService,
    device_service: DeviceService,
    stored_response_service: StoredResponseService,
}

impl MediaSessionService {
    pub fn with(
        repository: MediaSessionRepository,
        media_service: MediaService,
        device_service: DeviceService,
        stored_response_service: StoredResponseService,
    ) -> Self {
        Self {
            repository,
            media_service,
            device_service,
            stored_response_service,
        }
    }
}

impl MediaSessionService {
    pub async fn get_media_session(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<FixedOffset>,
    ) -> Result<SessionDTO, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let find_result = self
            .repository
            .find_by_id(user_id, media_id, start_datetime)
            .await;
        handle_get_result(find_result)
    }

    pub async fn search_media_sessions(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<SessionPageResult, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search =
            handle_list_search_mapping::<SessionDTO, SessionListSearch>(search, quicksearch)?;
        let find_result = self
            .repository
            .search_all_by_media_id(user_id, media_id, search)
            .await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn search_sessions(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<SessionPageResult, ApiErrors> {
        let search =
            handle_list_search_mapping::<SessionDTO, SessionListSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn aggregate_media_sessions(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        self.media_service.exists_media(media_id).await?;

        let search = handle_aggregate_search_mapping::<SessionDTO, SessionAggregateSearch>(
            search,
            quicksearch,
        )?;
        let aggregate_result = self
            .repository
            .aggregate_all_by_media_id(user_id, media_id, search)
            .await;
        handle_get_aggregate_result::<SessionDTO>(aggregate_result)
    }

    pub async fn aggregate_sessions(
        &self,
        user_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
        mode: Option<FetchMode>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        let request = build_stored_request(&search, quicksearch.clone())?;
        self.stored_response_service
            .get_based_on_mode(mode, user_id, "SESSION_AGGREGATE", &request, || {
                self.calculate_aggregate_sessions(user_id, search, quicksearch)
            })
            .await
    }

    async fn calculate_aggregate_sessions(
        &self,
        user_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        let search = handle_aggregate_search_mapping::<SessionDTO, SessionAggregateSearch>(
            search,
            quicksearch,
        )?;
        let aggregate_result = self.repository.aggregate_all(user_id, search).await;
        handle_get_aggregate_result::<SessionDTO>(aggregate_result)
    }

    pub async fn aggregate_group_sessions(
        &self,
        user_id: &Uuid,
        search: AggregateGroupSearchDTO,
        quicksearch: Option<String>,
        mode: Option<FetchMode>,
    ) -> Result<Vec<AggregateGroupResultDTO>, ApiErrors> {
        let request = build_stored_request(&search, quicksearch.clone())?;
        self.stored_response_service
            .get_based_on_mode(mode, user_id, "SESSION_AGGREGATE_GROUP", &request, || {
                self.calculate_aggregate_group_sessions(user_id, search, quicksearch)
            })
            .await
    }

    async fn calculate_aggregate_group_sessions(
        &self,
        user_id: &Uuid,
        search: AggregateGroupSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<Vec<AggregateGroupResultDTO>, ApiErrors> {
        let search = handle_aggregate_group_search_mapping::<
            SessionDTO,
            SessionAggregateGroupSearch,
        >(search, quicksearch)?;
        let aggregate_result = self.repository.aggregate_group_all(user_id, search).await;
        handle_get_aggregate_group_result::<SessionDTO>(aggregate_result)
    }

    pub async fn aggregate_first_sessions(
        &self,
        user_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
        mode: Option<FetchMode>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        let request = build_stored_request(&search, quicksearch.clone())?;
        self.stored_response_service
            .get_based_on_mode(mode, user_id, "SESSION_FIRST_AGGREGATE", &request, || {
                self.calculate_aggregate_first_sessions(user_id, search, quicksearch)
            })
            .await
    }

    async fn calculate_aggregate_first_sessions(
        &self,
        user_id: &Uuid,
        search: AggregateSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<AggregateResultDTO, ApiErrors> {
        let search = handle_aggregate_search_mapping::<SessionDTO, SessionAggregateSearch>(
            search,
            quicksearch,
        )?;
        let aggregate_result = self.repository.aggregate_all_first(user_id, search).await;
        handle_get_aggregate_result::<SessionDTO>(aggregate_result)
    }

    pub async fn create_media_session(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        session: NewSessionDTO,
    ) -> Result<(), ApiErrors> {
        self.media_service.exists_media(media_id).await?;
        if let Some(device_id) = session.device_id {
            self.device_service
                .exists_device(user_id, &device_id)
                .await?;
        }

        let start_datetime = session.start_datetime;
        let end_datetime = session.end_datetime;

        if start_datetime > end_datetime {
            return Err(ApiErrors::InvalidParameter(String::from(
                "Session start datetime must be previous than end datetime",
            )));
        }

        if start_datetime.date_naive() == end_datetime.date_naive()
            && start_datetime.time() == end_datetime.time()
        {
            return Err(ApiErrors::InvalidParameter(String::from(
                "Session to add must not have an empty span of time",
            )));
        }

        let exists_result = self
            .repository
            .exists_gap(user_id, start_datetime, end_datetime)
            .await;
        handle_already_exists_result::<SessionDTO>(exists_result)?;

        let session_to_create = MediaSession::from(SessionDTO::merge_with_default(session));
        let create_result = self.repository.create(session_to_create).await;
        handle_action_result::<SessionDTO>(create_result)
    }

    pub async fn delete_media_session(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<FixedOffset>,
    ) -> Result<(), ApiErrors> {
        self.media_service.exists_media(media_id).await?;
        self.exists_media_session(user_id, media_id, start_datetime)
            .await?;

        let delete_result = self
            .repository
            .delete_by_id(user_id, media_id, start_datetime)
            .await;
        handle_action_result::<SessionDTO>(delete_result)
    }

    pub async fn exists_media_session(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<FixedOffset>,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id(user_id, media_id, start_datetime)
            .await;
        handle_not_found_result::<SessionDTO>(exists_result)
    }

    pub async fn search_streaks(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
        mode: Option<FetchMode>,
    ) -> Result<SessionStreakPageResult, ApiErrors> {
        let request = build_stored_request(&search, quicksearch.clone())?;
        self.stored_response_service
            .get_based_on_mode(mode, user_id, "SESSION_STREAKS", &request, || {
                self.calculate_search_streaks(user_id, search, quicksearch)
            })
            .await
    }

    async fn calculate_search_streaks(
        &self,
        user_id: &Uuid,
        search: ListSearchDTO,
        quicksearch: Option<String>,
    ) -> Result<SessionStreakPageResult, ApiErrors> {
        let search =
            handle_list_search_mapping::<SessionDTO, SessionListSearch>(search, quicksearch)?;
        let find_result = self.repository.search_streaks(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }
}

fn build_stored_request<R>(request: &R, quicksearch: Option<String>) -> Result<String, ApiErrors>
where
    R: Sized + serde::Serialize,
{
    Ok([
        crate::convert_utils::to_json_string(request, "Request")
            .map_err(|err| ApiErrors::UnknownError(err.0))?,
        quicksearch.unwrap_or_default(),
    ]
    .join("-"))
}
