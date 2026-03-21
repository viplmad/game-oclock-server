use std::{cmp::Ordering, collections::HashMap};

use chrono::{DateTime, Datelike, Duration, NaiveTime, Timelike, Utc};
use uuid::Uuid;

use crate::models::{DurationDef, MediaIdsStreakDTO, SessionDTO, StreakDTO};

pub(super) fn fill_total_time_by_month(
    total_time_by_month_map: &mut HashMap<u32, DurationDef>,
    start_datetime: DateTime<Utc>,
    time: DurationDef,
) {
    let month = start_datetime.month();
    fill_single_total_time_grouped(total_time_by_month_map, month, time);
}

pub(super) fn fill_total_time_by_week(
    total_time_by_week_map: &mut HashMap<u32, DurationDef>,
    start_datetime: DateTime<Utc>,
    time: DurationDef,
) {
    let week = start_datetime.iso_week().week();
    fill_single_total_time_grouped(total_time_by_week_map, week, time);
}

pub(super) fn fill_total_time_by_weekday(
    total_time_by_weekday_map: &mut HashMap<u32, DurationDef>,
    start_datetime: DateTime<Utc>,
    time: DurationDef,
) {
    let weekday = start_datetime.weekday().number_from_monday();
    fill_single_total_time_grouped(total_time_by_weekday_map, weekday, time);
}

pub(super) fn fill_total_time_by_hour(
    total_time_by_hour_map: &mut HashMap<u32, DurationDef>,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) {
    // If session spans differents hours
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
    media_total_time_grouped: &HashMap<u32, DurationDef>,
) {
    for (group, time) in media_total_time_grouped {
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

pub(super) fn fill_total_optional_map(total_map: &mut HashMap<u32, u32>, value: &Option<u32>) {
    if let Some(v) = value {
        fill_total_map(total_map, v.clone());
    }
}

pub(super) fn fill_total_map(total_map: &mut HashMap<u32, u32>, value: u32) {
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

pub(super) fn fill_media_streaks(streaks: &mut Vec<StreakDTO>, session: SessionDTO) {
    match streaks.last_mut() {
        Some(last_streak) => {
            let previous_date = last_streak.start_date - Duration::days(1);
            match session.start_datetime.date_naive().cmp(&previous_date) {
                Ordering::Equal => {
                    // Continued the streak
                    last_streak.start_date = session.start_datetime.date_naive();
                    last_streak.days += 1;
                    if let Some(device_id) = session.device_id {
                        crate::vec_utils::push_if_not_contained(
                            &mut last_streak.devices_ids,
                            device_id,
                        );
                    }
                }
                Ordering::Less => {
                    // Lost the streak, start a new one
                    streaks.push(StreakDTO {
                        start_date: session.start_datetime.date_naive(),
                        end_date: session.end_datetime.date_naive(),
                        days: 1,
                        devices_ids: session.device_id.map_or(vec![], |id| vec![id]),
                    });
                }
                Ordering::Greater => (),
            }
        }
        None => {
            // Start first streak
            streaks.push(StreakDTO {
                start_date: session.start_datetime.date_naive(),
                end_date: session.end_datetime.date_naive(),
                days: 1,
                devices_ids: session.device_id.map_or(vec![], |id| vec![id]),
            })
        }
    }
}

pub(super) fn fill_media_sessions(sessions: &mut Vec<SessionDTO>, session: SessionDTO) {
    match sessions.last_mut() {
        Some(last_session) => {
            let last_session_time = last_session.time.clone();
            let last_session_start_datetime = last_session.start_datetime;
            // Check if this is part of a continuous session (ended on midnight and kept on)
            if
            // If the date of the current session is on the previous day of the last session
            session.start_datetime.date_naive() == (last_session_start_datetime.date_naive() - Duration::days(1))
                // and the end time of the current session is midnight
                && session.end_datetime.time() == NaiveTime::MIN
                // and the start time of the last session is midnight
                && last_session_start_datetime.time() == NaiveTime::MIN
            {
                last_session.start_datetime = session.start_datetime;
                last_session.time =
                    DurationDef::microseconds(last_session_time.micros + session.time.micros);
            } else {
                sessions.push(SessionDTO {
                    start_datetime: session.start_datetime,
                    end_datetime: session.end_datetime,
                    device_id: session.device_id.clone(),
                    time: session.time,
                    group_id: session.group_id,
                    started: session.started,
                    finished_status: session.finished_status.clone(),
                    added_datetime: session.added_datetime,
                    updated_datetime: session.updated_datetime,
                })
            }
        }
        None => {
            // Start first session
            sessions.push(SessionDTO {
                start_datetime: session.start_datetime,
                end_datetime: session.end_datetime,
                device_id: session.device_id.clone(),
                time: session.time,
                group_id: session.group_id,
                started: session.started,
                finished_status: session.finished_status.clone(),
                added_datetime: session.added_datetime,
                updated_datetime: session.updated_datetime,
            })
        }
    }
}

pub(super) fn fill_streaks(
    streaks: &mut Vec<MediaIdsStreakDTO>,
    media_id: &Uuid,
    session: SessionDTO,
) {
    let media_id_clone = media_id.clone();
    match streaks.last_mut() {
        Some(last_streak) => {
            let previous_date = last_streak.streak.start_date - Duration::days(1);
            match session.start_datetime.date_naive().cmp(&previous_date) {
                Ordering::Equal => {
                    // Continued the streak
                    crate::vec_utils::push_if_not_contained(
                        &mut last_streak.medias_ids,
                        media_id_clone,
                    );
                    last_streak.streak.start_date = session.start_datetime.date_naive();
                    last_streak.streak.days += 1;
                    if let Some(device_id) = session.device_id {
                        crate::vec_utils::push_if_not_contained(
                            &mut last_streak.streak.devices_ids,
                            device_id,
                        );
                    }
                }
                Ordering::Less => {
                    // Lost the streak, start a new one
                    streaks.push(MediaIdsStreakDTO {
                        medias_ids: vec![media_id_clone],
                        streak: StreakDTO {
                            start_date: session.start_datetime.date_naive(),
                            end_date: session.end_datetime.date_naive(),
                            days: 1,
                            devices_ids: session.device_id.map_or(vec![], |id| vec![id]),
                        },
                    });
                }
                Ordering::Greater => {
                    // Already on a streak day
                    // Add media if necessary
                    crate::vec_utils::push_if_not_contained(
                        &mut last_streak.medias_ids,
                        media_id_clone,
                    );
                    // Add device if necessary
                    if let Some(device_id) = session.device_id {
                        crate::vec_utils::push_if_not_contained(
                            &mut last_streak.streak.devices_ids,
                            device_id,
                        );
                    }
                }
            }
        }
        None => {
            // Start first streak
            streaks.push(MediaIdsStreakDTO {
                medias_ids: vec![media_id_clone],
                streak: StreakDTO {
                    start_date: session.start_datetime.date_naive(),
                    end_date: session.end_datetime.date_naive(),
                    days: 1,
                    devices_ids: session.device_id.map_or(vec![], |id| vec![id]),
                },
            });
        }
    }
}

pub(super) fn fill_total_finished_by_month(
    total_finished_by_month_map: &mut HashMap<u32, u32>,
    finish_datetime: DateTime<Utc>,
) {
    let month = finish_datetime.month();
    fill_single_total_finished_by_month(total_finished_by_month_map, month, 1);
}

pub(super) fn merge_total_finished_by_month(
    total_finished_by_month_map: &mut HashMap<u32, u32>,
    media_total_finished_by_month: &HashMap<u32, u32>,
) {
    for (month, amount) in media_total_finished_by_month {
        fill_single_total_finished_by_month(
            total_finished_by_month_map,
            month.clone(),
            amount.clone(),
        );
    }
}

fn fill_single_total_finished_by_month(
    total_finished_by_month_map: &mut HashMap<u32, u32>,
    month: u32,
    amount: u32,
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
