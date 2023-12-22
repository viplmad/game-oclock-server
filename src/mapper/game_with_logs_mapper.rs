use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};

use crate::entities::GameWithLog;
use crate::models::{
    DurationDef, GameLogDTO, GameStatus, GameStreakDTO, GameWithLogDTO, GameWithLogsDTO,
    GameWithLogsExtendedDTO,
};

impl From<GameWithLog> for GameWithLogsDTO {
    fn from(game: GameWithLog) -> Self {
        Self {
            id: game.id.to_string(),
            name: game.name,
            edition: game.edition,
            release_year: game.release_year,
            cover_filename: game.cover_filename,
            cover_url: None,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            save_folder: game.save_folder,
            screenshot_folder: game.screenshot_folder,
            backup: game.backup,
            logs: vec![],
        }
    }
}

impl From<GameWithLog> for GameWithLogDTO {
    fn from(game: GameWithLog) -> Self {
        Self {
            id: game.id.to_string(),
            name: game.name,
            edition: game.edition,
            release_year: game.release_year,
            cover_filename: game.cover_filename,
            cover_url: None,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            save_folder: game.save_folder,
            screenshot_folder: game.screenshot_folder,
            backup: game.backup,
            log_start_datetime: game.log_start_datetime,
            log_end_datetime: game.log_end_datetime,
            log_time: DurationDef::from(game.log_time),
        }
    }
}

impl From<GameWithLog> for GameWithLogsExtendedDTO {
    fn from(game: GameWithLog) -> Self {
        Self {
            id: game.id.to_string(),
            name: game.name,
            edition: game.edition,
            release_year: game.release_year,
            cover_filename: game.cover_filename,
            cover_url: None,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            save_folder: game.save_folder,
            screenshot_folder: game.screenshot_folder,
            backup: game.backup,
            longest_streak: GameStreakDTO {
                start_date: NaiveDate::default(),
                end_date: NaiveDate::default(),
                days: 0,
            },
            longest_session: GameLogDTO {
                start_datetime: NaiveDateTime::default(),
                end_datetime: NaiveDateTime::default(),
                time: DurationDef::default(),
            },
            total_sessions: 0,
            total_time: DurationDef::default(),
            total_time_grouped: HashMap::<u32, DurationDef>::new(),
            streaks: vec![],
            sessions: vec![],
        }
    }
}
