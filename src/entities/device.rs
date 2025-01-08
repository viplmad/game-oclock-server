use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type DeviceSearch = Search<DeviceIden>;

#[derive(Clone, Copy, Iden)]
#[iden = "Device"]
pub enum DeviceIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "name"]
    Name,
    #[iden = "icon_filepath"]
    IconFilepath,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}

impl TableIden for DeviceIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon_filepath: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

#[derive(FromRow)]
pub struct DeviceWithDate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon_filepath: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub query_date: NaiveDate,
}

impl FromStr for FieldIden<DeviceIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(DeviceIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(DeviceIden::Name, FieldType::String)),
            "icon_filepath" => Ok(FieldIden::new(
                DeviceIden::IconFilepath,
                FieldType::String,
            )),
            "added_datetime" => Ok(FieldIden::new(
                DeviceIden::AddedDateTime,
                FieldType::DateTime,
            )),
            "updated_datetime" => Ok(FieldIden::new(
                DeviceIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
