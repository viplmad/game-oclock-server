use sqlx::PgPool;

use super::query::game_query;
use crate::entities::{Game, GameSearch, PageResult};
use crate::errors::{RepositoryError, SearchErrors};

use super::base::{
    begin_transaction, commit_transaction, execute, exists_id, fetch_all, fetch_all_search,
    fetch_optional,
};

pub async fn find_by_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
) -> Result<Option<Game>, RepositoryError> {
    let query = game_query::select_by_id(user_id, id);
    fetch_optional(pool, query).await
}

pub async fn find_all_by_base_game_id(
    pool: &PgPool,
    user_id: &str,
    base_game_id: &str,
) -> Result<Vec<Game>, RepositoryError> {
    let query = game_query::select_all_by_base_game_id(user_id, base_game_id);
    fetch_all(pool, query).await
}

pub async fn search_all(
    pool: &PgPool,
    user_id: &str,
    search: GameSearch,
) -> Result<PageResult<Game>, SearchErrors> {
    let search_query = game_query::select_all_with_search(user_id, search)?;
    fetch_all_search(pool, search_query).await
}

pub async fn create(pool: &PgPool, user_id: &str, game: &Game) -> Result<String, RepositoryError> {
    let id = crate::uuid_utils::new_model_uuid();

    let mut transaction = begin_transaction(pool).await?;

    let query = game_query::insert(user_id, &id, game);
    execute(&mut *transaction, query).await?;

    let user_info_query = game_query::insert_user_info(user_id, &id, game);
    execute(&mut *transaction, user_info_query).await?;

    commit_transaction(transaction).await?;

    Ok(id)
}

pub async fn update_by_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
    game: &Game,
) -> Result<(), RepositoryError> {
    let mut transaction = begin_transaction(pool).await?;

    let query = game_query::update_by_id(user_id, id, game);
    execute(&mut *transaction, query).await?;

    let user_info_query = game_query::update_user_info_by_id(user_id, id, game);
    execute(&mut *transaction, user_info_query).await?;

    commit_transaction(transaction).await?;

    Ok(())
}

pub async fn update_base_game_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
    base_game_id: Option<String>,
) -> Result<(), RepositoryError> {
    let query = game_query::update_base_game_id_by_id(user_id, id, base_game_id);
    execute(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, user_id: &str, id: &str) -> Result<(), RepositoryError> {
    let mut transaction = begin_transaction(pool).await?;

    let query = game_query::delete_by_id(user_id, id);
    execute(&mut *transaction, query).await?;

    let user_info_query = game_query::delete_user_info_by_id(user_id, id);
    execute(&mut *transaction, user_info_query).await?;

    commit_transaction(transaction).await?;

    Ok(())
}

pub async fn exists_by_id(pool: &PgPool, user_id: &str, id: &str) -> Result<bool, RepositoryError> {
    let query = game_query::exists_by_id(user_id, id);
    exists_id(pool, query).await
}

pub async fn exists_with_unique(
    pool: &PgPool,
    user_id: &str,
    game: &Game,
) -> Result<bool, RepositoryError> {
    let query = game_query::exists_by_name_and_edition(user_id, &game.title, &game.edition);
    exists_id(pool, query).await
}

pub async fn exists_with_unique_except_id(
    pool: &PgPool,
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
    exists_id(pool, query).await
}
