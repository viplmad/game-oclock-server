use chrono::NaiveDate;
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
#[enum_def(table_name = "GameAvailable")]
pub struct GameAvailable {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub location_id: Uuid,
    pub date: NaiveDate,
}
