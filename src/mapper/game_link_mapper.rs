use crate::entities::GameLink;
use crate::models::GameLinkDTO;

impl From<GameLink> for GameLinkDTO {
    fn from(link: GameLink) -> Self {
        Self {
            url: link.url,
            description: link.description,
        }
    }
}

impl From<GameLinkDTO> for GameLink {
    fn from(link: GameLinkDTO) -> Self {
        Self {
            url: link.url,
            description: link.description,
        }
    }
}
