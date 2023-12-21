use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDateTime};

use crate::models::{DurationDef, GameStreakDTO, GamesStreakDTO};

pub fn fill_sum_game_by_month(
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

pub fn fill_game_streaks(
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

pub fn fill_streaks(
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
