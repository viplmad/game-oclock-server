use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;
use uuid::Uuid;

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
    #[iden = "title"]
    Title,
    #[iden = "edition"]
    Edition,
    #[iden = "release_date"]
    ReleaseDate,
    #[iden = "base_game_id"]
    BaseGameId,
    #[iden = "cover_url"]
    CoverUrl,
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
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<NaiveDate>,
    pub base_game_id: Option<Uuid>,
    pub cover_url: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub status: i16,
    pub rating: i16,
    pub notes: String,
}

#[derive(FromRow)]
pub struct GameWithDate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<NaiveDate>,
    pub base_game_id: Option<Uuid>,
    pub cover_url: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub status: i16,
    pub rating: i32,
    pub notes: String,
    pub query_date: NaiveDate,
}

impl FromStr for FieldIden<GameIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(GameIden::Id, FieldType::String)),
            "title" => Ok(FieldIden::new(GameIden::Title, FieldType::String)),
            "edition" => Ok(FieldIden::new(GameIden::Edition, FieldType::String)),
            "release_date" => Ok(FieldIden::new(GameIden::ReleaseDate, FieldType::Integer)),
            "base_game_id" => Ok(FieldIden::new(GameIden::BaseGameId, FieldType::String)),
            "cover_url" => Ok(FieldIden::new(GameIden::CoverUrl, FieldType::String)),
            "status" => Ok(FieldIden::new(
                GameUserInfoIden::Status,
                FieldType::GameStatus,
            )),
            "rating" => Ok(FieldIden::new(GameUserInfoIden::Rating, FieldType::Integer)),
            "notes" => Ok(FieldIden::new(GameUserInfoIden::Notes, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(GameIden::AddedDateTime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                GameIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
