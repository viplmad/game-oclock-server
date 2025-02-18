use crate::entities::GameWithFinish;
use crate::models::{GameStatus, GameWithFinishDTO};

impl From<GameWithFinish> for GameWithFinishDTO {
    fn from(game: GameWithFinish) -> Self {
        Self {
            id: game.id.to_string(),
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id.map(|id| id.to_string()),
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            finish_date: game.finish_date,
            finish_status: GameStatus::try_from(game.finish_status)
                .expect("Status was not within valid range"),
            finish_device_id: game.finish_device_id.to_string(),
        }
    }
}
