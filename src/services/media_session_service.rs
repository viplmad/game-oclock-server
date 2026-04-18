use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::entities::{
    MediaSession, MediaSessionWithTime, SessionAggregateGroupSearch, SessionAggregateSearch,
    SessionListSearch,
};
use crate::errors::ApiErrors;
use crate::models::{
    AggregateGroupResultKeyDTO, AggregateGroupSearchDTO, AggregateResultDTO, AggregateSearchDTO,
    ListSearchDTO, Merge, NewSessionDTO, SessionDTO, SessionPageResult,
};
use crate::repository::MediaSessionRepository;

use super::helpers::{
    handle_action_result, handle_aggregate_group_search_mapping, handle_aggregate_search_mapping,
    handle_already_exists_result, handle_get_aggregate_group_result, handle_get_aggregate_result,
    handle_get_list_paged_result, handle_get_result, handle_list_search_mapping,
    handle_not_found_result, handle_result,
};
use super::{DeviceService, MediaService};

#[derive(Clone)]
pub struct MediaSessionService {
    repository: MediaSessionRepository,
    media_service: MediaService,
    device_service: DeviceService,
}

impl MediaSessionService {
    pub fn with(
        repository: MediaSessionRepository,
        media_service: MediaService,
        device_service: DeviceService,
    ) -> Self {
        Self {
            repository,
            media_service,
            device_service,
        }
    }
}

impl MediaSessionService {
    pub async fn get_media_session(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<Utc>,
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
    ) -> Result<HashMap<AggregateGroupResultKeyDTO, AggregateResultDTO>, ApiErrors> {
        let search = handle_aggregate_group_search_mapping::<
            SessionDTO,
            SessionAggregateGroupSearch,
        >(search, quicksearch)?;
        let aggregate_result = self.repository.aggregate_group_all(user_id, search).await;
        handle_get_aggregate_group_result::<SessionDTO>(aggregate_result)
    }

    // For review
    pub(super) async fn find_first_media_sessions_by_medias(
        &self,
        user_id: &Uuid,
        media_ids: Vec<Uuid>,
    ) -> Result<Vec<MediaSessionWithTime>, ApiErrors> {
        let find_result = self
            .repository
            .find_all_first_by_user_id_and_media_id_in(user_id, media_ids)
            .await;
        handle_result::<Vec<MediaSessionWithTime>, SessionDTO>(find_result)
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
        start_datetime: DateTime<Utc>,
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
        start_datetime: DateTime<Utc>,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id(user_id, media_id, start_datetime)
            .await;
        handle_not_found_result::<SessionDTO>(exists_result)
    }
}
