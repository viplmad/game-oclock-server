use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime};

use crate::models::{DurationDef, GameLogDTO, GameStreakDTO, GamesStreakDTO};

pub(super) fn fill_sum_game_by_month(
    sum_by_month_map: &mut HashMap<u32, DurationDef>,
    start_datetime: NaiveDateTime,
    time: DurationDef,
) {
    let month = start_datetime.month();
    match sum_by_month_map.get(&month) {
        Some(month_sum) => {
            // Continue the month sum
            let added_time = DurationDef::microseconds(month_sum.micros + time.micros);
            sum_by_month_map.insert(month, added_time);
        }
        None => {
            // Start month sum
            sum_by_month_map.insert(month, time);
        }
    }
}

pub(super) fn fill_total_sessions_by_month(
    total_sessions_by_month_map: &mut HashMap<u32, i32>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) {
    let start_month = start_datetime.month();
    let end_month = end_datetime.month();

    for month in start_month..end_month + 1 {
        fill_single_total_sessions_by_month(total_sessions_by_month_map, month);
    }
}

fn fill_single_total_sessions_by_month(
    total_sessions_by_month_map: &mut HashMap<u32, i32>,
    month: u32,
) {
    match total_sessions_by_month_map.get(&month) {
        Some(month_total_sessions) => {
            // Continue the month total
            let added_total = month_total_sessions + 1;
            total_sessions_by_month_map.insert(month, added_total);
        }
        None => {
            // Start month total
            total_sessions_by_month_map.insert(month, 1);
        }
    }
}

pub(super) fn fill_game_streaks(
    streaks: &mut Vec<GameStreakDTO>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) {
    match streaks.last_mut() {
        Some(last_streak) => {
            let previous_date = last_streak.start_date - Duration::days(1);
            if start_datetime.date() == previous_date {
                // Continued the streak
                last_streak.start_date = start_datetime.date();
                last_streak.days += 1;
            } else if start_datetime.date() < previous_date {
                // Lost the streak, start a new one
                streaks.push(GameStreakDTO {
                    start_date: start_datetime.date(),
                    end_date: end_datetime.date(),
                    days: 1,
                });
            }
        }
        None => {
            // Start first streak
            streaks.push(GameStreakDTO {
                start_date: start_datetime.date(),
                end_date: end_datetime.date(),
                days: 1,
            })
        }
    }
}

pub(super) fn fill_game_sessions(
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

pub(super) fn fill_streaks(
    streaks: &mut Vec<GamesStreakDTO>,
    game_id: &str,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) {
    let game_id_clone = String::from(game_id);
    match streaks.last_mut() {
        Some(last_streak) => {
            let previous_date = last_streak.start_date - Duration::days(1);
            if start_datetime.date() == previous_date {
                // Continued the streak
                if !last_streak.games_ids.contains(&game_id_clone) {
                    last_streak.games_ids.push(game_id_clone);
                }
                last_streak.start_date = start_datetime.date();
                last_streak.days += 1;
            } else if start_datetime.date() < previous_date {
                // Lost the streak, start a new one
                streaks.push(GamesStreakDTO {
                    games_ids: vec![game_id_clone],
                    start_date: start_datetime.date(),
                    end_date: end_datetime.date(),
                    days: 1,
                });
            } else {
                // Already on a streak day, add game if necessary
                if !last_streak.games_ids.contains(&game_id_clone) {
                    last_streak.games_ids.push(game_id_clone);
                }
            }
        }
        None => {
            // Start first streak
            streaks.push(GamesStreakDTO {
                games_ids: vec![game_id_clone],
                start_date: start_datetime.date(),
                end_date: end_datetime.date(),
                days: 1,
            });
        }
    }
}
