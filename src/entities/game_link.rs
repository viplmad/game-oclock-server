use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::TableIden;

#[derive(FromRow)]
#[enum_def(table_name = "GameLink")]
pub struct GameLink {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub url: String,
    pub description: String,
}

impl TableIden for GameLinkIden {
    const TABLE: Self = Self::Table;
}
