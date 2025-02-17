use uuid::Uuid;

use crate::entities::GameLink;
use crate::errors::ApiErrors;
use crate::models::{LinkDTO, Merge, NewLinkDTO};
use crate::repository::GameLinkRepository;

use super::helpers::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::GameService;

#[derive(Clone)]
pub struct GameLinkService {
    repository: GameLinkRepository,
    game_service: GameService,
}

impl GameLinkService {
    pub fn with(repository: GameLinkRepository, game_service: GameService) -> Self {
        Self {
            repository,
            game_service,
        }
    }
}

impl GameLinkService {
    pub async fn get_game_links(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<LinkDTO>, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self.repository.find_all_by_game_id(user_id, game_id).await;
        handle_get_list_result::<GameLink, LinkDTO>(find_result)
    }

    pub async fn create_game_link(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        link: NewLinkDTO,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let exists_result = self
            .repository
            .exists_by_id(user_id, game_id, &link.url)
            .await;
        handle_already_exists_result::<LinkDTO>(exists_result)?;

        let merged_new = LinkDTO::merge_with_default(link);
        let mut link_to_create = GameLink::from(merged_new);
        link_to_create.user_id = user_id.clone();
        link_to_create.game_id = game_id.clone();
        let create_result = self.repository.create(&link_to_create).await;
        handle_action_result::<LinkDTO>(create_result)
    }

    pub async fn delete_game_link(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        url: &str,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;
        self.exists_game_link(user_id, game_id, url).await?;

        let delete_result = self.repository.delete_by_id(user_id, game_id, url).await;
        handle_action_result::<LinkDTO>(delete_result)
    }

    pub async fn exists_game_link(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        url: &str,
    ) -> Result<(), ApiErrors> {
        let exists_result = self.repository.exists_by_id(user_id, game_id, url).await;
        handle_not_found_result::<LinkDTO>(exists_result)
    }
}
