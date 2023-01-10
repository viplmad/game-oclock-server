use std::str::FromStr;

use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::FromRow;

use super::{FieldIden, FieldType, Search, TableIden};

pub type UserSearch = Search<UserIden>;

#[derive(Clone, Copy, Iden)]
#[iden = "User"]
pub enum UserIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "username"]
    Username,
    #[iden = "password"]
    Password,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}

impl TableIden for UserIden {
    const TABLE: Self = Self::Table;
}

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl FromStr for FieldIden<UserIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(UserIden::Id, FieldType::Integer)),
            "name" => Ok(FieldIden::new(UserIden::Username, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(UserIden::AddedDateTime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                UserIden::UpdatedDateTime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
