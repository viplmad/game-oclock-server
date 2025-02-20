use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, GameUserInfoIden, Search, TableIden};

pub type GameSearch = Search<GameIden>;

pub const QUERY_DATE_ALIAS: &str = "query_date";

#[derive(FromRow)]
#[enum_def(table_name = "Game")]
pub struct Game {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<NaiveDate>,
    pub base_game_id: Option<Uuid>,
    pub cover_url: Option<String>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}

impl TableIden for GameIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct GameWithUserInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<NaiveDate>,
    pub base_game_id: Option<Uuid>,
    pub cover_url: Option<String>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
    pub status: i16,
    pub rating: i16,
    pub notes: String,
}

#[derive(FromRow)]
pub struct GameWithUserInfoWithDate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub edition: String,
    pub release_date: Option<NaiveDate>,
    pub base_game_id: Option<Uuid>,
    pub cover_url: Option<String>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
    pub status: i16,
    pub rating: i16,
    pub notes: String,
    pub query_date: NaiveDate,
}

impl From<&GameWithUserInfo> for Game {
    fn from(game: &GameWithUserInfo) -> Self {
        Self {
            id: game.id,
            user_id: game.user_id,
            title: game.title.clone(),
            edition: game.edition.clone(),
            release_date: game.release_date,
            base_game_id: game.base_game_id,
            cover_url: game.cover_url.clone(),
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
        }
    }
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
            "added_datetime" => Ok(FieldIden::new(GameIden::AddedDatetime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                GameIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
