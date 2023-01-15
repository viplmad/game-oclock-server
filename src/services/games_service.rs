use sqlx::PgPool;

use crate::entities::GameSearch;
use crate::errors::ApiErrors;
use crate::models::{GameDTO, GamePageResult, NewGameDTO, SearchDTO};
use crate::repository::game_repository;

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};

pub async fn get_game(pool: &PgPool, user_id: i32, game_id: i32) -> Result<GameDTO, ApiErrors> {
    let find_result = game_repository::find_by_id(pool, user_id, game_id).await;
    handle_get_result(find_result)
}

pub async fn search_games(
    pool: &PgPool,
    user_id: i32,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GamePageResult, ApiErrors> {
    let search = handle_query_mapping::<GameDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_repository::search_all(pool, user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_game(
    pool: &PgPool,
    user_id: i32,
    game: NewGameDTO,
) -> Result<GameDTO, ApiErrors> {
    create_merged(
        game,
        async move |created_game_id| get_game(pool, user_id, created_game_id).await,
        async move |game_to_create| {
            let exists_result =
                game_repository::exists_with_unique(pool, user_id, &game_to_create).await;
            handle_already_exists_result::<GameDTO>(exists_result)?;

            let create_result = game_repository::create(pool, user_id, &game_to_create).await;
            handle_create_result::<i32, GameDTO>(create_result)
        },
    )
    .await
}

pub async fn update_game(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    game: NewGameDTO,
) -> Result<GameDTO, ApiErrors> {
    update_merged(
        game,
        async move || get_game(pool, user_id, game_id).await,
        async move |game_to_update| {
            let exists_result = game_repository::exists_with_unique_except_id(
                pool,
                user_id,
                &game_to_update,
                game_id,
            )
            .await;
            handle_already_exists_result::<GameDTO>(exists_result)?;

            let update_result =
                game_repository::update_by_id(pool, user_id, game_id, &game_to_update).await;
            handle_update_result::<i32, GameDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_game(pool: &PgPool, user_id: i32, game_id: i32) -> Result<(), ApiErrors> {
    exists_game(pool, user_id, game_id).await?;

    let delete_result = game_repository::delete_by_id(pool, user_id, game_id).await;
    handle_action_result::<GameDTO>(delete_result)
}

pub async fn set_game_cover(
    pool: &PgPool,
    user_id: i32,
    game_id: i32,
    filename: &str,
) -> Result<(), ApiErrors> {
    let update_result =
        game_repository::update_cover_filename_by_id(pool, user_id, game_id, filename).await;
    handle_action_result::<GameDTO>(update_result)
}

pub async fn exists_game(pool: &PgPool, user_id: i32, game_id: i32) -> Result<(), ApiErrors> {
    let exists_result = game_repository::exists_by_id(pool, user_id, game_id).await;
    handle_not_found_result::<GameDTO>(exists_result)
}
