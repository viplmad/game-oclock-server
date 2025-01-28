use sqlx::PgPool;

use crate::entities::{Game, Genre};
use crate::errors::RepositoryError;
use crate::query::game_genre_query;

use super::base::{execute, exists_id, fetch_all};

pub async fn find_all_games_with_genre(
    pool: &PgPool,
    user_id: &str,
    genre_id: &str,
) -> Result<Vec<Game>, RepositoryError> {
    let query = game_genre_query::select_all_games_by_genre_id(user_id, genre_id);
    fetch_all(pool, query).await
}

pub async fn find_all_genres_with_game(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<Genre>, RepositoryError> {
    let query = game_genre_query::select_all_genres_by_game_id(user_id, game_id);
    fetch_all(pool, query).await
}

pub async fn create(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    genre_id: &str,
) -> Result<(), RepositoryError> {
    let query = game_genre_query::insert(user_id, game_id, genre_id);
    execute(pool, query).await
}

pub async fn delete_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    genre_id: &str,
) -> Result<(), RepositoryError> {
    let query = game_genre_query::delete_by_id(user_id, game_id, genre_id);
    execute(pool, query).await
}

pub async fn exists_by_id(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    genre_id: &str,
) -> Result<bool, RepositoryError> {
    let query = game_genre_query::exists_by_id(user_id, game_id, genre_id);
    exists_id(pool, query).await
}
