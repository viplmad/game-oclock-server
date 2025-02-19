use crate::entities::GameWithFinish;
use crate::models::{FinishDTO, GameDTO, GameStatus, GameWithFinishDTO};

impl From<GameWithFinish> for GameWithFinishDTO {
    fn from(game: GameWithFinish) -> Self {
        Self {
            game: GameDTO {
                id: game.id,
                user_id: game.user_id,
                title: game.title,
                edition: game.edition,
                release_date: game.release_date,
                base_game_id: game.base_game_id,
                cover_url: game.cover_url,
                added_datetime: game.added_datetime,
                updated_datetime: game.updated_datetime,
                status: GameStatus::try_from(game.status)
                    .expect("Status is not within valid range"),
                rating: u32::try_from(game.rating).expect("Rating is not positive"),
                notes: game.notes,
            },
            finish: FinishDTO {
                date: game.finish_date,
                status: GameStatus::try_from(game.finish_status)
                    .expect("Status is not within valid range"),
                device_id: game.finish_device_id,
            },
        }
    }
}
