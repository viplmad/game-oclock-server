use crate::entities::{DeviceWithSession, MediaWithStateWithSession};
use crate::models::{
    DeviceDTO, DeviceSessionDTO, DurationDef, ExternalMediaIdDTO, MediaDTO, MediaDataDTO,
    MediaSessionDTO, MediaStateDTO, MediaStatus, MediaType, SessionDTO,
};

impl From<MediaWithStateWithSession> for MediaSessionDTO {
    fn from(media: MediaWithStateWithSession) -> Self {
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
            session: SessionDTO {
                media_id: media.session_media_id,
                start_datetime: media.session_start_date.fixed_offset(),
                end_datetime: media.session_end_date.fixed_offset(),
                device_id: media.session_device_id,
                group_id: media.session_group_id,
                started: media.session_started,
                finished_status: media
                    .session_finished_status
                    .map(|v| MediaStatus::try_from(v).expect("Status is not within valid range")),
                time: DurationDef::from(media.query_time),
                added_datetime: media.session_added_datetime,
                updated_datetime: media.session_updated_datetime,
            },
        }
    }
}

impl From<DeviceWithSession> for DeviceSessionDTO {
    fn from(device: DeviceWithSession) -> Self {
        Self {
            device: DeviceDTO {
                id: device.id,
                name: device.name,
                image_url: device.image_url,
                added_datetime: device.added_datetime,
                updated_datetime: device.updated_datetime,
            },
            session: SessionDTO {
                media_id: device.session_media_id,
                start_datetime: device.session_start_date.fixed_offset(),
                end_datetime: device.session_end_date.fixed_offset(),
                device_id: device.session_device_id,
                group_id: device.session_group_id,
                started: device.session_started,
                finished_status: device
                    .session_finished_status
                    .map(|v| MediaStatus::try_from(v).expect("Status is not within valid range")),
                time: DurationDef::from(device.query_time),
                added_datetime: device.session_added_datetime,
                updated_datetime: device.session_updated_datetime,
            },
        }
    }
}
