use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_tag_query;
use crate::entities::{GameTag, GameWithUserInfo, Tag};
use crate::errors::RepositoryError;

use super::helpers::{execute, exists_id, fetch_all};

#[derive(Clone)]
pub struct GameTagRepository {
    pool: PgPool,
}

impl GameTagRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameTagRepository {
    pub async fn find_all_games_with_tag(
        &self,
        user_id: &Uuid,
        tag_id: &Uuid,
    ) -> Result<Vec<GameWithUserInfo>, RepositoryError> {
        let query = game_tag_query::select_all_games_by_tag_id(user_id, tag_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn find_all_tags_with_game(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<Tag>, RepositoryError> {
        let query = game_tag_query::select_all_tags_by_game_id(user_id, game_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn create(&self, game_tag: &GameTag) -> Result<(), RepositoryError> {
        let query = game_tag_query::insert(game_tag);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        tag_id: &Uuid,
    ) -> Result<(), RepositoryError> {
        let query = game_tag_query::delete_by_id(user_id, game_id, tag_id);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        tag_id: &Uuid,
    ) -> Result<bool, RepositoryError> {
        let query = game_tag_query::exists_by_id(user_id, game_id, tag_id);
        exists_id(&self.pool, query).await
    }
}
