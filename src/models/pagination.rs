use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SearchResultDTO<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub size: u64,
}
