use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_log_query;
use crate::entities::{GameSearch, GameWithLog, PageResult};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{fetch_all, fetch_all_search};

#[derive(Clone)]
pub struct GameWithLogRepository {
    pool: PgPool,
}

impl GameWithLogRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameWithLogRepository {
    pub async fn search_first_by_start_datetime_between(
        &self,
        user_id: &Uuid,
        start_datetime: Option<DateTime<Utc>>,
        end_datetime: Option<DateTime<Utc>>,
        search: GameSearch,
    ) -> Result<PageResult<GameWithLog>, SearchErrors> {
        let search_query = game_log_query::select_all_first_game_with_log_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(user_id, start_datetime, end_datetime, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn search_last_by_start_datetime_between(
        &self,
        user_id: &Uuid,
        start_datetime: Option<DateTime<Utc>>,
        end_datetime: Option<DateTime<Utc>>,
        search: GameSearch,
    ) -> Result<PageResult<GameWithLog>, SearchErrors> {
        let search_query = game_log_query::select_all_last_game_with_log_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(user_id, start_datetime, end_datetime, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn find_all_by_start_datetime_between(
        &self,
        user_id: &Uuid,
        start_datetime: DateTime<Utc>,
        end_datetime: DateTime<Utc>,
    ) -> Result<Vec<GameWithLog>, RepositoryError> {
        let query = game_log_query::select_all_games_log_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(user_id, start_datetime, end_datetime);
        fetch_all(&self.pool, query).await
    }
}
