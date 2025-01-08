use sea_query::Iden;

#[derive(Iden)]
#[iden = "GameGenre"]
pub enum GameGenreIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
    #[iden = "genre_id"]
    GenreId,
}
