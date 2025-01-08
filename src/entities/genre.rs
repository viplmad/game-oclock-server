use std::str::FromStr;

use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type GenreSearch = Search<GenreIden>;

#[derive(Clone, Copy, Iden)]
#[iden = "Genre"]
pub enum GenreIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "name"]
    Name,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}

impl TableIden for GenreIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct Genre {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl FromStr for FieldIden<GenreIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(GenreIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(GenreIden::Name, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(GenreIden::AddedDateTime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                GenreIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
