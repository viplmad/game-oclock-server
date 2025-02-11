use crate::errors::ApiErrors;
use crate::models::{GameDTO, GameGenreDTO, GenreDTO};
use crate::repository::{GameGenreRepository, GameRepository, GenreRepository};

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::{games_service, genres_service};

pub async fn get_genre_games(
    repository: &GameGenreRepository,
    genre_repository: &GenreRepository,
    user_id: &str,
    genre_id: &str,
) -> Result<Vec<GameDTO>, ApiErrors> {
    genres_service::exists_genre(genre_repository, user_id, genre_id).await?;

    let find_result = repository
        .find_all_games_with_genre(user_id, genre_id)
        .await;
    handle_get_list_result(find_result)
}

pub async fn get_game_genres(
    repository: &GameGenreRepository,
    game_repository: &GameRepository,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<GenreDTO>, ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;

    let find_result = repository.find_all_genres_with_game(user_id, game_id).await;
    handle_get_list_result(find_result)
}

pub async fn create_game_genre(
    repository: &GameGenreRepository,
    game_repository: &GameRepository,
    genre_repository: &GenreRepository,
    user_id: &str,
    game_id: &str,
    genre_id: &str,
) -> Result<(), ApiErrors> {
    games_service::exists_game(game_repository, user_id, game_id).await?;
    genres_service::exists_genre(genre_repository, user_id, genre_id).await?;

    let exists_result = repository.exists_by_id(user_id, game_id, genre_id).await;
    handle_already_exists_result::<GameGenreDTO>(exists_result)?;

    let create_result = repository.create(user_id, game_id, genre_id).await;
    handle_action_result::<GameGenreDTO>(create_result)
}

pub async fn delete_game_genre(
    repository: &GameGenreRepository,
    user_id: &str,
    game_id: &str,
    genre_id: &str,
) -> Result<(), ApiErrors> {
    exists_game_genre(repository, user_id, game_id, genre_id).await?;

    let delete_result = repository.delete_by_id(user_id, game_id, genre_id).await;
    handle_action_result::<GameGenreDTO>(delete_result)
}

pub async fn exists_game_genre(
    repository: &GameGenreRepository,
    user_id: &str,
    game_id: &str,
    genre_id: &str,
) -> Result<(), ApiErrors> {
    let exists_result = repository.exists_by_id(user_id, game_id, genre_id).await;
    handle_not_found_result::<GameGenreDTO>(exists_result)
}
