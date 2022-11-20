use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelName, PlatformType};

#[derive(Serialize, ToSchema)]
pub struct PlatformDTO {
    pub id: i32,
    pub name: String,
    // Fix to use type reserved name
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ptype: Option<PlatformType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_filename: Option<String>,
    #[schema(value_type = String)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub updated_datetime: NaiveDateTime,
}

impl Default for PlatformDTO {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::default(),
            ptype: None,
            icon_filename: None,
            added_datetime: NaiveDateTime::default(),
            updated_datetime: NaiveDateTime::default(),
        }
    }
}

impl Merge<NewPlatformDTO> for PlatformDTO {
    fn merge(self, other: NewPlatformDTO) -> Self {
        Self {
            id: self.id,
            name: other.name.unwrap_or(self.name),
            ptype: other.ptype,
            icon_filename: other.icon_filename,
            added_datetime: self.added_datetime,
            updated_datetime: self.updated_datetime,
        }
    }
}

impl ModelName for PlatformDTO {
    const MODEL_NAME: &'static str = "Platform";
    const ID_FIELDS: &'static [&'static str] = &["id"];
    const UNIQUE_FIELDS: &'static [&'static str] = &["name"];
}

#[derive(Deserialize, ToSchema)]
pub struct NewPlatformDTO {
    pub name: Option<String>,
    pub ptype: Option<PlatformType>,
    pub icon_filename: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct PlatformAvailableDTO {
    pub id: i32,
    #[schema(value_type = String)]
    pub available_date: NaiveDate,
    pub name: String,
    // Fix to use type reserved name
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ptype: Option<PlatformType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_filename: Option<String>,
    #[schema(value_type = String)]
    pub added_datetime: NaiveDateTime,
    #[schema(value_type = String)]
    pub updated_datetime: NaiveDateTime,
}

impl ModelName for PlatformAvailableDTO {
    const MODEL_NAME: &'static str = "Relation with Platform";
    const ID_FIELDS: &'static [&'static str] = &["id", "platform id"];
    const UNIQUE_FIELDS: &'static [&'static str] = PlatformAvailableDTO::ID_FIELDS;
}
