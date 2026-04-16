use std::str::FromStr;

use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, ListSearch, TableIden};

pub type UserListSearch = ListSearch<UserIden>;

#[derive(FromRow)]
#[enum_def(table_name = "User")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub role: String,
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
            "role" => Ok(FieldIden::new(UserIden::Role, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(UserIden::AddedDatetime, FieldType::DateTime)),
            "updated_datetime" => Ok(FieldIden::new(
                UserIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
