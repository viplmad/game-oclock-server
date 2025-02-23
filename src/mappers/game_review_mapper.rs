use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};

use crate::entities::{GameWithFinish, GameWithLog};
use crate::models::{
    DurationDef, FinishDTO, GameDTO, GameFinishedReviewDTO, GamePlayedReviewDTO, GameStatus,
    LogDTO, StreakDTO,
};

impl From<GameWithLog> for GamePlayedReviewDTO {
    fn from(game: GameWithLog) -> Self {
        Self {
            game: GameDTO {
                id: game.id,
                user_id: game.user_id,
                title: game.title,
                edition: game.edition,
                release_date: game.release_date,
                base_game_id: game.base_game_id,
                cover_url: game.cover_url,
                added_datetime: game.added_datetime,
                updated_datetime: game.updated_datetime,
                status: GameStatus::try_from(game.status)
                    .expect("Status is not within valid range"),
                rating: u32::try_from(game.rating).expect("Rating is not positive"),
                notes: game.notes,
            },
            first_played: false,
            longest_streak: StreakDTO {
                start_date: NaiveDate::default(),
                end_date: NaiveDate::default(),
                days: 0,
            },
            longest_session: LogDTO::default(),
            first_session: LogDTO {
                start_datetime: DateTime::<Utc>::MAX_UTC,
                end_datetime: DateTime::default(),
                device_id: None,
                time: DurationDef::default(),
            },
            last_session: LogDTO {
                start_datetime: DateTime::<Utc>::MIN_UTC,
                end_datetime: DateTime::default(),
                device_id: None,
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
            game: GameDTO {
                id: game.id,
                user_id: game.user_id,
                title: game.title,
                edition: game.edition,
                release_date: game.release_date,
                base_game_id: game.base_game_id,
                cover_url: game.cover_url,
                added_datetime: game.added_datetime,
                updated_datetime: game.updated_datetime,
                status: GameStatus::try_from(game.status)
                    .expect("Status is not within valid range"),
                rating: u32::try_from(game.rating).expect("Rating is not positive"),
                notes: game.notes,
            },
            total_finished: 0,
            total_finished_grouped: HashMap::<u32, u32>::new(),
            first_finished: false,
            first_finish: FinishDTO {
                datetime: DateTime::<Utc>::MAX_UTC,
                status: GameStatus::LowPriority,
                device_id: None,
            },
            last_finish: FinishDTO {
                datetime: DateTime::<Utc>::MIN_UTC,
                status: GameStatus::LowPriority,
                device_id: None,
            },
            finishes: vec![],
        }
    }
}
