use sqlx::PgPool;

use crate::entities::Link;
use crate::errors::ApiErrors;
use crate::models::{LinkDTO, Merge, NewLinkDTO};
use crate::repository::game_link_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::games_service;

pub async fn get_game_links(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<LinkDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_link_repository::find_all_by_game_id(pool, user_id, game_id).await;
    handle_get_list_result::<Link, LinkDTO>(find_result)
}

pub async fn create_game_link(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    link: NewLinkDTO,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let exists_result = game_link_repository::exists_by_id(pool, user_id, game_id, &link.url).await;
    handle_already_exists_result::<LinkDTO>(exists_result)?;

    let merged_new = LinkDTO::merge_with_default(link);
    let link_to_create = Link::from(merged_new);
    let create_result = game_link_repository::create(pool, user_id, game_id, &link_to_create).await;
    handle_action_result::<LinkDTO>(create_result)
}

pub async fn delete_game_link(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    url: &str,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;
    exists_game_link(pool, user_id, game_id, url).await?;

    let delete_result = game_link_repository::delete_by_id(pool, user_id, game_id, url).await;
    handle_action_result::<LinkDTO>(delete_result)
}

pub async fn exists_game_link(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    url: &str,
) -> Result<(), ApiErrors> {
    let exists_result = game_link_repository::exists_by_id(pool, user_id, game_id, url).await;
    handle_not_found_result::<LinkDTO>(exists_result)
}
