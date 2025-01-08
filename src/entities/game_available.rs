use sea_query::Iden;

#[derive(Iden)]
#[iden = "GameAvailable"]
pub enum GameAvailableIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "location_id"]
    LocationId,
    #[iden = "date"]
    Date,
}
