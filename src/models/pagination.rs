use serde::Serialize;
use utoipa::ToSchema;

use super::ModelInfo;

#[derive(Serialize, ToSchema)]
pub struct SearchResultDTO<T>
where
    T: ModelInfo,
{
    pub data: Vec<T>,
    pub page: u64,
    pub size: u64,
}
