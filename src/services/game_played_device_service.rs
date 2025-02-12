use crate::errors::ApiErrors;
use crate::models::{DeviceDTO, GameDTO};
use crate::repository::GamePlayedDeviceRepository;

use super::base::handle_get_list_result;
use super::{DeviceService, GameService};

#[derive(Clone)]
pub struct GamePlayedDeviceService {
    repository: GamePlayedDeviceRepository,
    game_service: GameService,
    device_service: DeviceService,
}

impl GamePlayedDeviceService {
    pub fn with(
        repository: GamePlayedDeviceRepository,
        game_service: GameService,
        device_service: DeviceService,
    ) -> Self {
        Self {
            repository,
            game_service,
            device_service,
        }
    }
}

impl GamePlayedDeviceService {
    pub async fn get_device_played_games(
        &self,
        user_id: &str,
        device_id: &str,
    ) -> Result<Vec<GameDTO>, ApiErrors> {
        self.device_service
            .exists_device(user_id, device_id)
            .await?;

        let find_result = self
            .repository
            .find_all_games_with_played_device(user_id, device_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn get_game_played_devices(
        &self,
        user_id: &str,
        game_id: &str,
    ) -> Result<Vec<DeviceDTO>, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self
            .repository
            .find_all_devices_with_played_game(user_id, game_id)
            .await;
        handle_get_list_result(find_result)
    }
}
