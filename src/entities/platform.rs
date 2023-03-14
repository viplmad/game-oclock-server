use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type PlatformSearch = Search<PlatformIden>;

#[derive(Clone, Copy, Iden)]
#[iden = "Platform"]
pub enum PlatformIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "name"]
    Name,
    #[iden = "type"]
    Type,
    #[iden = "icon_filename"]
    IconFilename,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}

impl TableIden for PlatformIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct Platform {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    #[sqlx(rename = "type")] // Fix to use type reserved name
    pub ptype: Option<i16>,
    pub icon_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

#[derive(FromRow)]
pub struct PlatformWithDate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub query_date: NaiveDate,
    pub name: String,
    #[sqlx(rename = "type")] // Fix to use type reserved name
    pub ptype: Option<i16>,
    pub icon_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl FromStr for FieldIden<PlatformIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(PlatformIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(PlatformIden::Name, FieldType::String)),
            "type" => Ok(FieldIden::new(PlatformIden::Type, FieldType::PlatformType)),
            "icon_filename" => Ok(FieldIden::new(
                PlatformIden::IconFilename,
                FieldType::String,
            )),
            "added_datetime" => Ok(FieldIden::new(
                PlatformIden::AddedDateTime,
                FieldType::DateTime,
            )),
            "updated_datetime" => Ok(FieldIden::new(
                PlatformIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
