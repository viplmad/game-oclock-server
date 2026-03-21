use chrono::{DateTime, Utc};
use sqlx::{PgPool, postgres::types::PgInterval};
use uuid::Uuid;

use super::query::media_session_query;
use crate::entities::{MediaSession, MediaSessionWithTime, PageResult, SessionSearch};
use crate::errors::{RepositoryError, SearchErrors};
use crate::models::DurationDef;

use super::helpers::{
    count_all_search, execute, execute_return_single, exists_some, fetch_all, fetch_all_search,
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
    pub async fn find_sum_time_by_media_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
    ) -> Result<DurationDef, RepositoryError> {
        let query = media_session_query::select_sum_time_by_user_id_and_media_id(user_id, media_id);
        execute_return_single::<_, PgInterval>(&self.pool, query)
            .await
            .map(|interval| DurationDef::from(interval))
    }

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
        search: SessionSearch,
    ) -> Result<PageResult<MediaSessionWithTime>, SearchErrors> {
        let search_query =
            media_session_query::select_all_by_user_id_and_media_id(user_id, media_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn count_all_by_media_id(
        &self,
        user_id: &Uuid,
        media_id: &Uuid,
        search: SessionSearch,
    ) -> Result<u64, SearchErrors> {
        let search_query =
            media_session_query::count_all_by_user_id_and_media_id(user_id, media_id, search)?;
        count_all_search(&self.pool, search_query).await
    }

    // For review
    pub async fn find_all_first_by_user_id_and_media_id_in(
        &self,
        user_id: &Uuid,
        media_ids: Vec<Uuid>,
    ) -> Result<Vec<MediaSessionWithTime>, RepositoryError> {
        if media_ids.is_empty() {
            return Ok(vec![]);
        }

        let query =
            media_session_query::select_all_first_by_user_id_and_media_id_in(user_id, media_ids);
        fetch_all(&self.pool, query).await
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
