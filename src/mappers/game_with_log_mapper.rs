use crate::entities::GameWithLog;
use crate::models::{DurationDef, GameDTO, GameStatus, GameWithLogDTO, LogDTO};

impl From<GameWithLog> for GameWithLogDTO {
    fn from(game: GameWithLog) -> Self {
        Self {
            game: GameDTO {
                id: game.id,
                user_id: game.user_id,
                title: game.title,
                edition: game.edition,
                release_date: game.release_date,
                base_game_id: game.base_game_id,
                cover_url: game.cover_url,
                added_datetime: game.added_datetime,
                updated_datetime: game.updated_datetime,
                status: GameStatus::try_from(game.status)
                    .expect("Status is not within valid range"),
                rating: u32::try_from(game.rating).expect("Rating is not positive"),
                notes: game.notes,
            },
            log: LogDTO {
                start_datetime: game.log_start_datetime,
                end_datetime: game.log_end_datetime,
                device_id: game.log_device_id,
                time: DurationDef::from(game.log_time),
            },
        }
    }
}
