use sqlx::types::Uuid;

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
            id: Uuid::parse_str(&tag.id).expect("Id was not valid Uuid"),
            user_id: Uuid::default(), // TODO Possibly remove user_id field from entities
            name: tag.name,
            added_datetime: tag.added_datetime,
            updated_datetime: tag.updated_datetime,
        }
    }
}
