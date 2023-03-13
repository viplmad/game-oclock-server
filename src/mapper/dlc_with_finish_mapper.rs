use crate::entities::DLCWithDate;
use crate::models::DLCWithFinishDTO;

impl From<DLCWithDate> for DLCWithFinishDTO {
    fn from(dlc: DLCWithDate) -> Self {
        Self {
            id: dlc.id.to_string(),
            name: dlc.name,
            base_game_id: dlc.base_game_id.map(|id| id.to_string()),
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            cover_url: None,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
            finish_date: dlc.query_date,
        }
    }
}
