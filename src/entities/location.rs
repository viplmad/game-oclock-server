use std::str::FromStr;

use chrono::{DateTime, Utc};
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
    pub image_url: Option<String>,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}

impl TableIden for LocationIden {
    const TABLE: Self = Self::Table;
}

impl FromStr for FieldIden<LocationIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(LocationIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(LocationIden::Name, FieldType::String)),
            "image_url" => Ok(FieldIden::new(LocationIden::ImageUrl, FieldType::String)),
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
