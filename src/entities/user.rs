use std::str::FromStr;

use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type UserSearch = Search<UserIden>;

#[derive(FromRow)]
#[enum_def(table_name = "User")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub admin: bool,
    pub added_datetime: DateTime<Utc>,
    pub updated_datetime: DateTime<Utc>,
}

impl TableIden for UserIden {
    const TABLE: Self = Self::Table;
}

impl FromStr for FieldIden<UserIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(UserIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(UserIden::Username, FieldType::String)),
            "admin" => Ok(FieldIden::new(UserIden::Admin, FieldType::Boolean)),
            "added_datetime" => Ok(FieldIden::new(UserIden::AddedDatetime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                UserIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
