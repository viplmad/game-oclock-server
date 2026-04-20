use crate::entities::MediaSessionStreak;
use crate::models::SessionStreakDTO;

impl From<MediaSessionStreak> for SessionStreakDTO {
    fn from(streak: MediaSessionStreak) -> Self {
        Self {
            start_date: streak.start_date,
            end_date: streak.end_date,
            days: u32::try_from(streak.days).expect("Days is not positive"),
            devices_ids: vec![],
        }
    }
}
