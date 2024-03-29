use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type DLCSearch = Search<DLCIden>;

#[derive(Clone, Copy, Iden)]
#[iden = "DLC"]
pub enum DLCIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "name"]
    Name,
    #[iden = "base_game_id"]
    BaseGameId,
    #[iden = "release_year"]
    ReleaseYear,
    #[iden = "cover_filename"]
    CoverFilename,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}

impl TableIden for DLCIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct DLC {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub base_game_id: Option<Uuid>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

#[derive(FromRow)]
pub struct DLCWithDate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub base_game_id: Option<Uuid>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub query_date: NaiveDate,
}

impl FromStr for FieldIden<DLCIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(DLCIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(DLCIden::Name, FieldType::String)),
            "base_game_id" => Ok(FieldIden::new(DLCIden::BaseGameId, FieldType::String)),
            "release_year" => Ok(FieldIden::new(DLCIden::ReleaseYear, FieldType::Integer)),
            "cover_filename" => Ok(FieldIden::new(DLCIden::CoverFilename, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(DLCIden::AddedDateTime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                DLCIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
