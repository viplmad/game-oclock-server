use uuid::Uuid;

use crate::entities::{Game, GameWithDate};
use crate::models::{GameAvailableDTO, GameDTO, GameStatus};
use crate::uuid_utils;

impl From<Game> for GameDTO {
    fn from(game: Game) -> Self {
        Self {
            id: game.id.to_string(),
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id.map(|id| id.to_string()),
            cover_filename: None, // TODO extract filename from url
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
        }
    }
}

impl From<GameDTO> for Game {
    fn from(game: GameDTO) -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id.map(|id| uuid_utils::parse_uuid(&id)),
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: i16::from(game.status),
            rating: game.rating,
            notes: game.notes,
        }
    }
}

impl From<GameWithDate> for GameAvailableDTO {
    fn from(game: GameWithDate) -> Self {
        Self {
            id: game.id.to_string(),
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id.map(|id| id.to_string()),
            cover_filename: None, // TODO extract filename from url
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            available_date: game.query_date,
        }
    }
}
