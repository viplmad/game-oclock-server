use chrono::{DateTime, FixedOffset};
use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_session_query;
use crate::entities::{
    AggregateGroupResult, AggregateResult, MediaSession, MediaSessionStreak, MediaSessionWithTime,
    PageResult, SessionAggregateGroupSearch, SessionAggregateSearch, SessionListSearch,
};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{
    aggregate_all_search, aggregate_group_search, execute, exists_some, fetch_all_search,
    fetch_optional,
};

#[derive(Clone)]
pub struct MediaSessionRepository {
    pool: PgPool,
}

impl MediaSessionRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MediaSessionRepository {
    pub async fn find_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<FixedOffset>,
    ) -> Result<Option<MediaSessionWithTime>, RepositoryError> {
        let query = media_session_query::select_by_id(user_id, media_id, start_datetime);
        fetch_optional(&self.pool, query).await
    }

    pub async fn search_all_by_media_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SessionListSearch,
    ) -> Result<PageResult<MediaSessionWithTime>, SearchErrors> {
        let query =
            media_session_query::select_all_by_media_id_with_search(user_id, media_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &Uuid,
        search: SessionListSearch,
    ) -> Result<PageResult<MediaSessionWithTime>, SearchErrors> {
        let query = media_session_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all_by_media_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SessionAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query =
            media_session_query::aggregate_all_by_media_id_with_search(user_id, media_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all(
        &self,
        user_id: &Uuid,
        search: SessionAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = media_session_query::aggregate_all_with_search(user_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn aggregate_group_all(
        &self,
        user_id: &Uuid,
        search: SessionAggregateGroupSearch,
    ) -> Result<Vec<AggregateGroupResult>, SearchErrors> {
        let query = media_session_query::aggregate_group_with_search(user_id, search)?;
        aggregate_group_search(&self.pool, query).await
    }

    pub async fn aggregate_all_first(
        &self,
        user_id: &Uuid,
        search: SessionAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = media_session_query::aggregate_all_first_with_search(user_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn create(&self, media_session: MediaSession) -> Result<(), RepositoryError> {
        let query = media_session_query::insert(&media_session);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<FixedOffset>,
    ) -> Result<(), RepositoryError> {
        let query = media_session_query::delete_by_id(user_id, media_id, start_datetime);
        execute(&self.pool, query).await
    }

    pub async fn exists_gap(
        &self,
        user_id: &Uuid,
        start_datetime: DateTime<FixedOffset>,
        end_datetime: DateTime<FixedOffset>,
    ) -> Result<bool, RepositoryError> {
        let query = media_session_query::exists_by_start_datetime_lt_or_end_datetime_gt(
            user_id,
            end_datetime,
            start_datetime,
        );
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<FixedOffset>,
    ) -> Result<bool, RepositoryError> {
        let query = media_session_query::exists_by_id(user_id, media_id, start_datetime);
        exists_some(&self.pool, query).await
    }

    pub async fn search_streaks(
        &self,
        user_id: &Uuid,
        search: SessionListSearch,
    ) -> Result<PageResult<MediaSessionStreak>, SearchErrors> {
        let query = media_session_query::select_streaks_with_search(user_id, search)?;
        fetch_all_search(&self.pool, query).await
    }
}
