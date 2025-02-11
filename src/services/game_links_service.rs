use crate::entities::Link;
use crate::errors::ApiErrors;
use crate::models::{LinkDTO, Merge, NewLinkDTO};
use crate::repository::{GameLinkRepository, GameRepository};

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::games_service;

pub async fn get_game_links(
    repository: &GameLinkRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<LinkDTO>, ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let find_result = repository.find_all_by_game_id(user_id, game_id).await;
    handle_get_list_result::<Link, LinkDTO>(find_result)
}

pub async fn create_game_link(
    repository: &GameLinkRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
    link: NewLinkDTO,
) -> Result<(), ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let exists_result = repository.exists_by_id(user_id, game_id, &link.url).await;
    handle_already_exists_result::<LinkDTO>(exists_result)?;

    let merged_new = LinkDTO::merge_with_default(link);
    let link_to_create = Link::from(merged_new);
    let create_result = repository.create(user_id, game_id, &link_to_create).await;
    handle_action_result::<LinkDTO>(create_result)
}

pub async fn delete_game_link(
    repository: &GameLinkRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
    url: &str,
) -> Result<(), ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;
    exists_game_link(repository, user_id, game_id, url).await?;

    let delete_result = repository.delete_by_id(user_id, game_id, url).await;
    handle_action_result::<LinkDTO>(delete_result)
}

pub async fn exists_game_link(
    repository: &GameLinkRepository,
    user_id: &str,
    game_id: &str,
    url: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, game_id, url).await;
    handle_not_found_result::<LinkDTO>(exists_result)
}
