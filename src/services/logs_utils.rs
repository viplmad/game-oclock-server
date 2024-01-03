use std::collections::HashMap;

use chrono::{Datelike, Duration, Months, NaiveDate, NaiveDateTime, NaiveTime};

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

pub(super) fn fill_total_sessions_by_month(
    total_sessions_by_month_map: &mut HashMap<u32, i32>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) {
    let end_month = end_datetime.month();
    let end_year = end_datetime.year();

    let mut date = start_datetime.date();
    while date.year() < end_year || (date.year() == end_year && date.month() <= end_month) {
        let month = date.month();
        fill_single_total_sessions_by_month(total_sessions_by_month_map, month, 1);
        date = date.checked_add_months(Months::new(1)).unwrap(); // Safe unwrap: assume exists next month.
    }
}

fn fill_single_total_sessions_by_month(
    total_sessions_by_month_map: &mut HashMap<u32, i32>,
    month: u32,
    amount: i32,
) {
    match total_sessions_by_month_map.get(&month) {
        Some(month_total_sessions) => {
            // Continue the month total
            let added_total = month_total_sessions + amount;
            total_sessions_by_month_map.insert(month, added_total);
        }
        None => {
            // Start month total
            total_sessions_by_month_map.insert(month, amount);
        }
    }
}

pub(super) fn fill_total_by_release_year(
    total_by_relese_year_map: &mut HashMap<i32, i32>,
    release_year: &Option<i32>,
) {
    if let Some(y) = release_year {
        let year = y.clone();
        match total_by_relese_year_map.get(&year) {
            Some(total_played) => {
                // Continue the total
                let added_total = total_played + 1;
                total_by_relese_year_map.insert(year, added_total);
            }
            None => {
                // Start total
                total_by_relese_year_map.insert(year, 1);
            }
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
