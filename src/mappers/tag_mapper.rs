use uuid::Uuid;

use crate::entities::Tag;
use crate::models::TagDTO;

impl From<Tag> for TagDTO {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id.to_string(),
            name: tag.name,
            added_datetime: tag.added_datetime,
            updated_datetime: tag.updated_datetime,
        }
    }
}

impl From<TagDTO> for Tag {
    fn from(tag: TagDTO) -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            name: tag.name,
            added_datetime: tag.added_datetime,
            updated_datetime: tag.updated_datetime,
        }
    }
}
