use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};

use crate::entities::{GameWithFinish, GameWithLog};
use crate::models::{
    DurationDef, FinishDTO, GameFinishedReviewDTO, GamePlayedReviewDTO, GameStatus, LogDTO,
    StreakDTO,
};

impl From<GameWithLog> for GamePlayedReviewDTO {
    fn from(game: GameWithLog) -> Self {
        Self {
            id: game.id.to_string(),
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id.map(|id| id.to_string()),
            cover_filename: None, // TODO extract filename from url
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            first_played: false,
            longest_streak: StreakDTO {
                start_date: NaiveDate::default(),
                end_date: NaiveDate::default(),
                days: 0,
            },
            longest_session: LogDTO::default(),
            first_session: LogDTO {
                start_datetime: NaiveDateTime::MAX,
                end_datetime: NaiveDateTime::default(),
                device_id: String::default(),
                time: DurationDef::default(),
            },
            last_session: LogDTO {
                start_datetime: NaiveDateTime::MIN,
                end_datetime: NaiveDateTime::default(),
                device_id: String::default(),
                time: DurationDef::default(),
            },
            total_sessions: 0,
            total_time: DurationDef::default(),
            total_time_by_month: HashMap::<u32, DurationDef>::new(),
            total_time_by_week: HashMap::<u32, DurationDef>::new(),
            total_time_by_weekday: HashMap::<u32, DurationDef>::new(),
            total_time_by_hour: HashMap::<u32, DurationDef>::new(),
            streaks: vec![],
            sessions: vec![],
        }
    }
}

impl From<GameWithFinish> for GameFinishedReviewDTO {
    fn from(game: GameWithFinish) -> Self {
        Self {
            id: game.id.to_string(),
            title: game.title,
            edition: game.edition,
            release_date: game.release_date,
            base_game_id: game.base_game_id.map(|id| id.to_string()),
            cover_filename: None, // TODO extract filename from url
            cover_url: game.cover_url,
            added_datetime: game.added_datetime,
            updated_datetime: game.updated_datetime,
            status: GameStatus::try_from(game.status).expect("Status was not within valid range"),
            rating: game.rating,
            notes: game.notes,
            total_finished: 0,
            total_finished_grouped: HashMap::<u32, i32>::new(),
            first_finished: false,
            first_finish: FinishDTO {
                date: NaiveDate::MAX,
                status: GameStatus::LowPriority,
                device_id: String::default(),
            },
            last_finish: FinishDTO {
                date: NaiveDate::MIN,
                status: GameStatus::LowPriority,
                device_id: String::default(),
            },
            finishes: vec![],
        }
    }
}
