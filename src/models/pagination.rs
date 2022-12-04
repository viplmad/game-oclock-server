use serde::Serialize;
use utoipa::ToSchema;

use super::ModelName;

#[derive(Serialize, ToSchema)]
pub struct SearchResultDTO<T>
where
    T: ModelName,
{
    pub data: Vec<T>,
    pub page: u64,
    pub size: u64,
}
