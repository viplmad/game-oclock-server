use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{Merge, ModelInfo};

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct LinkDTO {
    pub url: String,
    pub description: String,
}

impl Merge<NewLinkDTO> for LinkDTO {
    fn merge(self, other: NewLinkDTO) -> Self {
        Self {
            url: other.url,
            description: other.description.unwrap_or(self.description),
        }
    }
}

impl ModelInfo for LinkDTO {
    const MODEL_NAME: &'static str = "Game link";
    const ID_FIELDS: &'static [&'static str] = &["game id", "url"];
    const UNIQUE_FIELDS: &'static [&'static str] = LinkDTO::ID_FIELDS;
}

#[derive(Deserialize, ToSchema)]
pub struct NewLinkDTO {
    pub url: String,
    pub description: Option<String>,
}
