use chrono::NaiveDateTime;
use sea_query::Iden;
use sqlx::FromRow;

#[derive(Iden)]
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

#[derive(FromRow)]
pub struct Tag {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub added_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}
