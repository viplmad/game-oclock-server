use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::{postgres::types::PgInterval, FromRow};
use uuid::Uuid;

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
    #[iden = "start_datetime"]
    StartDateTime,
    #[iden = "end_datetime"]
    EndDateTime,
    #[iden = "device_id"]
    DeviceId,
}

impl TableIden for GameLogIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct GameLog {
    pub game_id: Uuid,
    pub start_datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
    pub device_id: Option<Uuid>,
}

#[derive(FromRow)]
pub struct GameLogWithTime {
    pub game_id: Uuid,
    pub start_datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
    pub device_id: Option<Uuid>,
    pub query_time: PgInterval,
}
