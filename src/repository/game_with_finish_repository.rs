use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_finish_query;
use crate::entities::{GameSearch, GameWithFinish, PageResult};
use crate::errors::{RepositoryError, SearchErrors};

use super::helpers::{fetch_all, fetch_all_search};

#[derive(Clone)]
pub struct GameWithFinishRepository {
    pool: PgPool,
}

impl GameWithFinishRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameWithFinishRepository {
    pub async fn search_first_by_date_between(
        &self,
        user_id: &Uuid,
        start_datetime: Option<DateTime<Utc>>,
        end_datetime: Option<DateTime<Utc>>,
        search: GameSearch,
    ) -> Result<PageResult<GameWithFinish>, SearchErrors> {
        let search_query =
    game_finish_query::select_all_first_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_asc(
        user_id, start_datetime, end_datetime, search,
    )?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn search_last_by_date_between(
        &self,
        user_id: &Uuid,
        start_datetime: Option<DateTime<Utc>>,
        end_datetime: Option<DateTime<Utc>>,
        search: GameSearch,
    ) -> Result<PageResult<GameWithFinish>, SearchErrors> {
        let search_query =
        game_finish_query::select_all_last_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_desc(
            user_id, start_datetime, end_datetime, search,
        )?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn find_all_by_date_between(
        &self,
        user_id: &Uuid,
        start_datetime: DateTime<Utc>,
        end_datetime: DateTime<Utc>,
    ) -> Result<Vec<GameWithFinish>, RepositoryError> {
        let query =
            game_finish_query::select_all_games_finish_by_date_gte_and_date_lte_order_by_date_desc(
                user_id,
                start_datetime,
                end_datetime,
            );
        fetch_all(&self.pool, query).await
    }
}
