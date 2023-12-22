use std::collections::HashMap;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::PgPool;

use crate::entities::{GameSearch, GameWithLog};
use crate::errors::ApiErrors;
use crate::models::{
    DurationDef, GameLogDTO, GameStreakDTO, GameWithLogDTO, GameWithLogPageResult, GameWithLogsDTO,
    GameWithLogsExtendedDTO, GamesLogDTO, GamesStreakDTO, GamesWithLogsExtendedDTO, SearchDTO,
};
use crate::repository::game_with_log_repository;

use super::base::{
    check_optional_start_end, check_start_end, handle_get_list_paged_result, handle_query_mapping,
    handle_result, optional_start_end_to_datetime, start_end_to_datetime,
};
use super::logs_utils;

pub async fn search_first_played_games(
    pool: &PgPool,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithLogPageResult, ApiErrors> {
    check_optional_start_end(start_date, end_date)?;

    let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
    let search = handle_query_mapping::<GameWithLogDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_log_repository::search_first_by_start_datetime_between(
        pool,
        user_id,
        start_datetime,
        end_datetime,
        search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}

pub async fn search_last_played_games(
    pool: &PgPool,
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithLogPageResult, ApiErrors> {
    check_optional_start_end(start_date, end_date)?;

    let (start_datetime, end_datetime) = optional_start_end_to_datetime(start_date, end_date);
    let search = handle_query_mapping::<GameWithLogDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_log_repository::search_last_by_start_datetime_between(
        pool,
        user_id,
        start_datetime,
        end_datetime,
        search,
    )
    .await;
    handle_get_list_paged_result(find_result)
}

pub async fn get_game_with_logs(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameWithLogsDTO>, ApiErrors> {
    let entity_list = find_game_with_logs_between(pool, user_id, start_date, end_date).await?;

    let game_with_logs = create_list(entity_list);
    Ok(game_with_logs)
}

pub async fn get_detailed_game_with_logs(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<GamesWithLogsExtendedDTO, ApiErrors> {
    let entity_list = find_game_with_logs_between(pool, user_id, start_date, end_date).await?;

    let game_with_logs = create_detailed_list(entity_list);
    Ok(game_with_logs)
}

/*pub async fn get_sum_game_logs_grouped_by_month(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<HashMap<u32, DurationDef>, ApiErrors> {
    check_start_end(start_date, end_date)?;
    games_service::exists_game(pool, user_id, game_id).await?;

    let entity_list = find_game_with_logs_between(pool, user_id, start_date, end_date).await?;
    let mut sum_by_month_map = HashMap::<u32, DurationDef>::new();
    for log in logs {
        let start_datetime = log.datetime;
        let time = DurationDef::from(log.query_time);

        logs_utils::fill_sum_game_by_month(&mut sum_by_month_map, start_datetime, time);
    }

    Ok(sum_by_month_map)
}*/

async fn find_game_with_logs_between(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameWithLog>, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
    let find_result = game_with_log_repository::find_all_by_start_datetime_between(
        pool,
        user_id,
        start_datetime,
        end_datetime,
    )
    .await;
    handle_result::<Vec<GameWithLog>, GameWithLogDTO>(find_result)
}

fn create_list(game_with_logs: Vec<GameWithLog>) -> Vec<GameWithLogsDTO> {
    let mut map = HashMap::<String, GameWithLogsDTO>::new();

    for game_with_log in game_with_logs {
        let game_id = game_with_log.id.to_string();

        let log = GameLogDTO::from(&game_with_log);
        match map.get_mut(&game_id) {
            Some(game) => {
                game.logs.push(log);
            }
            None => {
                let mut game = GameWithLogsDTO::from(game_with_log);
                game.logs.push(log);

                map.insert(game_id, game);
            }
        }
    }

    map.into_values().collect()
}

fn create_detailed_list(game_with_logs: Vec<GameWithLog>) -> GamesWithLogsExtendedDTO {
    let mut map = HashMap::<String, GameWithLogsExtendedDTO>::new();

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
        let time = log.time.clone();

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
            let new_game = GameWithLogsExtendedDTO::from(game_with_log);
            map.insert(game_id.clone(), new_game);
        }
        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.

        game.total_time = DurationDef::microseconds(game.total_time.micros + time.micros);
        logs_utils::fill_sum_game_by_month(
            &mut game.total_time_grouped,
            start_datetime,
            time.clone(),
        );
        logs_utils::fill_game_streaks(&mut game.streaks, start_datetime, end_datetime);

        fill_game_sessions(
            &mut game.sessions,
            start_datetime,
            end_datetime,
            time.clone(),
        );
        game.total_sessions =
            i32::try_from(game.sessions.len()).expect("Count was not within valid range");

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

        if let Some(last_session) = game.sessions.last() {
            // Found longer session
            let last_session_time = last_session.time.clone();
            if last_session_time.micros > game.longest_session.time.micros {
                game.longest_session = GameLogDTO {
                    start_datetime: last_session.start_datetime,
                    end_datetime: last_session.end_datetime,
                    time: last_session_time.clone(),
                };
            }

            // Found longer global session
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

    let num_games = i32::try_from(map.len()).expect("Count was not within valid range");
    let total_sessions = map.iter().map(|(_, g)| g.total_sessions).sum();

    GamesWithLogsExtendedDTO {
        count: num_games,
        longest_streak,
        longest_session,
        total_sessions,
        total_time,
        total_time_grouped: total_sum_by_month,
        games_with_logs: map.into_values().collect(),
    }
}

fn fill_game_sessions(
    sessions: &mut Vec<GameLogDTO>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
    time: DurationDef,
) {
    match sessions.last_mut() {
        Some(last_session) => {
            let last_session_time = last_session.time.clone();
            let last_session_start_datetime = last_session.start_datetime;
            // Check if this is part of a continuous log (ended on midnight and kept playing)
            if
            // If the date of the current log is on the previous day of the last log
            start_datetime.date() == (last_session_start_datetime.date() - Duration::days(1))
                // and the end time of the current log is midnight
                && end_datetime.time() == NaiveTime::MIN
                // and the start time of the last log is midnight
                && last_session_start_datetime.time() == NaiveTime::MIN
            {
                last_session.start_datetime = start_datetime;
                last_session.time =
                    DurationDef::microseconds(last_session_time.micros + time.micros);
            } else {
                sessions.push(GameLogDTO {
                    start_datetime,
                    end_datetime,
                    time,
                })
            }
        }
        None => {
            // Start first session
            sessions.push(GameLogDTO {
                start_datetime,
                end_datetime,
                time,
            })
        }
    }
}
