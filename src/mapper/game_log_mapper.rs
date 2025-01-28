use crate::entities::{GameLog, GameLogWithTime, GameWithLog};
use crate::models::{DurationDef, GameLogDTO};
use crate::uuid_utils;

impl From<GameLogWithTime> for GameLogDTO {
    fn from(log: GameLogWithTime) -> Self {
        Self {
            start_datetime: log.start_datetime,
            end_datetime: log.end_datetime,
            device_id: log.device_id.map(|id| id.to_string()),
            time: DurationDef::from(log.query_time),
        }
    }
}

impl From<GameLogDTO> for GameLog {
    fn from(log: GameLogDTO) -> Self {
        Self {
            start_datetime: log.start_datetime,
            end_datetime: log.end_datetime,
            device_id: log.device_id.map(|id| uuid_utils::parse_uuid(&id)),
            // TODO get game_id?
        }
    }
}

// TODO Remove borrow
impl From<&GameWithLog> for GameLogDTO {
    fn from(game: &GameWithLog) -> Self {
        Self {
            start_datetime: game.log_start_datetime,
            end_datetime: game.log_end_datetime,
            device_id: game.log_device_id.map(|id| id.to_string()),
            time: DurationDef::from(game.log_time.clone()),
        }
    }
}
