use crate::entities::{MediaWithStateWithTag, TagWithTag};
use crate::models::{
    ExternalMediaIdDTO, MediaDTO, MediaDataDTO, MediaStateDTO, MediaStatus, MediaTagDTO, MediaType,
    TagDTO, TagMediaDTO, TaggedDTO,
};

impl From<MediaWithStateWithTag> for MediaTagDTO {
    fn from(media: MediaWithStateWithTag) -> Self {
        Self {
            media: MediaDTO {
                media: MediaDataDTO {
                    id: media.id,
                    kind: MediaType::try_from(media.kind).expect("Type is not within valid range"),
                    title: media.title,
                    edition: media.edition,
                    release_date: media.release_date,
                    genres: media.genres,
                    series: media.series,
                    image_url: media.image_url,
                    parent_id: media.parent_id,
                    parent_order: media
                        .parent_order
                        .map(|v| u32::try_from(v).expect("Order is not positive")),
                    added_datetime: media.added_datetime,
                    updated_datetime: media.updated_datetime,
                },
                external: ExternalMediaIdDTO {
                    source: media.external_source,
                    id: media.external_id,
                },
                state: MediaStateDTO {
                    status: MediaStatus::try_from(media.state_status)
                        .expect("Status is not within valid range"),
                    rating: u32::try_from(media.state_rating).expect("Rating is not positive"),
                    notes: media.state_notes,
                    added_datetime: media.state_added_datetime,
                    updated_datetime: media.state_updated_datetime,
                },
            },
            tagged: TaggedDTO {
                order: media
                    .tag_order
                    .map(|v| u32::try_from(v).expect("Order is not positive")),
                added_datetime: media.tag_added_datetime,
                updated_datetime: media.tag_updated_datetime,
            },
        }
    }
}

impl From<TagWithTag> for TagMediaDTO {
    fn from(tag: TagWithTag) -> Self {
        Self {
            tag: TagDTO {
                id: tag.id,
                name: tag.name,
                added_datetime: tag.added_datetime,
                updated_datetime: tag.updated_datetime,
            },
            tagged: TaggedDTO {
                order: tag
                    .tag_order
                    .map(|v| u32::try_from(v).expect("Order is not positive")),
                added_datetime: tag.tag_added_datetime,
                updated_datetime: tag.tag_updated_datetime,
            },
        }
    }
}
