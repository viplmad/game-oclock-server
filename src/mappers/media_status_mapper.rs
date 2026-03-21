use crate::models::MediaStatus;

impl TryFrom<i16> for MediaStatus {
    type Error = ();

    fn try_from(status: i16) -> Result<Self, Self::Error> {
        match status {
            0 => Ok(MediaStatus::Planning),
            1 => Ok(MediaStatus::InProgress),
            2 => Ok(MediaStatus::Paused),
            3 => Ok(MediaStatus::Dropped),
            4 => Ok(MediaStatus::Completed),
            _ => Err(()),
        }
    }
}

impl From<MediaStatus> for i16 {
    fn from(status: MediaStatus) -> Self {
        match status {
            MediaStatus::Planning => 0,
            MediaStatus::InProgress => 1,
            MediaStatus::Paused => 2,
            MediaStatus::Dropped => 3,
            MediaStatus::Completed => 4,
        }
    }
}
