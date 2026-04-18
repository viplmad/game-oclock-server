use std::collections::HashMap;

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_session_query;
use crate::entities::{
    AggregateGroupResultKey, AggregateResult, MediaSession, MediaSessionWithTime, PageResult,
    SessionAggregateGroupSearch, SessionAggregateSearch, SessionListSearch,
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
        start_datetime: DateTime<Utc>,
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
            media_session_query::select_all_by_user_id_and_media_id(user_id, media_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &Uuid,
        search: SessionListSearch,
    ) -> Result<PageResult<MediaSessionWithTime>, SearchErrors> {
        let query = media_session_query::select_all_by_user_id(user_id, search)?;
        fetch_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all_by_media_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SessionAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query =
            media_session_query::aggregate_all_by_user_id_and_media_id(user_id, media_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn aggregate_all(
        &self,
        user_id: &Uuid,
        search: SessionAggregateSearch,
    ) -> Result<AggregateResult, SearchErrors> {
        let query = media_session_query::aggregate_all_by_user_id(user_id, search)?;
        aggregate_all_search(&self.pool, query).await
    }

    pub async fn aggregate_group_all(
        &self,
        user_id: &Uuid,
        search: SessionAggregateGroupSearch,
    ) -> Result<HashMap<AggregateGroupResultKey, AggregateResult>, SearchErrors> {
        let query = media_session_query::aggregate_group_by_user_id(user_id, search)?;
        aggregate_group_search(&self.pool, query).await
    }

    pub async fn create(&self, media_session: MediaSession) -> Result<(), RepositoryError> {
        let query = media_session_query::insert(&media_session);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        start_datetime: DateTime<Utc>,
    ) -> Result<(), RepositoryError> {
        let query = media_session_query::delete_by_id(user_id, media_id, start_datetime);
        execute(&self.pool, query).await
    }

    pub async fn exists_gap(
        &self,
        user_id: &Uuid,
        start_datetime: DateTime<Utc>,
        end_datetime: DateTime<Utc>,
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
        start_datetime: DateTime<Utc>,
    ) -> Result<bool, RepositoryError> {
        let query = media_session_query::exists_by_id(user_id, media_id, start_datetime);
        exists_some(&self.pool, query).await
    }
}
