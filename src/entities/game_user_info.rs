use chrono::NaiveDateTime;
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{GameWithUserInfo, TableIden};

#[derive(FromRow)]
#[enum_def(table_name = "GameUserInfo")]
pub struct GameUserInfo {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub status: i16,
    pub rating: i16,
    pub notes: String,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl TableIden for GameUserInfoIden {
    const TABLE: Self = Self::Table;
}

impl From<&GameWithUserInfo> for GameUserInfo {
    fn from(game: &GameWithUserInfo) -> Self {
        Self {
            user_id: game.user_id,
            game_id: game.id,
            status: game.status,
            rating: game.rating,
            notes: game.notes.clone(),
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
        }
    }
}
