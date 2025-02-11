use crate::entities::GameSearch;
use crate::errors::ApiErrors;
use crate::models::{GameDTO, GamePageResult, GameStatus, NewGameDTO, SearchDTO};
use crate::repository::{GameAvailableRepository, GameRepository};

use super::base::{
    create_merged, handle_action_result, handle_already_exists_result, handle_create_result,
    handle_get_list_paged_result, handle_get_result, handle_not_found_result, handle_query_mapping,
    handle_update_result, update_merged,
};
use super::game_available_service;

pub async fn get_game(
    repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<GameDTO, ApiErrors> {
    let find_result = repository.find_by_id(user_id, game_id).await;
    handle_get_result(find_result)
}

pub async fn search_games(
    repository: &GameRepository,
    user_id: &str,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GamePageResult, ApiErrors> {
    let search = handle_query_mapping::<GameDTO, GameSearch>(search, quicksearch)?;
    let find_result = repository.search_all(user_id, search).await;
    handle_get_list_paged_result(find_result)
}

pub async fn create_game(
    repository: &GameRepository,
    user_id: &str,
    game: NewGameDTO,
) -> Result<GameDTO, ApiErrors> {
    // TODO check image is reachable
    create_merged(
        game,
        async move |created_game_id| get_game(repository, user_id, &created_game_id).await,
        async move |game_to_create| {
            let exists_result = repository
                .exists_with_unique(user_id, &game_to_create)
                .await;
            handle_already_exists_result::<GameDTO>(exists_result)?;

            let create_result = repository.create(user_id, &game_to_create).await;
            handle_create_result::<String, GameDTO>(create_result)
        },
    )
    .await
}

pub async fn update_game(
    repository: &GameRepository,
    game_available_repository: &GameAvailableRepository,
    user_id: &str,
    game_id: &str,
    game: NewGameDTO,
) -> Result<(), ApiErrors> {
    let new_status = game.status.clone();

    update_merged(
        game,
        async move || get_game(repository, user_id, game_id).await,
        async move |game_to_update| {
            let exists_result = repository
                .exists_with_unique_except_id(user_id, &game_to_update, game_id)
                .await;
            handle_already_exists_result::<GameDTO>(exists_result)?;

            // TODO Only check if old is not wishlist
            if new_status.is_some_and(|status| status == GameStatus::Wishlist) {
                game_available_service::exists_no_game_available(
                    game_available_repository,
                    user_id,
                    game_id,
                )
                .await?
            }

            let update_result = repository
                .update_by_id(user_id, game_id, &game_to_update)
                .await;
            handle_update_result::<GameDTO>(update_result)
        },
    )
    .await
}

pub async fn delete_game(
    repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<(), ApiErrors> {
    // TODO Error if game is used -> use sql contraints
    let delete_result = repository.delete_by_id(user_id, game_id).await;
    handle_action_result::<GameDTO>(delete_result)
}

pub async fn exists_game(
    repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, game_id).await;
    handle_not_found_result::<GameDTO>(exists_result)
}
