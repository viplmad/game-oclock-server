use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::entities::MediaWithStateWithSession;
use crate::models::{
    DurationDef, ExternalMediaIdDTO, MediaDTO, MediaRawDTO, MediaReviewDTO, MediaStateDTO,
    MediaStatus, MediaType, SessionDTO, StreakDTO,
};

impl From<MediaWithStateWithSession> for MediaReviewDTO {
    fn from(media: MediaWithStateWithSession) -> Self {
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
            first: false,
            longest_streak: StreakDTO {
                start_date: NaiveDate::default(),
                end_date: NaiveDate::default(),
                days: 0,
                devices_ids: vec![],
            },
            longest_session: SessionDTO::default(),
            first_session: SessionDTO {
                start_datetime: DateTime::<Utc>::MAX_UTC,
                end_datetime: DateTime::default(),
                device_id: None,
                group_id: Uuid::default(),
                started: false,
                finished_status: None,
                time: DurationDef::default(),
                added_datetime: DateTime::default(),
                updated_datetime: DateTime::default(),
            },
            last_session: SessionDTO {
                start_datetime: DateTime::<Utc>::MIN_UTC,
                end_datetime: DateTime::default(),
                device_id: None,
                group_id: Uuid::default(),
                started: false,
                finished_status: None,
                time: DurationDef::default(),
                added_datetime: DateTime::default(),
                updated_datetime: DateTime::default(),
            },
            total_sessions: 0,
            total_time: DurationDef::default(),
            total_time_by_month: HashMap::<u32, DurationDef>::new(),
            total_time_by_week: HashMap::<u32, DurationDef>::new(),
            total_time_by_weekday: HashMap::<u32, DurationDef>::new(),
            total_time_by_hour: HashMap::<u32, DurationDef>::new(),
            // Finished
            total_finished: 0,
            //total_finished_grouped: HashMap::<u32, u32>::new(),
            first_finished: false,
            first_finish: SessionDTO {
                start_datetime: DateTime::<Utc>::MAX_UTC,
                end_datetime: DateTime::default(),
                device_id: None,
                group_id: Uuid::default(),
                started: false,
                finished_status: None,
                time: DurationDef::default(),
                added_datetime: DateTime::default(),
                updated_datetime: DateTime::default(),
            },
            last_finish: SessionDTO {
                start_datetime: DateTime::<Utc>::MIN_UTC,
                end_datetime: DateTime::default(),
                device_id: None,
                group_id: Uuid::default(),
                started: false,
                finished_status: None,
                time: DurationDef::default(),
                added_datetime: DateTime::default(),
                updated_datetime: DateTime::default(),
            },
            //
            streaks: vec![],
            sessions: vec![],
        }
    }
}
