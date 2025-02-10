use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type LocationSearch = Search<LocationIden>;

#[derive(FromRow)]
#[enum_def(table_name = "Location")]
pub struct Location {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon_url: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl TableIden for LocationIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct LocationWithDate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon_url: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
    pub query_date: NaiveDate,
}

impl FromStr for FieldIden<LocationIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(LocationIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(LocationIden::Name, FieldType::String)),
            "icon_url" => Ok(FieldIden::new(LocationIden::IconUrl, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(
                LocationIden::AddedDatetime,
                FieldType::DateTime,
            )),
            "updated_datetime" => Ok(FieldIden::new(
                LocationIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
