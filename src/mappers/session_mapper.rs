use uuid::Uuid;

use crate::entities::{MediaSession, MediaSessionWithTime};
use crate::models::{DurationDef, MediaStatus, SessionDTO};

impl From<MediaSessionWithTime> for SessionDTO {
    fn from(session: MediaSessionWithTime) -> Self {
        Self {
            start_datetime: session.start_date,
            end_datetime: session.end_date,
            device_id: session.device_id,
            group_id: session.group_id,
            started: session.started,
            finished_status: session
                .finished_status
                .map(|v| MediaStatus::try_from(v).expect("Status is not within valid range")),
            time: DurationDef::from(session.query_time.clone()),
            added_datetime: session.added_datetime,
            updated_datetime: session.updated_datetime,
        }
    }
}

impl From<SessionDTO> for MediaSession {
    fn from(session: SessionDTO) -> Self {
        Self {
            user_id: Uuid::default(),
            media_id: Uuid::default(),
            start_date: session.start_datetime,
            end_date: session.end_datetime,
            device_id: session.device_id,
            group_id: session.group_id,
            started: session.started,
            finished_status: session.finished_status.map(|v| i16::from(v)),
            added_datetime: session.added_datetime,
            updated_datetime: session.updated_datetime,
        }
    }
}
