use uuid::Uuid;

use crate::entities::{GameFinish, GameWithFinish};
use crate::models::{FinishDTO, GameStatus};

impl From<GameFinish> for FinishDTO {
    fn from(finish: GameFinish) -> Self {
        Self {
            date: finish.date,
            status: GameStatus::try_from(finish.status).expect("Status was not within valid range"),
            device_id: finish.device_id,
        }
    }
}

impl From<FinishDTO> for GameFinish {
    fn from(finish: FinishDTO) -> Self {
        Self {
            user_id: Uuid::default(),
            game_id: Uuid::default(),
            date: finish.date,
            status: i16::from(finish.status),
            device_id: finish.device_id,
        }
    }
}

impl From<&GameWithFinish> for FinishDTO {
    fn from(game: &GameWithFinish) -> Self {
        Self {
            date: game.finish_date,
            status: GameStatus::try_from(game.finish_status)
                .expect("Status was not within valid range"),
            device_id: game.finish_device_id,
        }
    }
}
