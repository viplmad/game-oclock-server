use crate::errors::ApiErrors;
use crate::models::{GameDTO, GameTagDTO, TagDTO};
use crate::repository::GameTagRepository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::{GameService, TagService};

#[derive(Clone)]
pub struct GameTagService {
    repository: GameTagRepository,
    game_service: GameService,
    tag_service: TagService,
}

impl GameTagService {
    pub fn with(
        repository: GameTagRepository,
        game_service: GameService,
        tag_service: TagService,
    ) -> Self {
        Self {
            repository,
            game_service,
            tag_service,
        }
    }
}

impl GameTagService {
    pub async fn get_tag_games(
        &self,
        user_id: &str,
        tag_id: &str,
    ) -> Result<Vec<GameDTO>, ApiErrors> {
        self.tag_service.exists_tag(user_id, tag_id).await?;

        let find_result = self
            .repository
            .find_all_games_with_tag(user_id, tag_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn get_game_tags(
        &self,
        user_id: &str,
        game_id: &str,
    ) -> Result<Vec<TagDTO>, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self
            .repository
            .find_all_tags_with_game(user_id, game_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn create_game_tag(
        &self,
        user_id: &str,
        game_id: &str,
        tag_id: &str,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;
        self.tag_service.exists_tag(user_id, tag_id).await?;

        let exists_result = self.repository.exists_by_id(user_id, game_id, tag_id).await;
        handle_already_exists_result::<GameTagDTO>(exists_result)?;

        let create_result = self.repository.create(user_id, game_id, tag_id).await;
        handle_action_result::<GameTagDTO>(create_result)
    }

    pub async fn delete_game_tag(
        &self,
        user_id: &str,
        game_id: &str,
        tag_id: &str,
    ) -> Result<(), ApiErrors> {
        self.exists_game_tag(user_id, game_id, tag_id).await?;

        let delete_result = self.repository.delete_by_id(user_id, game_id, tag_id).await;
        handle_action_result::<GameTagDTO>(delete_result)
    }

    pub async fn exists_game_tag(
        &self,
        user_id: &str,
        game_id: &str,
        tag_id: &str,
    ) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, game_id, tag_id).await;
        handle_not_found_result::<GameTagDTO>(exists_result)
    }
}
