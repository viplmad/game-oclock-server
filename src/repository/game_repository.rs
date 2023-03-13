use sqlx::PgPool;

use crate::entities::{Game, GameSearch, PageResult};
use crate::errors::{RepositoryError, SearchErrors};
use crate::query::game_query;

use super::base::{
    begin_transaction, commit_transaction, execute, execute_return, execute_return_id, exists_id,
    fetch_all_search, fetch_optional,
};

pub async fn find_by_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
) -> Result<Option<Game>, RepositoryError> {
    let query = game_query::select_by_id(user_id, id);
    fetch_optional(pool, query).await
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
    let mut transaction = begin_transaction(pool).await?;

    let query = game_query::insert(user_id, game);
    let game_id = execute_return_id(&mut transaction, query).await?;

    let user_info_query = game_query::insert_user_info(user_id, &game_id, game);
    let _user_info_id: (String, String) = execute_return(&mut transaction, user_info_query).await?;

    commit_transaction(transaction).await?;

    Ok(game_id)
}

pub async fn update_by_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
    game: &Game,
) -> Result<String, RepositoryError> {
    // Update should not return id
    let mut transaction = begin_transaction(pool).await?;

    let query = game_query::update_by_id(user_id, id, game);
    execute_return_id(&mut transaction, query).await?;

    let user_info_query = game_query::update_user_info_by_id(user_id, id, game);
    let _user_info_id: (String, String) = execute_return(&mut transaction, user_info_query).await?;

    commit_transaction(transaction).await?;

    Ok(String::from(id))
}

pub async fn update_cover_filename_by_id(
    pool: &PgPool,
    user_id: &str,
    id: &str,
    cover_filename: Option<String>,
) -> Result<(), RepositoryError> {
    let query = game_query::update_cover_filename_by_id(user_id, id, cover_filename);
    execute(pool, query).await
}

pub async fn delete_by_id(pool: &PgPool, user_id: &str, id: &str) -> Result<(), RepositoryError> {
    let mut transaction = begin_transaction(pool).await?;

    let query = game_query::delete_by_id(user_id, id);
    execute(&mut transaction, query).await?;

    let user_info_query = game_query::delete_user_info_by_id(user_id, id);
    execute(&mut transaction, user_info_query).await?;

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
    let query = game_query::exists_by_name_and_edition(user_id, &game.name, &game.edition);
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
        &game.name,
        &game.edition,
        excluded_id,
    );
    exists_id(pool, query).await
}
