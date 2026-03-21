use sea_query::enum_def;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
#[enum_def(table_name = "ExternalMedia")]
pub struct ExternalMedia {
    pub media_id: Uuid,
    pub external_source: String,
    pub external_id: String,
    pub primary: bool,
}
