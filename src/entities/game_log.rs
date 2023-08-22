use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::{postgres::types::PgInterval, FromRow};

use super::TableIden;

pub const QUERY_TIME_ALIAS: &str = "query_time";

#[derive(Iden)]
#[iden = "GameLog"]
pub enum GameLogIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "datetime"]
    StartDateTime,
    #[iden = "end_datetime"]
    EndDateTime,
}

impl TableIden for GameLogIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct GameLog {
    pub datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
}

#[derive(FromRow)]
pub struct GameLogWithTime {
    pub datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
    pub query_time: PgInterval,
}
