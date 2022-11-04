use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub enum GameStatus {
    LowPriority,
    NextUp,
    Playing,
    Played,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::LowPriority
    }
}
