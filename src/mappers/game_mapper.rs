use uuid::Uuid;

use crate::entities::{GameWithUserInfo, GameWithUserInfoWithDate};
use crate::models::{GameAvailableDTO, GameDTO, GameStatus};

impl From<GameWithUserInfo> for GameDTO {
    fn from(game: GameWithUserInfo) -> Self {
        Self {
            id: game.id,
            user_id: game.user_id,
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id,
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status is not within valid range"),
            rating: u32::try_from(game.rating).expect("Rating is not positive"),
            notes: game.notes,
        }
    }
}

impl From<GameDTO> for GameWithUserInfo {
    fn from(game: GameDTO) -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id,
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: i16::from(game.status),
            rating: i16::try_from(game.rating).expect("Rating is not within valid range"),
            notes: game.notes,
        }
    }
}

impl From<GameWithUserInfoWithDate> for GameAvailableDTO {
    fn from(game: GameWithUserInfoWithDate) -> Self {
        Self {
            id: game.id,
            user_id: game.user_id,
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id,
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status is not within valid range"),
            rating: u32::try_from(game.rating).expect("Rating is not positive"),
            notes: game.notes,
            available_date: game.query_date,
        }
    }
}
