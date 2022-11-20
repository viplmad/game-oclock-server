use crate::entities::{Game, GameAvailable};
use crate::models::{GameAvailableDTO, GameDTO, GameStatus};

impl From<Game> for GameDTO {
    fn from(game: Game) -> Self {
        Self {
            id: game.id,
            user_id: game.user_id,
            name: game.name,
            edition: game.edition,
            release_year: game.release_year,
            cover_filename: game.cover_filename,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            save_folder: game.save_folder,
            screenshot_folder: game.screenshot_folder,
            backup: game.backup,
        }
    }
}

impl From<GameDTO> for Game {
    fn from(game: GameDTO) -> Self {
        Self {
            id: game.id,
            user_id: game.user_id,
            name: game.name,
            edition: game.edition,
            release_year: game.release_year,
            cover_filename: game.cover_filename,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: i16::from(game.status),
            rating: game.rating,
            notes: game.notes,
            save_folder: game.save_folder,
            screenshot_folder: game.screenshot_folder,
            backup: game.backup,
        }
    }
}

impl From<GameAvailable> for GameAvailableDTO {
    fn from(game: GameAvailable) -> Self {
        Self {
            id: game.id,
            user_id: game.user_id,
            available_date: game.available_date,
            name: game.name,
            edition: game.edition,
            release_year: game.release_year,
            cover_filename: game.cover_filename,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            save_folder: game.save_folder,
            screenshot_folder: game.screenshot_folder,
            backup: game.backup,
        }
    }
}
