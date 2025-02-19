use chrono::NaiveDate;
use uuid::Uuid;

use crate::entities::GameFinish;
use crate::errors::ApiErrors;
use crate::models::{FinishDTO, GameStatus, Merge, NewFinishDTO};
use crate::repository::GameFinishRepository;

use super::helpers::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_get_result_raw, handle_not_found_result, handle_result,
};
use super::{DeviceService, GameService};

#[derive(Clone)]
pub struct GameFinishService {
    repository: GameFinishRepository,
    game_service: GameService,
    device_service: DeviceService,
}

impl GameFinishService {
    pub fn with(
        repository: GameFinishRepository,
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

impl GameFinishService {
    pub async fn get_first_game_finish(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<NaiveDate, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self
            .repository
            .find_first_by_game_id(user_id, game_id)
            .await;
        handle_get_result_raw::<NaiveDate, FinishDTO>(find_result)
    }

    pub async fn get_game_finishes(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<FinishDTO>, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self.repository.find_all_by_game_id(user_id, game_id).await;
        handle_get_list_result::<GameFinish, FinishDTO>(find_result)
    }

    // For review
    pub(super) async fn find_first_game_finishes_by_games(
        &self,
        user_id: &Uuid,
        game_ids: Vec<Uuid>,
    ) -> Result<Vec<GameFinish>, ApiErrors> {
        let find_result = self
            .repository
            .find_all_first_by_user_id_and_game_id_in(user_id, game_ids)
            .await;
        handle_result::<Vec<GameFinish>, FinishDTO>(find_result)
    }

    pub async fn create_game_finish(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        finish: NewFinishDTO,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;
        if let Some(device_id) = finish.device_id {
            self.device_service
                .exists_device(user_id, &device_id)
                .await?;
        }
        if finish.status != GameStatus::Completed || finish.status != GameStatus::Retired {
            return Err(ApiErrors::InvalidParameter(String::from(
                "Status must be Completed or Retired",
            )));
        }

        let exists_result = self
            .repository
            .exists_by_id(user_id, game_id, finish.date)
            .await;
        handle_already_exists_result::<FinishDTO>(exists_result)?;

        let merged_new = FinishDTO::merge_with_default(finish);
        let mut finish_to_create = GameFinish::from(merged_new);
        finish_to_create.user_id = user_id.clone();
        finish_to_create.game_id = game_id.clone();
        let create_result = self.repository.create(&finish_to_create).await;
        handle_action_result::<FinishDTO>(create_result)
    }

    pub async fn delete_game_finish(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        date: NaiveDate,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;
        self.exists_game_finish(user_id, game_id, date).await?;

        let delete_result = self.repository.delete_by_id(user_id, game_id, date).await;
        handle_action_result::<FinishDTO>(delete_result)
    }

    pub async fn exists_game_finish(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        date: NaiveDate,
    ) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, game_id, date).await;
        handle_not_found_result::<FinishDTO>(exists_result)
    }
}
