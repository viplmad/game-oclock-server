use sea_query::Iden;

#[derive(Iden)]
#[iden = "GameUserInfo"]
pub enum GameUserInfoIden {
    Table,
    #[iden = "game_id"]
    GameId,
    #[iden = "user_id"]
    UserId,
    #[iden = "status"]
    Status,
    #[iden = "rating"]
    Rating,
    #[iden = "notes"]
    Notes,
    #[iden = "save_folder"]
    SaveFolder,
    #[iden = "screenshot_folder"]
    ScreenshotFolder,
    #[iden = "backup"]
    Backup,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}
