use crate::entities::Tag;
use crate::models::TagDTO;

impl From<Tag> for TagDTO {
    fn from(tag: Tag) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            added_datetime: tag.added_datetime,
            updated_datetime: tag.updated_datetime,
        }
    }
}

impl From<TagDTO> for Tag {
    fn from(tag: TagDTO) -> Self {
        Self {
            id: tag.id,
            user_id: -1, // TODO Possibly remove user_id field from entities
            name: tag.name,
            added_datetime: tag.added_datetime,
            updated_datetime: tag.updated_datetime,
        }
    }
}
