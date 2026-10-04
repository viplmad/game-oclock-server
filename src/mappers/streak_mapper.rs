use crate::entities::{MediaSessionDate, MediaSessionStreak};
use crate::models::{SessionDateDTO, SessionStreakDTO};

impl From<MediaSessionStreak> for SessionStreakDTO {
    fn from(streak: MediaSessionStreak) -> Self {
        Self {
            start_date: streak.start_date,
            end_date: streak.end_date,
            days: u32::try_from(streak.days).expect("Days is not positive"),
            media_ids: streak.media_ids,
            device_ids: streak.device_ids,
        }
    }
}

impl From<MediaSessionDate> for SessionDateDTO {
    fn from(streak: MediaSessionDate) -> Self {
        Self {
            start_date: streak.start_date,
            end_date: streak.end_date,
            date: streak.date,
        }
    }
}
