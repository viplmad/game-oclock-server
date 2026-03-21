use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct OrderDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
}
