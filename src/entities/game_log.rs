use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::{postgres::types::PgInterval, FromRow};

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

#[derive(FromRow)]
pub struct GameLog {
    pub datetime: NaiveDateTime,
    pub time: PgInterval,
}
