use std::str::FromStr;

use chrono::NaiveDateTime;
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type TagSearch = Search<TagIden>;

#[derive(FromRow)]
#[enum_def(table_name = "Tag")]
pub struct Tag {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl TableIden for TagIden {
    const TABLE: Self = Self::Table;
}

impl FromStr for FieldIden<TagIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(TagIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(TagIden::Name, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(TagIden::AddedDatetime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                TagIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
