use sqlx::PgPool;

use super::query::game_query;
use crate::entities::{Game, GameSearch, PageResult};
use crate::errors::{RepositoryError, SearchErrors};

use super::base::{
    begin_transaction, commit_transaction, execute, exists_id, fetch_all, fetch_all_search,
    fetch_optional,
};

#[derive(Clone)]
pub struct GameRepository {
    pool: PgPool,
}

impl GameRepository {
    pub fn with_connection(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl GameRepository {
    pub async fn find_by_id(
        &self,
        user_id: &str,
        id: &str,
    ) -> Result<Option<Game>, RepositoryError> {
        let query = game_query::select_by_id(user_id, id);
        fetch_optional(&self.pool, query).await
    }

    pub async fn find_all_by_base_game_id(
        &self,
        user_id: &str,
        base_game_id: &str,
    ) -> Result<Vec<Game>, RepositoryError> {
        let query = game_query::select_all_by_base_game_id(user_id, base_game_id);
        fetch_all(&self.pool, query).await
    }

    pub async fn search_all(
        &self,
        user_id: &str,
        search: GameSearch,
    ) -> Result<PageResult<Game>, SearchErrors> {
        let search_query = game_query::select_all_with_search(user_id, search)?;
        fetch_all_search(&self.pool, search_query).await
    }

    pub async fn create(&self, user_id: &str, game: &Game) -> Result<String, RepositoryError> {
        let id = crate::uuid_utils::new_model_uuid();

        let mut transaction = begin_transaction(&self.pool).await?;

        let query = game_query::insert(user_id, &id, game);
        execute(&mut *transaction, query).await?;

        let user_info_query = game_query::insert_user_info(user_id, &id, game);
        execute(&mut *transaction, user_info_query).await?;

        commit_transaction(transaction).await?;

        Ok(id)
    }

    pub async fn update_by_id(
        &self,
        user_id: &str,
        id: &str,
        game: &Game,
    ) -> Result<(), RepositoryError> {
        let mut transaction = begin_transaction(&self.pool).await?;

        let query = game_query::update_by_id(user_id, id, game);
        execute(&mut *transaction, query).await?;

        let user_info_query = game_query::update_user_info_by_id(user_id, id, game);
        execute(&mut *transaction, user_info_query).await?;

        commit_transaction(transaction).await?;

        Ok(())
    }

    pub async fn update_base_game_id(
        &self,
        user_id: &str,
        id: &str,
        base_game_id: Option<String>,
    ) -> Result<(), RepositoryError> {
        let query = game_query::update_base_game_id_by_id(user_id, id, base_game_id);
        execute(&self.pool, query).await
    }

    pub async fn delete_by_id(&self, user_id: &str, id: &str) -> Result<(), RepositoryError> {
        let mut transaction = begin_transaction(&self.pool).await?;

        let query = game_query::delete_by_id(user_id, id);
        execute(&mut *transaction, query).await?;

        let user_info_query = game_query::delete_user_info_by_id(user_id, id);
        execute(&mut *transaction, user_info_query).await?;

        commit_transaction(transaction).await?;

        Ok(())
    }

    pub async fn exists_by_id(&self, user_id: &str, id: &str) -> Result<bool, RepositoryError> {
        let query = game_query::exists_by_id(user_id, id);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_unique(
        &self,
        user_id: &str,
        game: &Game,
    ) -> Result<bool, RepositoryError> {
        let query = game_query::exists_by_name_and_edition(user_id, &game.title, &game.edition);
        exists_id(&self.pool, query).await
    }

    pub async fn exists_with_unique_except_id(
        &self,
        user_id: &str,
        game: &Game,
        excluded_id: &str,
    ) -> Result<bool, RepositoryError> {
        let query = game_query::exists_by_name_and_edition_and_id_not(
            user_id,
            &game.title,
            &game.edition,
            excluded_id,
        );
        exists_id(&self.pool, query).await
    }
}
