use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{AggregateSearch, ColIden, FieldIden, FieldType, ListSearch, TableIden};

pub type LocationListSearch = ListSearch<LocationIden>;
pub type LocationAggregateSearch = AggregateSearch<LocationIden>;

#[derive(FromRow)]
#[enum_def(table_name = "Location")]
pub struct Location {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
}

impl TableIden for LocationIden {
    const TABLE: Self = Self::Table;
}

impl FromStr for FieldIden<LocationIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::Col(ColIden::new(
                LocationIden::Id,
                FieldType::String,
            ))),
            "name" => Ok(FieldIden::Col(ColIden::new(
                LocationIden::Name,
                FieldType::String,
            ))),
            "image_url" => Ok(FieldIden::Col(ColIden::new(
                LocationIden::ImageUrl,
                FieldType::String,
            ))),
            "added_datetime" => Ok(FieldIden::Col(ColIden::new(
                LocationIden::AddedDatetime,
                FieldType::DateTime,
            ))),
            "updated_datetime" => Ok(FieldIden::Col(ColIden::new(
                LocationIden::UpdatedDatetime,
                FieldType::DateTime,
            ))),
            _ => Err(()),
        }
    }
}
