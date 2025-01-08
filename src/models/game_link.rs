use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelInfo};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct GameLinkDTO {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Merge<NewGameLinkDTO> for GameLinkDTO {
    fn merge(self, other: NewGameLinkDTO) -> Self {
        Self {
            url: other.url,
            description: other.description,
        }
    }
}

impl ModelInfo for GameLinkDTO {
    const MODEL_NAME: &'static str = "Game link";
    const ID_FIELDS: &'static [&'static str] = &["game id", "url"];
    const UNIQUE_FIELDS: &'static [&'static str] = GameLinkDTO::ID_FIELDS;
}

#[derive(Deserialize, ToSchema)]
pub struct NewGameLinkDTO {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
