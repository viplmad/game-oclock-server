use std::collections::{HashMap, HashSet};

use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

use crate::entities::{GameLogWithTime, GameWithLog};
use crate::errors::ApiErrors;
use crate::models::{
    DurationDef, GameLogDTO, GameReviewDTO, GameStreakDTO, GamesLogDTO, GamesReviewDTO,
    GamesStreakDTO,
};

use super::{game_logs_service, game_with_logs_service, logs_utils};

pub async fn get_games_review(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<GamesReviewDTO, ApiErrors> {
    let game_with_logs =
        game_with_logs_service::find_game_with_logs_between(pool, user_id, start_date, end_date)
            .await?;

    let game_ids = game_with_logs
        .iter()
        .map(|game| game.id.to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();
    let first_logs =
        game_logs_service::find_first_game_logs_by_games(pool, user_id, game_ids).await?;

    let game_with_logs = build_review(game_with_logs, first_logs);
    Ok(game_with_logs)
}

fn build_review(
    game_with_logs: Vec<GameWithLog>,
    first_logs: Vec<GameLogWithTime>,
) -> GamesReviewDTO {
    let mut map = HashMap::<String, GameReviewDTO>::new();

    let mut total_time = DurationDef::default();
    let mut total_sum_by_month = HashMap::<u32, DurationDef>::new();
    let mut total_streaks: Vec<GamesStreakDTO> = vec![];
    let mut longest_streak = GamesStreakDTO {
        games_ids: vec![],
        start_date: NaiveDate::default(),
        end_date: NaiveDate::default(),
        days: 0,
    };
    let mut longest_session = GamesLogDTO {
        game_id: String::default(),
        start_datetime: NaiveDateTime::default(),
        end_datetime: NaiveDateTime::default(),
        time: DurationDef::default(),
    };

    for game_with_log in game_with_logs {
        let game_id = game_with_log.id.to_string();

        let log = GameLogDTO::from(&game_with_log);
        let start_datetime = log.start_datetime;
        let end_datetime = log.end_datetime;
        let time = log.time;

        total_time = DurationDef::microseconds(total_time.micros + time.micros);
        logs_utils::fill_sum_game_by_month(&mut total_sum_by_month, start_datetime, time.clone());
        logs_utils::fill_streaks(&mut total_streaks, &game_id, start_datetime, end_datetime);

        // Found longer global streak
        if let Some(last_streak) = total_streaks.last() {
            let last_streak_days = last_streak.days;
            if last_streak_days > longest_streak.days {
                longest_streak = GamesStreakDTO {
                    games_ids: last_streak.games_ids.clone(),
                    start_date: last_streak.start_date,
                    end_date: last_streak.end_date,
                    days: last_streak_days,
                }
            }
        }

        if !map.contains_key(&game_id) {
            let new_game = GameReviewDTO::from(game_with_log);
            map.insert(game_id.clone(), new_game);
        }
        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.
        fill_game_review(game, start_datetime, end_datetime, time);

        // Found longer global session
        if let Some(last_session) = game.sessions.last() {
            let last_session_time = last_session.time.clone();
            if last_session_time.micros > longest_session.time.micros {
                longest_session = GamesLogDTO {
                    game_id: game_id.clone(),
                    start_datetime: last_session.start_datetime,
                    end_datetime: last_session.end_datetime,
                    time: last_session_time.clone(),
                };
            }
        }
    }

    for first_log in first_logs {
        let game_id = first_log.game_id.to_string();

        let log = GameLogDTO::from(first_log);
        let first_start_datetime = log.start_datetime;

        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.

        if let Some(last_session) = game.sessions.last() {
            let start_datetime = last_session.start_datetime;
            if first_start_datetime == start_datetime {
                game.first_played = true;
            } else {
                game.first_played = false;
            }
        }
    }

    let mut total_played = 0;
    let mut total_sessions = 0;
    let mut total_first_played = 0;
    let mut total_sessions_by_month = HashMap::<u32, i32>::new();
    for game in map.values_mut() {
        game.total_sessions =
            i32::try_from(game.sessions.len()).expect("Count was not within valid range");

        total_played += 1;
        total_sessions += game.total_sessions;
        total_first_played += if game.first_played { 1 } else { 0 };

        for session in game.sessions.iter() {
            let start_datetime = session.start_datetime;
            let end_datetime = session.end_datetime;

            logs_utils::fill_total_sessions_by_month(
                &mut game.total_sessions_grouped,
                start_datetime,
                end_datetime,
            );

            logs_utils::fill_total_sessions_by_month(
                &mut total_sessions_by_month,
                start_datetime,
                end_datetime,
            );
        }
    }

    GamesReviewDTO {
        total_played,
        total_first_played,
        longest_streak,
        longest_session,
        total_sessions,
        total_sessions_grouped: total_sessions_by_month,
        total_time,
        total_time_grouped: total_sum_by_month,
        games: map.into_values().collect(),
    }
}

fn fill_game_review(
    game: &mut GameReviewDTO,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
    time: DurationDef,
) {
    // Fill total time
    game.total_time = DurationDef::microseconds(game.total_time.micros + time.micros);
    logs_utils::fill_sum_game_by_month(&mut game.total_time_grouped, start_datetime, time.clone());

    // Fill streaks
    logs_utils::fill_game_streaks(&mut game.streaks, start_datetime, end_datetime);

    // Found longer streak
    if let Some(last_streak) = game.streaks.last() {
        let last_streak_days = last_streak.days;
        if last_streak_days > game.longest_streak.days {
            game.longest_streak = GameStreakDTO {
                start_date: last_streak.start_date,
                end_date: last_streak.end_date,
                days: last_streak_days,
            }
        }
    }

    // Fill sessions
    logs_utils::fill_game_sessions(
        &mut game.sessions,
        start_datetime,
        end_datetime,
        time.clone(),
    );

    // Found longer session
    if let Some(last_session) = game.sessions.last() {
        let last_session_time = last_session.time.clone();
        if last_session_time.micros > game.longest_session.time.micros {
            game.longest_session = GameLogDTO {
                start_datetime: last_session.start_datetime,
                end_datetime: last_session.end_datetime,
                time: last_session_time.clone(),
            };
        }
    }
}
