use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_finish_query;
use crate::entities::GameFinish;
use crate::errors::RepositoryError;

use super::helpers::{execute, execute_return_single, exists_id, fetch_all};

#[derive(Clone)]
pub struct GameFinishRepository {
    pool: PgPool,
}

impl GameFinishRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameFinishRepository {
    pub async fn find_first_by_game_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Option<NaiveDate>, RepositoryError> {
        let query = game_finish_query::select_min_date_by_user_id_and_game_id(user_id, game_id);
        execute_return_single(&self.pool, query).await
    }

    pub async fn find_all_by_game_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<GameFinish>, RepositoryError> {
        let query = game_finish_query::select_all_by_user_id_and_game_id(user_id, game_id);
        fetch_all(&self.pool, query).await
    }

    // For review
    pub async fn find_all_first_by_user_id_and_game_id_in(
        &self,
        user_id: &Uuid,
        game_ids: Vec<Uuid>,
    ) -> Result<Vec<GameFinish>, RepositoryError> {
        if game_ids.is_empty() {
            return Ok(vec![]);
        }

        let query =
            game_finish_query::select_all_first_by_user_id_and_game_id_in(user_id, game_ids);
        fetch_all(&self.pool, query).await
    }

    pub async fn create(&self, game_finish: &GameFinish) -> Result<(), RepositoryError> {
        let query = game_finish_query::insert(game_finish);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        date: NaiveDate,
    ) -> Result<(), RepositoryError> {
        let query = game_finish_query::delete_by_id(user_id, game_id, date);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        date: NaiveDate,
    ) -> Result<bool, RepositoryError> {
        let query = game_finish_query::exists_by_id(user_id, game_id, date);
        exists_id(&self.pool, query).await
    }
}
