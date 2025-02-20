use uuid::Uuid;

use crate::entities::{GameSearch, GameWithUserInfo};
use crate::errors::{ApiErrors, error_message_builder};
use crate::models::{GameDTO, GamePageResult, GameStatus, NewGameDTO, SearchDTO};
use crate::repository::GameRepository;

use super::GameAvailableService;
use super::helpers::{
    create_merged, handle_action_result, handle_already_exists_result,
    handle_get_list_paged_result, handle_get_list_result, handle_get_result,
    handle_not_found_result, handle_query_mapping, handle_result, handle_update_result,
    update_merged,
};

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
    pub async fn get_game(&self, user_id: &Uuid, game_id: &Uuid) -> Result<GameDTO, ApiErrors> {
        let find_result = self.repository.find_by_id(user_id, game_id).await;
        handle_get_result(find_result)
    }

    pub async fn search_games(
        &self,
        user_id: &Uuid,
        search: SearchDTO,
        quicksearch: Option<String>,
    ) -> Result<GamePageResult, ApiErrors> {
        let search = handle_query_mapping::<GameDTO, GameSearch>(search, quicksearch)?;
        let find_result = self.repository.search_all(user_id, search).await;
        handle_get_list_paged_result(find_result)
    }

    pub async fn create_game(
        &self,
        user_id: &Uuid,
        game: NewGameDTO,
    ) -> Result<GameDTO, ApiErrors> {
        check_game_rating(&game)?;
        // TODO check image is reachable
        let new_id = crate::uuid_utils::new_model_uuid();
        create_merged(
            game,
            async move || self.get_game(user_id, &new_id).await,
            async move |mut game_to_create: GameWithUserInfo| {
                let exists_result = self
                    .repository
                    .exists_by_title_and_edition(
                        user_id,
                        &game_to_create.title,
                        &game_to_create.edition,
                    )
                    .await;
                handle_already_exists_result::<GameDTO>(exists_result)?;

                game_to_create.user_id = user_id.clone();
                game_to_create.id = new_id.clone();
                game_to_create.added_datetime = crate::date_utils::now();
                game_to_create.updated_datetime = crate::date_utils::now();
                let create_result = self.repository.create(&game_to_create).await;
                handle_action_result::<GameDTO>(create_result)
            },
        )
        .await
    }

    pub async fn update_game(
        &self,
        game_available_service: &GameAvailableService,
        user_id: &Uuid,
        id: &Uuid,
        game: NewGameDTO,
    ) -> Result<(), ApiErrors> {
        check_game_rating(&game)?;
        let new_status = game.status.clone();

        update_merged(
            game,
            async move || self.get_game(user_id, id).await,
            async move |mut game_to_update: GameWithUserInfo| {
                let exists_result = self
                    .repository
                    .exists_by_title_and_edition_except_id(
                        user_id,
                        &game_to_update.title,
                        &game_to_update.edition,
                        id,
                    )
                    .await;
                handle_already_exists_result::<GameDTO>(exists_result)?;

                let old_status = GameStatus::try_from(game_to_update.status)
                    .expect("Status is not within valid range");
                if old_status != GameStatus::Wishlist
                    && new_status.is_some_and(|status| status == GameStatus::Wishlist)
                {
                    // Check only if change is from owned status to wishlist -> cannot change to wishlist if there are available locations
                    game_available_service
                        .exists_no_game_available(user_id, id)
                        .await?
                }

                game_to_update.user_id = user_id.clone();
                game_to_update.id = id.clone();
                game_to_update.updated_datetime = crate::date_utils::now();
                let update_result = self.repository.update(&game_to_update).await;
                handle_update_result::<GameDTO>(update_result)
            },
        )
        .await
    }

    pub async fn delete_game(&self, user_id: &Uuid, game_id: &Uuid) -> Result<(), ApiErrors> {
        // TODO Error if game is used -> use sql contraints
        let delete_result = self.repository.delete_by_id(user_id, game_id).await;
        handle_action_result::<GameDTO>(delete_result)
    }

    pub async fn exists_game(&self, user_id: &Uuid, game_id: &Uuid) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, game_id).await;
        handle_not_found_result::<GameDTO>(exists_result)
    }

    pub async fn get_game_base_game(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<GameDTO, ApiErrors> {
        let game = self.get_game(user_id, game_id).await?;
        let base_game_id = game.base_game_id.ok_or_else(|| {
            ApiErrors::InvalidParameter(error_message_builder::empty_param("Game base game"))
        })?;
        self.get_game(user_id, &base_game_id).await
    }

    pub async fn get_game_dlcs(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
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
        user_id: &Uuid,
        game_id: &Uuid,
        base_game_id: Option<Uuid>,
    ) -> Result<(), ApiErrors> {
        self.exists_game(user_id, game_id).await?;

        if let Some(id) = &base_game_id {
            // Set base game
            if game_id == id {
                return Err(ApiErrors::InvalidParameter(String::from(
                    "Game and base game cannot be the same",
                )));
            }

            let base_game = self.get_game(user_id, id).await?;
            if base_game.base_game_id.is_some() {
                return Err(ApiErrors::InvalidParameter(String::from(
                    "Base game cannot be itself a dlc of another game",
                )));
            }

            let has_dlcs = self.has_game_dlcs(user_id, game_id).await?;
            if has_dlcs {
                return Err(ApiErrors::InvalidParameter(String::from(
                    "Game cannot have dlcs",
                )));
            }
        }
        // else unset base game

        let update_result = self
            .repository
            .update_base_game_id(user_id, game_id, base_game_id)
            .await;
        handle_action_result::<GameDTO>(update_result)
    }

    async fn has_game_dlcs(&self, user_id: &Uuid, game_id: &Uuid) -> Result<bool, ApiErrors> {
        let exists_result = self
            .repository
            .exists_any_by_base_game_id(user_id, game_id)
            .await;
        handle_result::<_, GameDTO>(exists_result)
    }
}

fn check_game_rating(game: &NewGameDTO) -> Result<(), ApiErrors> {
    if game.rating.is_some_and(|rating| rating > 10) {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Rating must be between 0 and 10, both inclusive",
        )));
    }
    Ok(())
}
