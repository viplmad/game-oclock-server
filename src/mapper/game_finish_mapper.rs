use crate::entities::{Finish, GameFinish, GameWithFinish};
use crate::models::{FinishDTO, GameStatus};
use crate::uuid_utils;

impl From<Finish> for FinishDTO {
    fn from(finish: Finish) -> Self {
        Self {
            date: finish.date,
            status: GameStatus::try_from(finish.status).expect("Status was not within valid range"),
            device_id: finish.device_id.map(|id| id.to_string()),
        }
    }
}

impl From<FinishDTO> for Finish {
    fn from(finish: FinishDTO) -> Self {
        Self {
            date: finish.date,
            status: i16::from(finish.status),
            device_id: finish.device_id.map(|id| uuid_utils::parse_uuid(&id)),
        }
    }
}

// TODO Remove borrow
impl From<&GameWithFinish> for FinishDTO {
    fn from(game: &GameWithFinish) -> Self {
        Self {
            date: game.finish_date,
            status: GameStatus::try_from(game.finish_status)
                .expect("Status was not within valid range"),
            device_id: game.finish_device_id.map(|id| id.to_string()),
        }
    }
}

impl From<GameFinish> for FinishDTO {
    fn from(finish: GameFinish) -> Self {
        Self {
            date: finish.date,
            status: GameStatus::try_from(finish.status).expect("Status was not within valid range"),
            device_id: finish.device_id.map(|id| id.to_string()),
        }
    }
}
