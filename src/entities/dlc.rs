use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;

use crate::errors::MappingError;

use super::{FieldIden, FieldType, Query};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Iden)]
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

pub type DLCQuery = Query<DLCIden>;

impl FromStr for FieldIden<DLCIden> {
    type Err = MappingError;

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden {
                iden: DLCIden::Id,
                _type: FieldType::Integer,
            }),
            "name" => Ok(FieldIden {
                iden: DLCIden::Name,
                _type: FieldType::String,
            }),
            "base_game_id" => Ok(FieldIden {
                iden: DLCIden::BaseGameId,
                _type: FieldType::Integer,
            }),
            "release_year" => Ok(FieldIden {
                iden: DLCIden::ReleaseYear,
                _type: FieldType::Integer,
            }),
            "cover_filename" => Ok(FieldIden {
                iden: DLCIden::CoverFilename,
                _type: FieldType::String,
            }),
            "added_datetime" => Ok(FieldIden {
                iden: DLCIden::AddedDateTime,
                _type: FieldType::DateTime,
            }),
            "updated_datetime" => Ok(FieldIden {
                iden: DLCIden::UpdatedDateTime,
                _type: FieldType::DateTime,
            }),
            _ => Err(MappingError(String::from("Field does not exist"))), // TODO
        }
    }
}

#[derive(FromRow)]
pub struct DLC {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub base_game_id: Option<i32>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

#[derive(FromRow)]
pub struct DLCAvailable {
    pub id: i32,
    pub user_id: i32,
    pub available_date: NaiveDate,
    pub name: String,
    pub base_game_id: Option<i32>,
    pub release_year: Option<i32>,
    pub cover_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}
