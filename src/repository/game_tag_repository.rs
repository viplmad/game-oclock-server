use sqlx::PgPool;

use crate::entities::{Game, Tag};
use crate::errors::RepositoryError;
use crate::query::game_tag_query;

use super::base::{execute, exists_id, fetch_all};

pub async fn find_all_games_with_tag(
    pool: &PgPool,
    user_id: i32,
    tag_id: i32,
) -> Result<Vec<Game>, RepositoryError> {
    let query = game_tag_query::select_all_games_by_tag_id(user_id, tag_id);
    fetch_all(pool, query).await
}

pub async fn find_all_tags_with_game(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
) -> Result<Vec<Tag>, RepositoryError> {
    let query = game_tag_query::select_all_tags_by_game_id(user_id, game_id);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    tag_id: i32,
) -> Result<(), RepositoryError> {
    let query = game_tag_query::insert(user_id, game_id, tag_id);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    tag_id: i32,
) -> Result<(), RepositoryError> {
    let query = game_tag_query::delete_by_id(user_id, game_id, tag_id);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    tag_id: i32,
) -> Result<bool, RepositoryError> {
    let query = game_tag_query::exists_by_id(user_id, game_id, tag_id);
    exists_id(pool, query).await
}
