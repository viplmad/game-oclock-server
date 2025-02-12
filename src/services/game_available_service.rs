use chrono::NaiveDate;

use crate::errors::ApiErrors;
use crate::models::{GameAvailableDTO, GameStatus, LocationAvailableDTO, NewGameDTO};
use crate::repository::GameAvailableRepository;

use super::helpers::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result, handle_result,
};
use super::{GameService, LocationService};

#[derive(Clone)]
pub struct GameAvailableService {
    repository: GameAvailableRepository,
    game_service: GameService,
    location_service: LocationService,
}

impl GameAvailableService {
    pub fn with(
        repository: GameAvailableRepository,
        game_service: GameService,
        location_service: LocationService,
    ) -> Self {
        Self {
            repository,
            game_service,
            location_service,
        }
    }
}

impl GameAvailableService {
    pub async fn get_location_games(
        &self,
        user_id: &str,
        location_id: &str,
    ) -> Result<Vec<GameAvailableDTO>, ApiErrors> {
        self.location_service
            .exists_location(user_id, location_id)
            .await?;

        let find_result = self
            .repository
            .find_all_games_with_location(user_id, location_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn get_game_locations(
        &self,
        user_id: &str,
        game_id: &str,
    ) -> Result<Vec<LocationAvailableDTO>, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self
            .repository
            .find_all_locations_with_game(user_id, game_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn create_game_available(
        &self,
        user_id: &str,
        game_id: &str,
        location_id: &str,
        available_date: NaiveDate,
    ) -> Result<(), ApiErrors> {
        let game = self.game_service.get_game(user_id, game_id).await?;
        self.location_service
            .exists_location(user_id, location_id)
            .await?;

        let exists_result = self
            .repository
            .exists_by_id(user_id, game_id, location_id)
            .await;
        handle_already_exists_result::<GameAvailableDTO>(exists_result)?;

        if game.status == GameStatus::Wishlist {
            // Change status as it has become available
            self.game_service
                .update_game(
                    &self,
                    user_id,
                    game_id,
                    NewGameDTO {
                        status: Some(GameStatus::NextUp),
                        title: None,
                        edition: None,
                        release_date: None,
                        rating: None,
                        notes: None,
                    },
                )
                .await?
        }

        let create_result = self
            .repository
            .create(user_id, game_id, location_id, available_date)
            .await;
        handle_action_result::<GameAvailableDTO>(create_result)
    }

    pub async fn delete_game_available(
        &self,
        user_id: &str,
        game_id: &str,
        location_id: &str,
    ) -> Result<(), ApiErrors> {
        self.exists_game_available(user_id, game_id, location_id)
            .await?;

        let delete_result = self
            .repository
            .delete_by_id(user_id, game_id, location_id)
            .await;
        handle_action_result::<GameAvailableDTO>(delete_result)
    }

    pub async fn exists_game_available(
        &self,
        user_id: &str,
        game_id: &str,
        location_id: &str,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id(user_id, game_id, location_id)
            .await;
        handle_not_found_result::<GameAvailableDTO>(exists_result)
    }

    pub(super) async fn exists_no_game_available(
        &self,
        user_id: &str,
        game_id: &str,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_locations_with_game(user_id, game_id)
            .await;
        let exists = handle_result::<bool, GameAvailableDTO>(exists_result)?;
        match exists {
            true => Err(ApiErrors::AlreadyExists(String::from(
                "Game has locations available",
            ))),
            false => Ok(()),
        }
    }
}
