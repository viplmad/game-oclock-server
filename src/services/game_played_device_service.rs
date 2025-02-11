use crate::errors::ApiErrors;
use crate::models::{DeviceDTO, GameDTO};
use crate::repository::{DeviceRepository, GamePlayedDeviceRepository, GameRepository};

use super::base::handle_get_list_result;
use super::{devices_service, games_service};

pub async fn get_device_played_games(
    repository: &GamePlayedDeviceRepository,
    device_repository: &DeviceRepository,
    user_id: &str,
    device_id: &str,
) -> Result<Vec<GameDTO>, ApiErrors> {
    devices_service::exists_device(device_repository, user_id, device_id).await?;

    let find_result = repository
        .find_all_games_with_played_device(user_id, device_id)
        .await;
    handle_get_list_result(find_result)
}

pub async fn get_game_played_devices(
    repository: &GamePlayedDeviceRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<DeviceDTO>, ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let find_result = repository
        .find_all_devices_with_played_game(user_id, game_id)
        .await;
    handle_get_list_result(find_result)
}
