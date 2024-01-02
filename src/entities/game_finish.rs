use chrono::NaiveDate;
use sea_query::Iden;
use sqlx::FromRow;
use uuid::Uuid;

use super::TableIden;

#[derive(Iden)]
#[iden = "GameFinish"]
pub enum GameFinishIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "date"]
    Date,
}

impl TableIden for GameFinishIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct GameFinish {
    pub game_id: Uuid,
    pub date: NaiveDate,
}
