use sea_query::Iden;

use super::TableIden;

#[derive(Iden)]
#[iden = "GameUserInfo"]
pub enum GameUserInfoIden {
    Table,
    #[iden = "user_id"]
    UserId,
    #[iden = "game_id"]
    GameId,
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

impl TableIden for GameUserInfoIden {
    const TABLE: Self = Self::Table;
}
