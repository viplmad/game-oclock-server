use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Default, Serialize, Deserialize, ToSchema)]
pub enum GameStatus {
    #[default]
    LowPriority,
    NextUp,
    Playing,
    Played,
}
