use sqlx::PgPool;

use crate::errors::ApiErrors;
use crate::models::{GameDTO, GameTag, TagDTO};
use crate::repository::game_tag_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::{games_service, tags_service};

pub async fn get_tag_games(
    pool: &PgPool,
    user_id: i32,
    tag_id: i32,
) -> Result<Vec<GameDTO>, ApiErrors> {
    tags_service::exists_tag(pool, user_id, tag_id).await?;

    let find_result = game_tag_repository::find_all_games_with_tag(pool, user_id, tag_id).await;
    handle_get_list_result(find_result)
}

pub async fn get_game_tags(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<TagDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_tag_repository::find_all_tags_with_game(pool, user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn create_game_tag(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    tag_id: i32,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;
    tags_service::exists_tag(pool, user_id, tag_id).await?;

    let exists_result = game_tag_repository::exists_by_id(pool, user_id, game_id, tag_id).await;
    handle_already_exists_result::<GameTag>(exists_result)?;

    let create_result = game_tag_repository::create(pool, user_id, game_id, tag_id).await;
    handle_action_result::<GameTag>(create_result)
}

pub async fn delete_game_tag(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    tag_id: i32,
) -> Result<(), ApiErrors> {
    exists_game_tag(pool, user_id, game_id, tag_id).await?;

    let delete_result = game_tag_repository::delete_by_id(pool, user_id, game_id, tag_id).await;
    handle_action_result::<GameTag>(delete_result)
}

pub async fn exists_game_tag(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    tag_id: i32,
) -> Result<(), ApiErrors> {
    let exists_result = game_tag_repository::exists_by_id(pool, user_id, game_id, tag_id).await;
    handle_not_found_result::<GameTag>(exists_result)
}
