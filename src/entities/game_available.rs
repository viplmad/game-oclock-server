use sea_query::Iden;

pub const AVAILABLE_DATE_ALIAS: &str = "available_date";

#[derive(Iden)]
#[iden = "GameAvailable"]
pub enum GameAvailableIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "platform_id"]
    PlatformId,
    #[iden = "added_date"]
    AddedDate,
}
