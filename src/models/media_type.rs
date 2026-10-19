use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Default, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum MediaType {
    #[default]
    Game,
    GameDlc,
    GameDemo,
}
