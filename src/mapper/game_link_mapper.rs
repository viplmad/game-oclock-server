use crate::entities::Link;
use crate::models::LinkDTO;

impl From<Link> for LinkDTO {
    fn from(link: Link) -> Self {
        Self {
            url: link.url,
            description: link.description,
        }
    }
}

impl From<LinkDTO> for Link {
    fn from(link: LinkDTO) -> Self {
        Self {
            url: link.url,
            description: link.description,
        }
    }
}
