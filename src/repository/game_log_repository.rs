use chrono::NaiveDateTime;
use sqlx::{postgres::types::PgInterval, PgPool};
use uuid::Uuid;

use super::query::game_log_query;
use crate::entities::{GameLog, GameLogWithTime};
use crate::errors::RepositoryError;
use crate::models::DurationDef;

use super::helpers::{
    begin_transaction, commit_transaction, execute, execute_return_single, exists_some, fetch_all,
};

#[derive(Clone)]
pub struct GameLogRepository {
    pool: PgPool,
}

impl GameLogRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameLogRepository {
    pub async fn find_sum_time_by_game_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<DurationDef, RepositoryError> {
        let query = game_log_query::select_sum_time_by_user_id_and_game_id(user_id, game_id);
        execute_return_single::<_, PgInterval>(&self.pool, query)
            .await
            .map(|interval| DurationDef::from(interval))
    }

    pub async fn find_all_by_game_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<GameLogWithTime>, RepositoryError> {
        let query = game_log_query::select_all_by_user_id_and_game_id(user_id, game_id);
        fetch_all(&self.pool, query).await
    }

    // For review
    pub async fn find_all_first_by_user_id_and_game_id_in(
        &self,
        user_id: &Uuid,
        game_ids: Vec<Uuid>,
    ) -> Result<Vec<GameLogWithTime>, RepositoryError> {
        if game_ids.is_empty() {
            return Ok(vec![]);
        }

        let query = game_log_query::select_all_first_by_user_id_and_game_id_in(user_id, game_ids);
        fetch_all(&self.pool, query).await
    }

    pub async fn create_multiple(&self, game_logs: Vec<GameLog>) -> Result<(), RepositoryError> {
        let mut transaction = begin_transaction(&self.pool).await?;

        for game_log in game_logs.into_iter() {
            let query = game_log_query::insert(&game_log);
            execute(&mut *transaction, query).await?;
        }

        commit_transaction(transaction).await?;

        Ok(())
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        start_datetime: NaiveDateTime,
    ) -> Result<(), RepositoryError> {
        let query = game_log_query::delete_by_id(user_id, game_id, start_datetime);
        execute(&self.pool, query).await
    }

    pub async fn exists_gap(
        &self,
        user_id: &Uuid,
        start_datetime: NaiveDateTime,
        end_datetime: NaiveDateTime,
    ) -> Result<bool, RepositoryError> {
        let query = game_log_query::exists_by_start_datetime_lt_or_end_datetime_gt(
            user_id,
            end_datetime,
            start_datetime,
        );
        exists_some(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        start_datetime: NaiveDateTime,
    ) -> Result<bool, RepositoryError> {
        let query = game_log_query::exists_by_id(user_id, game_id, start_datetime);
        exists_some(&self.pool, query).await
    }
}
