use std::collections::{HashMap, HashSet};

use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

use crate::entities::{GameFinish, GameLogWithTime, GameWithDate, GameWithLog};
use crate::errors::ApiErrors;
use crate::models::{
    DurationDef, GameFinishedReviewDTO, GameLogDTO, GamePlayedReviewDTO, GameStreakDTO,
    GamesFinishedReviewDTO, GamesLogDTO, GamesPlayedReviewDTO, GamesStreakDTO,
};

use super::{
    game_finishes_service, game_logs_service, game_with_finish_service, game_with_logs_service,
    logs_utils,
};

pub async fn get_played_games_review(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<GamesPlayedReviewDTO, ApiErrors> {
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

    let review = build_played_review(game_with_logs, first_logs);
    Ok(review)
}

pub async fn get_finished_games_review(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<GamesFinishedReviewDTO, ApiErrors> {
    let game_with_finishes = game_with_finish_service::find_game_with_finishes_between(
        pool, user_id, start_date, end_date,
    )
    .await?;

    let game_ids = game_with_finishes
        .iter()
        .map(|game| game.id.to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();
    let first_finishes =
        game_finishes_service::find_first_game_finishes_by_games(pool, user_id, game_ids).await?;

    let review = build_finished_review(game_with_finishes, first_finishes);
    Ok(review)
}

fn build_played_review(
    game_with_logs: Vec<GameWithLog>,
    first_logs: Vec<GameLogWithTime>,
) -> GamesPlayedReviewDTO {
    let mut map = HashMap::<String, GamePlayedReviewDTO>::new();

    let mut total_streaks: Vec<GamesStreakDTO> = vec![];
    let mut longest_streak = GamesStreakDTO {
        games_ids: vec![],
        start_date: NaiveDate::default(),
        end_date: NaiveDate::default(),
        days: 0,
    };

    // Fill logs map and global streaks
    for game_with_log in game_with_logs {
        let game_id = game_with_log.id.to_string();

        let log = GameLogDTO::from(&game_with_log);
        let start_datetime = log.start_datetime;
        let end_datetime = log.end_datetime;
        let time = log.time;

        // Fill global streaks
        logs_utils::fill_streaks(&mut total_streaks, &game_id, start_datetime, end_datetime);

        // Found longer global streak
        if let Some(new_longest_streak) = get_longest_streak(&total_streaks, &longest_streak) {
            longest_streak = new_longest_streak;
        }

        if !map.contains_key(&game_id) {
            let new_game = GamePlayedReviewDTO::from(game_with_log);
            map.insert(game_id.clone(), new_game);
        }
        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.
        fill_played_game_review(game, start_datetime, end_datetime, time);
    }

    // Fill first played
    for first_log in first_logs {
        let game_id = first_log.game_id.to_string();

        let log = GameLogDTO::from(first_log);
        let first_start_datetime = log.start_datetime;

        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.

        if let Some(last_session) = game.sessions.last() {
            let start_datetime = last_session.start_datetime;
            game.first_played = first_start_datetime == start_datetime;
        }
    }

    // Fill globals and grouped
    let mut total_played = 0;
    let mut total_first_played = 0;
    let mut total_sessions = 0;
    let mut total_rated = 0;
    let mut total_time = DurationDef::default();
    let mut total_time_by_month = HashMap::<u32, DurationDef>::new();
    let mut total_time_by_week = HashMap::<u32, DurationDef>::new();
    let mut total_time_by_weekday = HashMap::<u32, DurationDef>::new();
    let mut total_time_by_hour = HashMap::<u32, DurationDef>::new();
    let mut total_played_by_release_year = HashMap::<i32, i32>::new();
    let mut total_rated_by_rating = HashMap::<i32, i32>::new();
    let mut longest_session = GamesLogDTO::default();
    let mut first_session = GamesLogDTO {
        game_id: String::default(),
        start_datetime: NaiveDateTime::MAX,
        end_datetime: NaiveDateTime::default(),
        time: DurationDef::default(),
    };
    let mut last_session = GamesLogDTO {
        game_id: String::default(),
        start_datetime: NaiveDateTime::MIN,
        end_datetime: NaiveDateTime::default(),
        time: DurationDef::default(),
    };
    for game in map.values_mut() {
        let game_id = game.id.to_string();

        total_played += 1;
        total_first_played += if game.first_played { 1 } else { 0 };
        total_sessions += game.total_sessions;

        // Fill global total time
        total_time = DurationDef::microseconds(total_time.micros + game.total_time.micros);
        logs_utils::merge_total_time_grouped(&mut total_time_by_month, &game.total_time_by_month);
        logs_utils::merge_total_time_grouped(&mut total_time_by_week, &game.total_time_by_week);
        logs_utils::merge_total_time_grouped(
            &mut total_time_by_weekday,
            &game.total_time_by_weekday,
        );
        logs_utils::merge_total_time_grouped(&mut total_time_by_hour, &game.total_time_by_hour);

        // Fill global total by release year
        logs_utils::fill_total_optional_map(&mut total_played_by_release_year, &game.release_year);

        if game.rating != 0 {
            // Fill global total by rating
            total_rated += 1;
            logs_utils::fill_total_map(&mut total_rated_by_rating, game.rating);
        }

        // Found longer global session
        if let Some(new_longest_session) =
            get_longest_session(&game.longest_session, &longest_session, &game_id)
        {
            longest_session = new_longest_session;
        };

        if let Some(new_first_session) =
            get_first_session(&game.first_session, &first_session, &game_id)
        {
            first_session = new_first_session;
        }

        if let Some(new_last_session) =
            get_last_session(&game.last_session, &last_session, &game_id)
        {
            last_session = new_last_session;
        }
    }

    GamesPlayedReviewDTO {
        total_played,
        total_first_played,
        longest_streak,
        longest_session,
        first_session,
        last_session,
        total_sessions,
        total_time,
        total_time_by_month,
        total_time_by_week,
        total_time_by_weekday,
        total_time_by_hour,
        total_played_by_release_year,
        total_rated,
        total_rated_by_rating,
        games: map.into_values().collect(),
    }
}

fn build_finished_review(
    game_with_finishes: Vec<GameWithDate>,
    first_finishes: Vec<GameFinish>,
) -> GamesFinishedReviewDTO {
    let mut map = HashMap::<String, GameFinishedReviewDTO>::new();

    // Fill finishes map
    for game_with_finish in game_with_finishes {
        let game_id = game_with_finish.id.to_string();

        let finish_date = game_with_finish.query_date;

        if !map.contains_key(&game_id) {
            let new_game = GameFinishedReviewDTO::from(game_with_finish);
            map.insert(game_id.clone(), new_game);
        }
        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.
        fill_finished_game_review(game, finish_date);
    }

    // Fill first played
    for first_finish in first_finishes {
        let game_id = first_finish.game_id.to_string();

        let first_finish_date = first_finish.date;

        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.

        if let Some(finish_date) = game.finishes.last() {
            game.first_finished = first_finish_date == finish_date.clone();
        }
    }

    // Fill globals and grouped
    let mut total_finished = 0;
    let mut total_first_finished = 0;
    let mut total_finished_by_month = HashMap::<u32, i32>::new();
    let mut total_finished_by_release_year = HashMap::<i32, i32>::new();
    for game in map.values_mut() {
        total_finished += 1;
        total_first_finished += if game.first_finished { 1 } else { 0 };

        // Fill global total finished
        logs_utils::merge_total_finished_by_month(
            &mut total_finished_by_month,
            &game.total_finished_grouped,
        );

        // Fill global total by release year
        logs_utils::fill_total_optional_map(
            &mut total_finished_by_release_year,
            &game.release_year,
        );
    }

    GamesFinishedReviewDTO {
        total_finished,
        total_first_finished,
        total_finished_grouped: total_finished_by_month,
        total_finished_by_release_year,
        games: map.into_values().collect(),
    }
}

fn get_longest_streak(
    streaks: &[GamesStreakDTO],
    current_longest_streak: &GamesStreakDTO,
) -> Option<GamesStreakDTO> {
    if let Some(last_streak) = streaks.last() {
        let last_streak_days = last_streak.days;
        if last_streak_days > current_longest_streak.days {
            return Some(GamesStreakDTO {
                games_ids: last_streak.games_ids.clone(),
                start_date: last_streak.start_date,
                end_date: last_streak.end_date,
                days: last_streak_days,
            });
        }
    }
    None
}

fn get_longest_session(
    longest_session: &GameLogDTO,
    current_longest_session: &GamesLogDTO,
    game_id: &str,
) -> Option<GamesLogDTO> {
    let longest_session_time = longest_session.time.clone();
    if longest_session_time.micros > current_longest_session.time.micros {
        return Some(GamesLogDTO {
            game_id: String::from(game_id),
            start_datetime: longest_session.start_datetime,
            end_datetime: longest_session.end_datetime,
            time: longest_session_time,
        });
    }
    None
}

fn get_first_session(
    first_sesion: &GameLogDTO,
    current_first_session: &GamesLogDTO,
    game_id: &str,
) -> Option<GamesLogDTO> {
    let first_session_start_datetime = first_sesion.start_datetime;
    if first_session_start_datetime < current_first_session.start_datetime {
        return Some(GamesLogDTO {
            game_id: String::from(game_id),
            start_datetime: first_session_start_datetime,
            end_datetime: first_sesion.end_datetime,
            time: first_sesion.time.clone(),
        });
    }
    None
}

fn get_last_session(
    last_sesion: &GameLogDTO,
    current_last_session: &GamesLogDTO,
    game_id: &str,
) -> Option<GamesLogDTO> {
    let last_session_start_datetime = last_sesion.start_datetime;
    if last_session_start_datetime > current_last_session.start_datetime {
        return Some(GamesLogDTO {
            game_id: String::from(game_id),
            start_datetime: last_session_start_datetime,
            end_datetime: last_sesion.end_datetime,
            time: last_sesion.time.clone(),
        });
    }
    None
}

fn fill_played_game_review(
    game: &mut GamePlayedReviewDTO,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
    time: DurationDef,
) {
    // Fill total time
    game.total_time = DurationDef::microseconds(game.total_time.micros + time.micros);
    logs_utils::fill_total_time_by_month(
        &mut game.total_time_by_month,
        start_datetime,
        time.clone(),
    );
    logs_utils::fill_total_time_by_week(&mut game.total_time_by_week, start_datetime, time.clone());
    logs_utils::fill_total_time_by_weekday(
        &mut game.total_time_by_weekday,
        start_datetime,
        time.clone(),
    );
    logs_utils::fill_total_time_by_hour(&mut game.total_time_by_hour, start_datetime, end_datetime);

    // Fill streaks
    logs_utils::fill_game_streaks(&mut game.streaks, start_datetime, end_datetime);

    // Found longer streak
    fill_longest_game_streak(game);

    // Fill sessions
    logs_utils::fill_game_sessions(
        &mut game.sessions,
        start_datetime,
        end_datetime,
        time.clone(),
    );
    game.total_sessions =
        i32::try_from(game.sessions.len()).expect("Count was not within valid range");

    // Found longer session
    fill_longest_first_last_game_session(game);
}

fn fill_finished_game_review(game: &mut GameFinishedReviewDTO, finish_date: NaiveDate) {
    // Fill total finished
    logs_utils::fill_total_finished_by_month(&mut game.total_finished_grouped, finish_date);

    // Fill finishes
    logs_utils::fill_game_finishes(&mut game.finishes, finish_date);
    game.total_finished =
        i32::try_from(game.finishes.len()).expect("Count was not within valid range");

    if finish_date < game.first_finish {
        game.first_finish = finish_date;
    }
    if finish_date > game.last_finish {
        game.last_finish = finish_date;
    }
}

fn fill_longest_game_streak(game: &mut GamePlayedReviewDTO) {
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
}

fn fill_longest_first_last_game_session(game: &mut GamePlayedReviewDTO) {
    if let Some(last_session) = game.sessions.last() {
        let last_session_time = last_session.time.clone();
        if last_session_time.micros > game.longest_session.time.micros {
            game.longest_session = GameLogDTO {
                start_datetime: last_session.start_datetime,
                end_datetime: last_session.end_datetime,
                time: last_session_time.clone(),
            };
        }

        let last_session_start_datetime = last_session.start_datetime;
        if last_session_start_datetime <= game.first_session.start_datetime {
            game.first_session = GameLogDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_session.end_datetime,
                time: last_session_time.clone(),
            }
        }

        if last_session_start_datetime >= game.last_session.start_datetime {
            game.last_session = GameLogDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_session.end_datetime,
                time: last_session_time.clone(),
            }
        }
    }
}
