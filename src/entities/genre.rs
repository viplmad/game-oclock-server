use std::str::FromStr;

use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type GenreSearch = Search<GenreIden>;

#[derive(FromRow)]
#[enum_def(table_name = "Genre")]
pub struct Genre {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}

impl TableIden for GenreIden {
    const TABLE: Self = Self::Table;
}

impl FromStr for FieldIden<GenreIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(GenreIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(GenreIden::Name, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(
                GenreIden::AddedDatetime,
                FieldType::DateTime,
            )),
            "updated_datetime" => Ok(FieldIden::new(
                GenreIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
