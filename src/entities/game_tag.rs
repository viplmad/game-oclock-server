use sea_query::Iden;

#[derive(Iden)]
#[iden = "GameTag"]
pub enum GameTagIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "tag_id"]
    TagId,
}
