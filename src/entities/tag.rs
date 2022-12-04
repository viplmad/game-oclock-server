use std::str::FromStr;

use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::FromRow;

use super::{FieldIden, FieldType, Search, TableIden};

pub type TagSearch = Search<TagIden>;

#[derive(Clone, Copy, Iden)]
#[iden = "Tag"]
pub enum TagIden {
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

impl TableIden for TagIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct Tag {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl FromStr for FieldIden<TagIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(TagIden::Id, FieldType::Integer)),
            "name" => Ok(FieldIden::new(TagIden::Name, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(TagIden::AddedDateTime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                TagIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
