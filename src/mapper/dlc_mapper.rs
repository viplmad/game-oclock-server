use crate::entities::{DLCAvailable, DLC};
use crate::models::{DLCAvailableDTO, DLCDTO};

impl From<DLC> for DLCDTO {
    fn from(dlc: DLC) -> Self {
        Self {
            id: dlc.id,
            user_id: dlc.user_id,
            name: dlc.name,
            base_game_id: dlc.base_game_id,
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
        }
    }
}

impl From<DLCDTO> for DLC {
    fn from(dlc: DLCDTO) -> Self {
        Self {
            id: dlc.id,
            user_id: dlc.user_id,
            name: dlc.name,
            base_game_id: dlc.base_game_id,
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
        }
    }
}

impl From<DLCAvailable> for DLCAvailableDTO {
    fn from(dlc: DLCAvailable) -> Self {
        Self {
            id: dlc.id,
            user_id: dlc.user_id,
            available_date: dlc.available_date,
            name: dlc.name,
            base_game_id: dlc.base_game_id,
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
        }
    }
}
