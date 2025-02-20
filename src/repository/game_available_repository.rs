use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_available_query;
use crate::entities::{GameAvailable, GameWithUserInfoWithDate, LocationWithDate};
use crate::errors::RepositoryError;

use super::helpers::{execute, exists_some, fetch_all};

#[derive(Clone)]
pub struct GameAvailableRepository {
    pool: PgPool,
}

impl GameAvailableRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameAvailableRepository {
    pub async fn find_all_games_with_location(
        &self,
        user_id: &Uuid,
        location_id: &Uuid,
    ) -> Result<Vec<GameWithUserInfoWithDate>, RepositoryError> {
        let query = game_available_query::select_all_games_by_location_id_order_by_date(
            user_id,
            location_id,
        );
        fetch_all(&self.pool, query).await
    }

    pub async fn find_all_locations_with_game(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<LocationWithDate>, RepositoryError> {
        let query =
            game_available_query::select_all_locations_by_game_id_order_by_date(user_id, game_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn create(&self, game_available: &GameAvailable) -> Result<(), RepositoryError> {
        let query = game_available_query::insert(game_available);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        location_id: &Uuid,
    ) -> Result<(), RepositoryError> {
        let query = game_available_query::delete_by_id(user_id, game_id, location_id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        location_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = game_available_query::exists_by_id(user_id, game_id, location_id);
        exists_some(&self.pool, query).await
    }

    pub async fn exists_locations_with_game(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = game_available_query::exists_locations_by_game_id(user_id, game_id);
        exists_some(&self.pool, query).await
    }
}
