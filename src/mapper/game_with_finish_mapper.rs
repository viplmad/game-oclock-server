use crate::entities::GameWithFinish;
use crate::models::{GameStatus, GameWithFinishDTO};

impl From<GameWithFinish> for GameWithFinishDTO {
    fn from(game: GameWithFinish) -> Self {
        Self {
            id: game.id,
            name: game.name,
            edition: game.edition,
            release_year: game.release_year,
            cover_filename: game.cover_filename,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            save_folder: game.save_folder,
            screenshot_folder: game.screenshot_folder,
            backup: game.backup,
            finish_date: game.finish_date,
        }
    }
}
