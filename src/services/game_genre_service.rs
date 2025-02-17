use uuid::Uuid;

use crate::entities::GameGenre;
use crate::errors::ApiErrors;
use crate::models::{GameDTO, GameGenreDTO, GenreDTO};
use crate::repository::GameGenreRepository;

use super::helpers::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result,
};
use super::{GameService, GenreService};

#[derive(Clone)]
pub struct GameGenreService {
    repository: GameGenreRepository,
    game_service: GameService,
    genre_service: GenreService,
}

impl GameGenreService {
    pub fn with(
        repository: GameGenreRepository,
        game_service: GameService,
        genre_service: GenreService,
    ) -> Self {
        Self {
            repository,
            game_service,
            genre_service,
        }
    }
}

impl GameGenreService {
    pub async fn get_genre_games(
        &self,
        user_id: &Uuid,
        genre_id: &Uuid,
    ) -> Result<Vec<GameDTO>, ApiErrors> {
        self.genre_service.exists_genre(user_id, genre_id).await?;

        let find_result = self
            .repository
            .find_all_games_with_genre(user_id, genre_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn get_game_genres(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
    ) -> Result<Vec<GenreDTO>, ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;

        let find_result = self
            .repository
            .find_all_genres_with_game(user_id, game_id)
            .await;
        handle_get_list_result(find_result)
    }

    pub async fn create_game_genre(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        genre_id: &Uuid,
    ) -> Result<(), ApiErrors> {
        self.game_service.exists_game(user_id, game_id).await?;
        self.genre_service.exists_genre(user_id, genre_id).await?;

        let exists_result = self
            .repository
            .exists_by_id(user_id, game_id, genre_id)
            .await;
        handle_already_exists_result::<GameGenreDTO>(exists_result)?;

        let create_result = self
            .repository
            .create(&GameGenre {
                user_id: user_id.clone(),
                game_id: game_id.clone(),
                genre_id: genre_id.clone(),
            })
            .await;
        handle_action_result::<GameGenreDTO>(create_result)
    }

    pub async fn delete_game_genre(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        genre_id: &Uuid,
    ) -> Result<(), ApiErrors> {
        self.exists_game_genre(user_id, game_id, genre_id).await?;

        let delete_result = self
            .repository
            .delete_by_id(user_id, game_id, genre_id)
            .await;
        handle_action_result::<GameGenreDTO>(delete_result)
    }

    pub async fn exists_game_genre(
        &self,
        user_id: &Uuid,
        game_id: &Uuid,
        genre_id: &Uuid,
    ) -> Result<(), ApiErrors> {
        let exists_result = self
            .repository
            .exists_by_id(user_id, game_id, genre_id)
            .await;
        handle_not_found_result::<GameGenreDTO>(exists_result)
    }
}
