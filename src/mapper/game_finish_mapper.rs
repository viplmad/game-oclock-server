use crate::entities::GameFinish;
use crate::models::{GameFinishDTO, GameStatus};
use crate::uuid_utils;

impl From<GameFinish> for GameFinishDTO {
    fn from(finish: GameFinish) -> Self {
        Self {
            date: finish.date,
            status: GameStatus::try_from(finish.status).expect("Status was not within valid range"),
            device_id: finish.device_id.map(|id| id.to_string()),
        }
    }
}

impl From<GameFinishDTO> for GameFinish {
    fn from(finish: GameFinishDTO) -> Self {
        Self {
            date: finish.date,
            status: i16::from(finish.status),
            device_id: finish.device_id.map(|id| uuid_utils::parse_uuid(&id)),
            // TODO get game_id?
        }
    }
}
