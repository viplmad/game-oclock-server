use chrono::{NaiveDate, NaiveDateTime};
use sea_query::Iden;
use sqlx::FromRow;

#[derive(Iden)]
#[iden = "Platform"]
pub enum PlatformIden {
    Table,
    #[iden = "id"]
    Id,
    #[iden = "user_id"]
    UserId,
    #[iden = "name"]
    Name,
    #[iden = "type"]
    Type,
    #[iden = "icon_filename"]
    IconFilename,
    #[iden = "added_datetime"]
    AddedDateTime,
    #[iden = "updated_datetime"]
    UpdatedDateTime,
}

#[derive(FromRow)]
pub struct Platform {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    #[sqlx(rename = "type")] // Fix to use type reserved name
    pub ptype: Option<i16>,
    pub icon_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

#[derive(FromRow)]
pub struct PlatformAvailable {
    pub id: i32,
    pub user_id: i32,
    #[sqlx(rename = "added_date")] // TODO Prefer rename in sql
    pub available_date: NaiveDate,
    pub name: String,
    #[sqlx(rename = "type")] // Fix to use type reserved name
    pub ptype: Option<i16>,
    pub icon_filename: Option<String>,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}
