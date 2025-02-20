use chrono::NaiveDateTime;
use sea_query::enum_def;
use sqlx::{FromRow, postgres::types::PgInterval};
use uuid::Uuid;

use super::TableIden;

pub const QUERY_TIME_ALIAS: &str = "query_time";

#[derive(FromRow)]
#[enum_def(table_name = "GameLog")]
pub struct GameLog {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub start_datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
    pub device_id: Option<Uuid>,
}

impl TableIden for GameLogIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct GameLogWithTime {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub start_datetime: NaiveDateTime,
    pub end_datetime: NaiveDateTime,
    pub device_id: Option<Uuid>,
    pub query_time: PgInterval,
}
