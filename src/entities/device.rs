use std::str::FromStr;

use chrono::NaiveDateTime;
use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

use super::{FieldIden, FieldType, Search, TableIden};

pub type DeviceSearch = Search<DeviceIden>;

#[derive(FromRow)]
#[enum_def(table_name = "Device")]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub icon_url: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

impl TableIden for DeviceIden {
    const TABLE: Self = Self::Table;
}

impl FromStr for FieldIden<DeviceIden> {
    type Err = ();

    fn from_str(field: &str) -> Result<Self, Self::Err> {
        match field {
            "id" => Ok(FieldIden::new(DeviceIden::Id, FieldType::String)),
            "name" => Ok(FieldIden::new(DeviceIden::Name, FieldType::String)),
            "icon_url" => Ok(FieldIden::new(DeviceIden::IconUrl, FieldType::String)),
            "added_datetime" => Ok(FieldIden::new(
                DeviceIden::AddedDatetime,
                FieldType::DateTime,
            )),
            "updated_datetime" => Ok(FieldIden::new(
                DeviceIden::UpdatedDatetime,
                FieldType::DateTime,
            )),
            _ => Err(()),
        }
    }
}
