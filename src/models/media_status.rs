use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum MediaStatus {
    #[default]
    Planning,
    InProgress,
    Paused,
    Dropped,
    Completed,
}
