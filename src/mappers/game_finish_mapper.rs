use uuid::Uuid;

use crate::entities::{GameFinish, GameWithFinish};
use crate::models::{FinishDTO, GameStatus};

impl From<GameFinish> for FinishDTO {
    fn from(finish: GameFinish) -> Self {
        Self {
            datetime: finish.datetime,
            status: GameStatus::try_from(finish.status).expect("Status is not within valid range"),
            device_id: finish.device_id,
        }
    }
}

impl From<FinishDTO> for GameFinish {
    fn from(finish: FinishDTO) -> Self {
        Self {
            user_id: Uuid::default(),
            game_id: Uuid::default(),
            datetime: finish.datetime,
            status: i16::from(finish.status),
            device_id: finish.device_id,
        }
    }
}

impl From<&GameWithFinish> for FinishDTO {
    fn from(game: &GameWithFinish) -> Self {
        Self {
            datetime: game.finish_datetime,
            status: GameStatus::try_from(game.finish_status)
                .expect("Status is not within valid range"),
            device_id: game.finish_device_id,
        }
    }
}
