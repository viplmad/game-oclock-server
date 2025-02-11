use crate::errors::ApiErrors;
use crate::models::{GameDTO, GameTagDTO, TagDTO};
use crate::repository::{GameRepository, GameTagRepository, TagRepository};

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::{games_service, tags_service};

pub async fn get_tag_games(
    repository: &GameTagRepository,
    tag_repository: &TagRepository,
    user_id: &str,
    tag_id: &str,
) -> Result<Vec<GameDTO>, ApiErrors> {
    tags_service::exists_tag(tag_repository, user_id, tag_id).await?;

    let find_result = repository.find_all_games_with_tag(user_id, tag_id).await;
    handle_get_list_result(find_result)
}

pub async fn get_game_tags(
    repository: &GameTagRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<TagDTO>, ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let find_result = repository.find_all_tags_with_game(user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn create_game_tag(
    repository: &GameTagRepository,
    game_repository: &GameRepository,
    tag_repository: &TagRepository,
    user_id: &str,
    game_id: &str,
    tag_id: &str,
) -> Result<(), ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;
    tags_service::exists_tag(tag_repository, user_id, tag_id).await?;

    let exists_result = repository.exists_by_id(user_id, game_id, tag_id).await;
    handle_already_exists_result::<GameTagDTO>(exists_result)?;

    let create_result = repository.create(user_id, game_id, tag_id).await;
    handle_action_result::<GameTagDTO>(create_result)
}

pub async fn delete_game_tag(
    repository: &GameTagRepository,
    user_id: &str,
    game_id: &str,
    tag_id: &str,
) -> Result<(), ApiErrors> {
    exists_game_tag(repository, user_id, game_id, tag_id).await?;

    let delete_result = repository.delete_by_id(user_id, game_id, tag_id).await;
    handle_action_result::<GameTagDTO>(delete_result)
}

pub async fn exists_game_tag(
    repository: &GameTagRepository,
    user_id: &str,
    game_id: &str,
    tag_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, game_id, tag_id).await;
    handle_not_found_result::<GameTagDTO>(exists_result)
}
