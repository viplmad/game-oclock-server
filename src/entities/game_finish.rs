use chrono::NaiveDate;
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::TableIden;

#[derive(FromRow)]
#[enum_def(table_name = "GameFinish")]
pub struct GameFinish {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub date: NaiveDate, // TODO store as datetime
    pub status: i16,
    pub device_id: Option<Uuid>,
}

impl TableIden for GameFinishIden {
    const TABLE: Self = Self::Table;
}
