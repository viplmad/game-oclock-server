use crate::entities::{GameLog, GameLogWithTime, GameWithLog};
use crate::models::{DurationDef, GameLogDTO};

impl From<GameLogWithTime> for GameLogDTO {
    fn from(log: GameLogWithTime) -> Self {
        Self {
            start_datetime: log.datetime,
            end_datetime: log.end_datetime,
            time: DurationDef::from(log.query_time),
        }
    }
}

impl From<GameLogDTO> for GameLog {
    fn from(log: GameLogDTO) -> Self {
        Self {
            datetime: log.start_datetime,
            end_datetime: log.end_datetime,
        }
    }
}

impl From<&GameWithLog> for GameLogDTO {
    fn from(game: &GameWithLog) -> Self {
        Self {
            start_datetime: game.log_start_datetime,
            end_datetime: game.log_end_datetime,
            time: DurationDef::from(game.log_time.clone()),
        }
    }
}
