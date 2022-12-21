use sqlx::postgres::types::PgInterval;

use crate::entities::{GameLog, GameWithLog};
use crate::models::{DurationDef, GameLogDTO};

impl From<GameLog> for GameLogDTO {
    fn from(log: GameLog) -> Self {
        Self {
            datetime: log.datetime,
            time: DurationDef::from(log.time),
        }
    }
}

impl From<GameLogDTO> for GameLog {
    fn from(log: GameLogDTO) -> Self {
        Self {
            datetime: log.datetime,
            time: PgInterval::from(log.time),
        }
    }
}

impl From<&GameWithLog> for GameLogDTO {
    fn from(game: &GameWithLog) -> Self {
        Self {
            datetime: game.log_datetime,
            time: DurationDef::from(game.log_time.clone()),
        }
    }
}
