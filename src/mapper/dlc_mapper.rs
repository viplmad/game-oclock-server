use sqlx::types::Uuid;

use crate::entities::{DLCWithDate, DLC};
use crate::models::{DLCAvailableDTO, DLCDTO};

impl From<DLC> for DLCDTO {
    fn from(dlc: DLC) -> Self {
        Self {
            id: dlc.id.to_string(),
            name: dlc.name,
            base_game_id: dlc.base_game_id.map(|id| id.to_string()),
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            cover_url: None,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
        }
    }
}

impl From<DLCDTO> for DLC {
    fn from(dlc: DLCDTO) -> Self {
        Self {
            id: Uuid::parse_str(&dlc.id).expect("Id was not valid Uuid"),
            user_id: Uuid::default(),
            name: dlc.name,
            base_game_id: dlc
                .base_game_id
                .map(|id| Uuid::parse_str(&id).expect("Id was not valid Uuid")),
            release_year: dlc.release_year,
            cover_filename: dlc.cover_filename,
            added_datetime: dlc.added_datetime,
            updated_datetime: dlc.updated_datetime,
        }
    }
}

impl From<DLCWithDate> for DLCAvailableDTO {
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
            available_date: dlc.query_date,
        }
    }
}
