use sea_query::Iden;
use sqlx::FromRow;
use uuid::Uuid;

use super::TableIden;

#[derive(Iden)]
#[iden = "GameLink"]
pub enum GameLinkIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "url"]
    Url,
    #[iden = "description"]
    Description,
}

impl TableIden for GameLinkIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct GameLink {
    pub game_id: Uuid,
    pub url: String,
    pub description: String,
}
