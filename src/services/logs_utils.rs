use std::{cmp::Ordering, collections::HashMap};

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

use crate::models::{DurationDef, GameLogDTO, GameStreakDTO, GamesStreakDTO};

pub(super) fn fill_total_time_by_month(
    total_time_by_month_map: &mut HashMap<u32, DurationDef>,
    start_datetime: NaiveDateTime,
    time: DurationDef,
) {
    let month = start_datetime.month();
    fill_single_total_time_grouped(total_time_by_month_map, month, time);
}

pub(super) fn fill_total_time_by_week(
    total_time_by_week_map: &mut HashMap<u32, DurationDef>,
    start_datetime: NaiveDateTime,
    time: DurationDef,
) {
    let week = start_datetime.iso_week().week();
    fill_single_total_time_grouped(total_time_by_week_map, week, time);
}

pub(super) fn fill_total_time_by_weekday(
    total_time_by_weekday_map: &mut HashMap<u32, DurationDef>,
    start_datetime: NaiveDateTime,
    time: DurationDef,
) {
    let weekday = start_datetime.weekday().number_from_monday();
    fill_single_total_time_grouped(total_time_by_weekday_map, weekday, time);
}

pub(super) fn fill_total_time_by_hour(
    total_time_by_hour_map: &mut HashMap<u32, DurationDef>,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) {
    // If log spans differents hours
    let mut temp_time = start_datetime.time();
    let end_time = end_datetime.time();
    while temp_time.hour() < end_time.hour() {
        let hour = temp_time.hour();

        let next_time_at_next_hour = NaiveTime::from_hms_opt(hour + 1, 0, 0);
        if let Some(next_time) = next_time_at_next_hour {
            let remaining_time_micros = (next_time - temp_time).num_microseconds();
            if let Some(micros) = remaining_time_micros {
                let remaining_time = DurationDef::microseconds(micros);
                fill_single_total_time_grouped(total_time_by_hour_map, hour, remaining_time);
            }

            temp_time = next_time;
        }
    }
    if end_time.hour() == temp_time.hour() && end_time.minute() != temp_time.minute() {
        let remaining_time_micros = (end_time - temp_time).num_microseconds();
        if let Some(micros) = remaining_time_micros {
            let remianing_time = DurationDef::microseconds(micros);
            let hour = temp_time.hour();
            fill_single_total_time_grouped(total_time_by_hour_map, hour, remianing_time);
        }
    }
}

pub(super) fn merge_total_time_grouped(
    total_time_grouped_map: &mut HashMap<u32, DurationDef>,
    game_total_time_grouped: &HashMap<u32, DurationDef>,
) {
    for (group, time) in game_total_time_grouped {
        fill_single_total_time_grouped(total_time_grouped_map, group.clone(), time.clone());
    }
}

fn fill_single_total_time_grouped(
    total_time_grouped_map: &mut HashMap<u32, DurationDef>,
    group: u32,
    time: DurationDef,
) {
    match total_time_grouped_map.get(&group) {
        Some(group_total_time) => {
            // Continue the group total
            let added_time = DurationDef::microseconds(group_total_time.micros + time.micros);
            total_time_grouped_map.insert(group, added_time);
        }
        None => {
            // Start group total
            total_time_grouped_map.insert(group, time);
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
            match start_datetime.date().cmp(&previous_date) {
                Ordering::Equal => {
                    // Continued the streak
                    last_streak.start_date = start_datetime.date();
                    last_streak.days += 1;
                }
                Ordering::Less => {
                    // Lost the streak, start a new one
                    streaks.push(GameStreakDTO {
                        start_date: start_datetime.date(),
                        end_date: end_datetime.date(),
                        days: 1,
                    });
                }
                Ordering::Greater => (),
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
            match start_datetime.date().cmp(&previous_date) {
                Ordering::Equal => {
                    // Continued the streak
                    if !last_streak.games_ids.contains(&game_id_clone) {
                        last_streak.games_ids.push(game_id_clone);
                    }
                    last_streak.start_date = start_datetime.date();
                    last_streak.days += 1;
                }
                Ordering::Less => {
                    // Lost the streak, start a new one
                    streaks.push(GamesStreakDTO {
                        games_ids: vec![game_id_clone],
                        start_date: start_datetime.date(),
                        end_date: end_datetime.date(),
                        days: 1,
                    });
                }
                Ordering::Greater => {
                    // Already on a streak day, add game if necessary
                    if !last_streak.games_ids.contains(&game_id_clone) {
                        last_streak.games_ids.push(game_id_clone);
                    }
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
