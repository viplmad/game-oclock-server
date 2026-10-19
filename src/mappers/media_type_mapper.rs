use crate::entities::{MEDIA_TYPE_GAME, MEDIA_TYPE_GAME_DEMO, MEDIA_TYPE_GAME_DLC};
use crate::models::MediaType;

impl TryFrom<String> for MediaType {
    type Error = ();

    fn try_from(status: String) -> Result<Self, Self::Error> {
        match status.as_str() {
            MEDIA_TYPE_GAME => Ok(MediaType::Game),
            MEDIA_TYPE_GAME_DLC => Ok(MediaType::GameDlc),
            MEDIA_TYPE_GAME_DEMO => Ok(MediaType::GameDemo),
            _ => Err(()),
        }
    }
}

impl From<MediaType> for String {
    fn from(status: MediaType) -> Self {
        match status {
            MediaType::Game => String::from(MEDIA_TYPE_GAME),
            MediaType::GameDlc => String::from(MEDIA_TYPE_GAME_DLC),
            MediaType::GameDemo => String::from(MEDIA_TYPE_GAME_DEMO),
        }
    }
}
