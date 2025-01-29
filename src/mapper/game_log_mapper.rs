use sqlx::postgres::types::PgInterval;

use crate::entities::{GameLogWithTime, GameWithLog, LogWithTime};
use crate::models::{DurationDef, LogDTO};
use crate::uuid_utils;

impl From<LogWithTime> for LogDTO {
    fn from(log: LogWithTime) -> Self {
        Self {
            start_datetime: log.start_datetime,
            end_datetime: log.end_datetime,
            device_id: log.device_id.map(|id| id.to_string()),
            time: DurationDef::from(log.query_time),
        }
    }
}

impl From<LogDTO> for LogWithTime {
    fn from(log: LogDTO) -> Self {
        Self {
            start_datetime: log.start_datetime,
            end_datetime: log.end_datetime,
            device_id: log.device_id.map(|id| uuid_utils::parse_uuid(&id)),
            query_time: PgInterval::default(), // Ignored
        }
    }
}

// TODO Remove borrow
impl From<&GameWithLog> for LogDTO {
    fn from(game: &GameWithLog) -> Self {
        Self {
            start_datetime: game.log_start_datetime,
            end_datetime: game.log_end_datetime,
            device_id: game.log_device_id.map(|id| id.to_string()),
            time: DurationDef::from(game.log_time.clone()),
        }
    }
}

impl From<GameLogWithTime> for LogDTO {
    fn from(log: GameLogWithTime) -> Self {
        Self {
            start_datetime: log.start_datetime,
            end_datetime: log.end_datetime,
            device_id: log.device_id.map(|id| id.to_string()),
            time: DurationDef::from(log.query_time.clone()),
        }
    }
}
