use crate::entities::{LocationWithAvailable, MediaWithStateWithAvailable};
use crate::models::{
    AvailableDTO, ExternalMediaIdDTO, LocationAvailableDTO, LocationDTO, MediaAvailableDTO,
    MediaDTO, MediaRawDTO, MediaStateDTO, MediaStatus, MediaType,
};

impl From<MediaWithStateWithAvailable> for MediaAvailableDTO {
    fn from(media: MediaWithStateWithAvailable) -> Self {
        Self {
            media: MediaDTO {
                media: MediaRawDTO {
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
            available: AvailableDTO {
                datetime: media.available_date,
                added_datetime: media.available_added_datetime,
                updated_datetime: media.available_updated_datetime,
            },
        }
    }
}

impl From<LocationWithAvailable> for LocationAvailableDTO {
    fn from(location: LocationWithAvailable) -> Self {
        Self {
            location: LocationDTO {
                id: location.id,
                name: location.name,
                image_url: location.image_url,
                added_datetime: location.added_datetime,
                updated_datetime: location.updated_datetime,
            },
            available: AvailableDTO {
                datetime: location.available_date,
                added_datetime: location.available_added_datetime,
                updated_datetime: location.available_updated_datetime,
            },
        }
    }
}
