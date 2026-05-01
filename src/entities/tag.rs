use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{AggregateSearch, ColIden, FieldIden, FieldType, ListSearch, TableIden};

pub type TagListSearch = ListSearch<TagIden>;
pub type TagAggregateSearch = AggregateSearch<TagIden>;

#[derive(FromRow)]
#[enum_def(table_name = "Tag")]
pub struct Tag {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub added_datetime: DateTime<FixedOffset>,
    pub updated_datetime: DateTime<FixedOffset>,
}

impl TableIden for TagIden {
    const TABLE: Self = Self::Table;
}

impl FromStr for FieldIden<TagIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::Col(ColIden::new(TagIden::Id, FieldType::String))),
            "name" => Ok(FieldIden::Col(ColIden::new(
                TagIden::Name,
                FieldType::String,
            ))),
            "added_datetime" => Ok(FieldIden::Col(ColIden::new(
                TagIden::AddedDatetime,
                FieldType::DateTime,
            ))),
            "updated_datetime" => Ok(FieldIden::Col(ColIden::new(
                TagIden::UpdatedDatetime,
                FieldType::DateTime,
            ))),
            _ => Err(()),
        }
    }
}
