use sqlx::PgPool;

use crate::entities::GameLink;
use crate::errors::RepositoryError;
use crate::query::game_link_query;

use super::base::{execute, exists_id, fetch_all};

pub async fn find_all_by_game_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<GameLink>, RepositoryError> {
    let query = game_link_query::select_all_by_user_id_and_game_id(user_id, game_id);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    link: &GameLink,
) -> Result<(), RepositoryError> {
    let query = game_link_query::insert(user_id, game_id, link);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    url: &str,
) -> Result<(), RepositoryError> {
    let query = game_link_query::delete_by_id(user_id, game_id, url);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    url: &str,
) -> Result<bool, RepositoryError> {
    let query = game_link_query::exists_by_id(user_id, game_id, url);
    exists_id(pool, query).await
}
