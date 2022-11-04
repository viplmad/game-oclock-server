use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::FromRow;

#[derive(Iden)]
#[iden = "Game"]
pub enum GameIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "name"]
    Name,
    #[iden = "edition"]
    Edition,
    #[iden = "release_year"]
    ReleaseYear,
    #[iden = "cover_filename"]
    CoverFilename,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}

#[derive(FromRow)]
pub struct Game {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub edition: String,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub status: i16,
    pub rating: i32,
    pub notes: String,
    pub save_folder: String,
    pub screenshot_folder: String,
    pub backup: bool,
}
