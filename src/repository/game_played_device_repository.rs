use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_played_device_query;
use crate::entities::{Device, GameWithUserInfo};
use crate::errors::RepositoryError;

use super::helpers::fetch_all;

#[derive(Clone)]
pub struct GamePlayedDeviceRepository {
    pool: PgPool,
}

impl GamePlayedDeviceRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GamePlayedDeviceRepository {
    pub async fn find_all_games_with_played_device(
        &self,
        user_id: &Uuid,
        device_id: &Uuid,
    ) -> Result<Vec<GameWithUserInfo>, RepositoryError> {
        let query = game_played_device_query::select_all_games_by_device_id_order_by_date(
            user_id, device_id,
        );
        fetch_all(&self.pool, query).await
    }

    pub async fn find_all_devices_with_played_game(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<Device>, RepositoryError> {
        let query =
            game_played_device_query::select_all_devices_by_game_id_order_by_date(user_id, game_id);
        fetch_all(&self.pool, query).await
    }
}
