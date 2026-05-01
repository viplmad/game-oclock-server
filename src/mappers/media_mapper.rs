use uuid::Uuid;

use crate::entities::{ExternalMedia, Media, MediaState, MediaStateWithExternal, MediaWithState};
use crate::models::{
    ExternalMediaIdDTO, MediaDTO, MediaDataDTO, MediaStateDTO, MediaStatus, MediaType,
};

impl From<MediaWithState> for MediaDTO {
    fn from(media: MediaWithState) -> Self {
        Self {
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
        }
    }
}

impl From<Media> for MediaDataDTO {
    fn from(media: Media) -> Self {
        Self {
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
        }
    }
}

impl From<MediaState> for MediaStateDTO {
    fn from(state: MediaState) -> Self {
        Self {
            status: MediaStatus::try_from(state.status).expect("Status is not within valid range"),
            rating: u32::try_from(state.rating).expect("Rating is not positive"),
            notes: state.notes,
            added_datetime: state.added_datetime,
            updated_datetime: state.updated_datetime,
        }
    }
}

impl From<&MediaStateWithExternal> for MediaStateDTO {
    fn from(state: &MediaStateWithExternal) -> Self {
        Self {
            status: MediaStatus::try_from(state.status).expect("Status is not within valid range"),
            rating: u32::try_from(state.rating).expect("Rating is not positive"),
            notes: state.notes.clone(),
            added_datetime: state.added_datetime,
            updated_datetime: state.updated_datetime,
        }
    }
}

impl From<ExternalMedia> for ExternalMediaIdDTO {
    fn from(external: ExternalMedia) -> Self {
        Self {
            source: external.external_source,
            id: external.external_id,
        }
    }
}

impl From<MediaDataDTO> for Media {
    fn from(media: MediaDataDTO) -> Self {
        Self {
            id: media.id,
            title: media.title,
            edition: media.edition,
            release_date: media.release_date,
            genres: media.genres,
            series: media.series,
            image_url: media.image_url,
            kind: String::from(media.kind),
            parent_id: media.parent_id,
            parent_order: media
                .parent_order
                .map(|v| i32::try_from(v).expect("Order is not within valid range")),
            added_datetime: media.added_datetime,
            updated_datetime: media.updated_datetime,
        }
    }
}

impl From<MediaStateDTO> for MediaState {
    fn from(state: MediaStateDTO) -> Self {
        Self {
            user_id: Uuid::default(),
            media_id: Uuid::default(),
            status: i16::from(state.status),
            rating: i16::try_from(state.rating).expect("Rating is not within valid range"),
            notes: state.notes,
            added_datetime: state.added_datetime,
            updated_datetime: state.updated_datetime,
        }
    }
}
