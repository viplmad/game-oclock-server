use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
#[enum_def(table_name = "GameTag")]
pub struct GameTag {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub tag_id: Uuid,
}
