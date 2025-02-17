use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
#[enum_def(table_name = "GameGenre")]
pub struct GameGenre {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub genre_id: Uuid,
}
