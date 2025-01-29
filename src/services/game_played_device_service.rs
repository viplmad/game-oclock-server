use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{DeviceDTO, GameDTO};
use crate::repository::game_played_device_repository;

use super::base::handle_get_list_result;
use super::{devices_service, games_service};

pub async fn get_device_played_games(
    pool: &PgPool,
    user_id: &str,
    device_id: &str,
) -> Result<Vec<GameDTO>, ApiErrors> {
    devices_service::exists_device(pool, user_id, device_id).await?;

    let find_result =
        game_played_device_repository::find_all_games_with_played_device(pool, user_id, device_id)
            .await;
    handle_get_list_result(find_result)
}

pub async fn get_game_played_devices(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<DeviceDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result =
        game_played_device_repository::find_all_devices_with_played_game(pool, user_id, game_id)
            .await;
    handle_get_list_result(find_result)
}
