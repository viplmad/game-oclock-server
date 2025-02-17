use uuid::Uuid;

use crate::entities::GameLink;
use crate::models::LinkDTO;

impl From<GameLink> for LinkDTO {
    fn from(link: GameLink) -> Self {
        Self {
            url: link.url,
            description: link.description,
        }
    }
}

impl From<LinkDTO> for GameLink {
    fn from(link: LinkDTO) -> Self {
        Self {
            user_id: Uuid::default(),
            game_id: Uuid::default(),
            url: link.url,
            description: link.description,
        }
    }
}
