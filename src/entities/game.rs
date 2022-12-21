use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;

use super::{FieldIden, FieldType, GameUserInfoIden, Search, TableIden};

pub type GameSearch = Search<GameIden>;

pub const QUERY_DATE_ALIAS: &str = "query_date";

#[derive(Clone, Copy, Iden)]
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

impl TableIden for GameIden {
    const TABLE: Self = Self::Table;
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

#[derive(FromRow)]
pub struct GameWithDate {
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
    pub query_date: NaiveDate,
}

impl FromStr for FieldIden<GameIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(GameIden::Id, FieldType::Integer)),
            "name" => Ok(FieldIden::new(GameIden::Name, FieldType::String)),
            "edition" => Ok(FieldIden::new(GameIden::Edition, FieldType::String)),
            "release_year" => Ok(FieldIden::new(GameIden::ReleaseYear, FieldType::Integer)),
            "cover_filename" => Ok(FieldIden::new(GameIden::CoverFilename, FieldType::String)),
            "status" => Ok(FieldIden::new(
                GameUserInfoIden::Status,
                FieldType::GameStatus,
            )),
            "rating" => Ok(FieldIden::new(GameUserInfoIden::Rating, FieldType::Integer)),
            "notes" => Ok(FieldIden::new(GameUserInfoIden::Notes, FieldType::String)),
            "save_folder" => Ok(FieldIden::new(
                GameUserInfoIden::SaveFolder,
                FieldType::String,
            )),
            "screenshot_folder" => Ok(FieldIden::new(
                GameUserInfoIden::ScreenshotFolder,
                FieldType::String,
            )),
            "backup" => Ok(FieldIden::new(GameUserInfoIden::Backup, FieldType::Boolean)),
            "added_datetime" => Ok(FieldIden::new(GameIden::AddedDateTime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                GameIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
