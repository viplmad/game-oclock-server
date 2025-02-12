use crate::entities::GameWithLog;
use crate::models::{DurationDef, GameStatus, GameWithLogDTO};

impl From<GameWithLog> for GameWithLogDTO {
    fn from(game: GameWithLog) -> Self {
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
            log_start_datetime: game.log_start_datetime,
            log_end_datetime: game.log_end_datetime,
            log_device_id: game.log_device_id.to_string(),
            log_time: DurationDef::from(game.log_time),
        }
    }
}
