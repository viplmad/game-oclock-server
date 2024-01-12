use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};

use crate::entities::{GameWithDate, GameWithLog};
use crate::models::{
    DurationDef, GameFinishedReviewDTO, GameLogDTO, GamePlayedReviewDTO, GameStatus, GameStreakDTO,
};

impl From<GameWithLog> for GamePlayedReviewDTO {
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
            first_played: false,
            first_session_start_datetime: NaiveDateTime::MAX,
            last_session_start_datetime: NaiveDateTime::MIN,
            streaks: vec![],
            sessions: vec![],
        }
    }
}

impl From<GameWithDate> for GameFinishedReviewDTO {
    fn from(game: GameWithDate) -> Self {
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
            total_finished: 0,
            total_finished_grouped: HashMap::<u32, i32>::new(),
            first_finished: false,
            first_finish_date: NaiveDate::MAX,
            last_finish_date: NaiveDate::MIN,
            finishes: vec![],
        }
    }
}
