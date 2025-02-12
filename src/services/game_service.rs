use crate::entities::GameSearch;
use crate::errors::{error_message_builder, ApiErrors};
use crate::models::{GameDTO, GamePageResult, GameStatus, NewGameDTO, SearchDTO};
use crate::repository::GameRepository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_list_result, handle_get_result,
    handle_not_found_result, handle_query_mapping, handle_update_result, update_merged,
};
use super::GameAvailableService;

#[derive(Clone)]
pub struct GameService {
    repository: GameRepository,
}

impl GameService {
    pub fn with(repository: GameRepository) -> Self {
        Self { repository }
    }
}

impl GameService {
    pub async fn get_game(&self, user_id: &str, game_id: &str) -> Result<GameDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, game_id).await;
        handle_get_result(find_result)
    }

    pub async fn search_games(
        &self,
        user_id: &str,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<GamePageResult, ApiErrors> {
        let search = handle_query_mapping::<GameDTO, GameSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn create_game(&self, user_id: &str, game: NewGameDTO) -> Result<GameDTO, ApiErrors> {
        // TODO check image is reachable
        create_merged(
            game,
            async move |created_game_id| self.get_game(user_id, &created_game_id).await,
            async move |game_to_create| {
                let exists_result = self
                    .repository
                    .exists_with_unique(user_id, &game_to_create)
                    .await;
                handle_already_exists_result::<GameDTO>(exists_result)?;

                let create_result = self.repository.create(user_id, &game_to_create).await;
                handle_create_result::<String, GameDTO>(create_result)
            },
        )
        .await
    }

    pub async fn update_game(
        &self,
        game_available_service: &GameAvailableService,
        user_id: &str,
        game_id: &str,
        game: NewGameDTO,
    ) -> Result<(), ApiErrors> {
        let new_status = game.status.clone();

        update_merged(
            game,
            async move || self.get_game(user_id, game_id).await,
            async move |game_to_update| {
                let exists_result = self
                    .repository
                    .exists_with_unique_except_id(user_id, &game_to_update, game_id)
                    .await;
                handle_already_exists_result::<GameDTO>(exists_result)?;

                let old_status = GameStatus::try_from(game_to_update.status)
                    .expect("Status was not within valid range");
                if old_status != GameStatus::Wishlist
                    && new_status.is_some_and(|status| status == GameStatus::Wishlist)
                {
                    // Check only if change is from owned status to wishlist -> cannot change to wishlist if there are available locations
                    game_available_service
                        .exists_no_game_available(user_id, game_id)
                        .await?
                }

                let update_result = self
                    .repository
                    .update_by_id(user_id, game_id, &game_to_update)
                    .await;
                handle_update_result::<GameDTO>(update_result)
            },
        )
        .await
    }

    pub async fn delete_game(&self, user_id: &str, game_id: &str) -> Result<(), ApiErrors> {
        // TODO Error if game is used -> use sql contraints
        let delete_result = self.repository.delete_by_id(user_id, game_id).await;
        handle_action_result::<GameDTO>(delete_result)
    }

    pub async fn exists_game(&self, user_id: &str, game_id: &str) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, game_id).await;
        handle_not_found_result::<GameDTO>(exists_result)
    }

    pub async fn get_game_base_game(
        &self,
        user_id: &str,
        game_id: &str,
    ) -> Result<GameDTO, ApiErrors> {
        let game = self.get_game(user_id, game_id).await?;
        let base_game_id = game.base_game_id.ok_or_else(|| {
            ApiErrors::InvalidParameter(error_message_builder::empty_param("Game base game"))
        })?;
        self.get_game(user_id, &base_game_id).await
    }

    pub async fn get_game_dlcs(
        &self,
        user_id: &str,
        game_id: &str,
    ) -> Result<Vec<GameDTO>, ApiErrors> {
        self.exists_game(user_id, game_id).await?;

        let find_result = self
            .repository
            .find_all_by_base_game_id(user_id, game_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn set_game_base_game(
        &self,
        user_id: &str,
        game_id: &str,
        base_game_id: Option<String>,
    ) -> Result<(), ApiErrors> {
        self.exists_game(user_id, game_id).await?;

        // TODO
        // check dlc is not already in other base_game
        // check game to set as base_game is not a dlc

        if let Some(game_id) = &base_game_id {
            self.exists_game(user_id, game_id).await?;
        }

        let update_result = self
            .repository
            .update_base_game_id(user_id, game_id, base_game_id)
            .await;
        handle_action_result::<GameDTO>(update_result)
    }
}
