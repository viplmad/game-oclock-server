use uuid::Uuid;

use crate::entities::{GameLog, GameLogWithTime, GameWithLog};
use crate::models::{DurationDef, LogDTO};

// Mapping to DTO only with time
impl From<GameLogWithTime> for LogDTO {
    fn from(log: GameLogWithTime) -> Self {
        Self {
            start_datetime: log.start_datetime,
            end_datetime: log.end_datetime,
            device_id: log.device_id,
            time: DurationDef::from(log.query_time.clone()),
        }
    }
}

impl From<LogDTO> for GameLog {
    fn from(log: LogDTO) -> Self {
        Self {
            user_id: Uuid::default(),
            game_id: Uuid::default(),
            start_datetime: log.start_datetime,
            end_datetime: log.end_datetime,
            device_id: log.device_id,
        }
    }
}

// TODO Remove borrow
impl From<&GameWithLog> for LogDTO {
    fn from(game: &GameWithLog) -> Self {
        Self {
            start_datetime: game.log_start_datetime,
            end_datetime: game.log_end_datetime,
            device_id: game.log_device_id,
            time: DurationDef::from(game.log_time.clone()),
        }
    }
}
