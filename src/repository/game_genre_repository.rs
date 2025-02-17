use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_genre_query;
use crate::entities::{GameGenre, GameWithUserInfo, Genre};
use crate::errors::RepositoryError;

use super::helpers::{execute, exists_id, fetch_all};

#[derive(Clone)]
pub struct GameGenreRepository {
    pool: PgPool,
}

impl GameGenreRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameGenreRepository {
    pub async fn find_all_games_with_genre(
        &self,
        user_id: &Uuid,
        genre_id: &Uuid,
    ) -> Result<Vec<GameWithUserInfo>, RepositoryError> {
        let query = game_genre_query::select_all_games_by_genre_id(user_id, genre_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn find_all_genres_with_game(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<Genre>, RepositoryError> {
        let query = game_genre_query::select_all_genres_by_game_id(user_id, game_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn create(&self, game_genre: &GameGenre) -> Result<(), RepositoryError> {
        let query = game_genre_query::insert(game_genre);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        genre_id: &Uuid,
    ) -> Result<(), RepositoryError> {
        let query = game_genre_query::delete_by_id(user_id, game_id, genre_id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        genre_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = game_genre_query::exists_by_id(user_id, game_id, genre_id);
        exists_id(&self.pool, query).await
    }
}
