use sqlx::PgPool;
use uuid::Uuid;

use super::query::game_link_query;
use crate::entities::GameLink;
use crate::errors::RepositoryError;

use super::helpers::{execute, exists_id, fetch_all};

#[derive(Clone)]
pub struct GameLinkRepository {
    pool: PgPool,
}

impl GameLinkRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameLinkRepository {
    pub async fn find_all_by_game_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<GameLink>, RepositoryError> {
        let query = game_link_query::select_all_by_user_id_and_game_id(user_id, game_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn create(&self, game_link: &GameLink) -> Result<(), RepositoryError> {
        let query = game_link_query::insert(game_link);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        url: &str,
    ) -> Result<(), RepositoryError> {
        let query = game_link_query::delete_by_id(user_id, game_id, url);
        execute(&self.pool, query).await
    }

    pub async fn exists_by_id(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        url: &str,
    ) -> Result<bool, RepositoryError> {
        let query = game_link_query::exists_by_id(user_id, game_id, url);
        exists_id(&self.pool, query).await
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_all_by_game_id() {
        assert_eq!(
            game_link_query::select_all_by_user_id_and_game_id(&Uuid::now_v7(), &Uuid::now_v7())
                .to_string(sea_query::PostgresQueryBuilder),
            "SELECT \"GameLink\".\"url\", \"GameLink\".\"description\" FROM \"GameLink\" WHERE \"GameLink\".\"user_id\" = 'user_id' AND \"GameLink\".\"game_id\" = 'game_id'"
        );
    }
}
