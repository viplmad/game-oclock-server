use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::{postgres::types::PgInterval, FromRow};

use super::TableIden;

#[derive(Iden)]
#[iden = "GameLog"]
pub enum GameLogIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "datetime"]
    DateTime,
    #[iden = "time"]
    Time,
}

impl TableIden for GameLogIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct GameLog {
    pub datetime: NaiveDateTime,
    pub time: PgInterval,
}
