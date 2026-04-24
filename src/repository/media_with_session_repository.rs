use chrono::{DateTime, FixedOffset};
use sqlx::PgPool;
use uuid::Uuid;

use super::query::media_session_query;
use crate::entities::{MediaListSearch, MediaWithStateWithSession, PageResult};
use crate::errors::SearchErrors;

use super::helpers::fetch_all_search;

#[derive(Clone)]
pub struct MediaWithSessionRepository {
    pool: PgPool,
}

impl MediaWithSessionRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MediaWithSessionRepository {
    pub async fn search_first_by_start_datetime_between(
        &self,
        user_id: &Uuid,
        start_datetime: Option<DateTime<FixedOffset>>,
        end_datetime: Option<DateTime<FixedOffset>>,
        search: MediaListSearch,
    ) -> Result<PageResult<MediaWithStateWithSession>, SearchErrors> {
        let search_query = media_session_query::select_all_first_media_with_session_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(user_id, start_datetime, end_datetime, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn search_last_by_start_datetime_between(
        &self,
        user_id: &Uuid,
        start_datetime: Option<DateTime<FixedOffset>>,
        end_datetime: Option<DateTime<FixedOffset>>,
        search: MediaListSearch,
    ) -> Result<PageResult<MediaWithStateWithSession>, SearchErrors> {
        let search_query = media_session_query::select_all_last_media_with_session_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(user_id, start_datetime, end_datetime, search)?;
        fetch_all_search(&self.pool, search_query).await
    }
}
