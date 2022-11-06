use chrono::NaiveDate;
use sea_query::Iden;
use sqlx::FromRow;

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

#[derive(FromRow)]
pub struct GameFinish {
    pub user_id: i32,
    pub game_id: i32,
    pub date: NaiveDate,
}