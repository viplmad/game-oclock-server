use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::FromRow;

#[derive(Iden)]
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

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}
