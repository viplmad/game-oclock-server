use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};

use crate::models::{DurationDef, GameLogDTO, GameStreakDTO, GamesStreakDTO};

pub(super) fn fill_total_time_by_month(
    total_time_by_month_map: &mut HashMap<u32, DurationDef>,
    start_datetime: NaiveDateTime,
    time: DurationDef,
) {
    let month = start_datetime.month();
    fill_single_total_time_by_month(total_time_by_month_map, month, time);
}

pub(super) fn merge_total_time_by_month(
    total_time_by_month_map: &mut HashMap<u32, DurationDef>,
    game_total_time_by_month: &HashMap<u32, DurationDef>,
) {
    for (month, time) in game_total_time_by_month {
        fill_single_total_time_by_month(total_time_by_month_map, month.clone(), time.clone());
    }
}

fn fill_single_total_time_by_month(
    total_time_by_month_map: &mut HashMap<u32, DurationDef>,
    month: u32,
    time: DurationDef,
) {
    match total_time_by_month_map.get(&month) {
        Some(month_total_time) => {
            // Continue the month total
            let added_time = DurationDef::microseconds(month_total_time.micros + time.micros);
            total_time_by_month_map.insert(month, added_time);
        }
        None => {
            // Start month total
            total_time_by_month_map.insert(month, time);
        }
    }
}

pub(super) fn fill_total_optional_map(total_map: &mut HashMap<i32, i32>, value: &Option<i32>) {
    if let Some(v) = value {
        fill_total_map(total_map, v.clone());
    }
}

pub(super) fn fill_total_map(total_map: &mut HashMap<i32, i32>, value: i32) {
    match total_map.get(&value) {
        Some(total) => {
            // Continue the total
            let added_total = total + 1;
            total_map.insert(value, added_total);
        }
        None => {
            // Start total
            total_map.insert(value, 1);
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

pub(super) fn fill_total_finished_by_month(
    total_finished_by_month_map: &mut HashMap<u32, i32>,
    finish_date: NaiveDate,
) {
    let month = finish_date.month();
    fill_single_total_finished_by_month(total_finished_by_month_map, month, 1);
}

pub(super) fn merge_total_finished_by_month(
    total_finished_by_month_map: &mut HashMap<u32, i32>,
    game_total_finished_by_month: &HashMap<u32, i32>,
) {
    for (month, amount) in game_total_finished_by_month {
        fill_single_total_finished_by_month(
            total_finished_by_month_map,
            month.clone(),
            amount.clone(),
        );
    }
}

fn fill_single_total_finished_by_month(
    total_finished_by_month_map: &mut HashMap<u32, i32>,
    month: u32,
    amount: i32,
) {
    match total_finished_by_month_map.get(&month) {
        Some(month_total_finished) => {
            // Continue the month total
            let added_total = month_total_finished + amount;
            total_finished_by_month_map.insert(month, added_total);
        }
        None => {
            // Start month total
            total_finished_by_month_map.insert(month, amount);
        }
    }
}

pub(super) fn fill_game_finishes(finishes: &mut Vec<NaiveDate>, finish_date: NaiveDate) {
    finishes.push(finish_date);
}
