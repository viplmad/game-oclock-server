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

        let existing_game = map.get_mut(&game_id);
        let log = GameLogDTO::from(&game_with_log);
        match existing_game {
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
    let mut longest_session = GamesLogDTO {
        game_id: String::default(),
        start_datetime: NaiveDateTime::default(),
        end_datetime: NaiveDateTime::default(),
        time: DurationDef::default(),
    };
    let mut streaks: Vec<GamesStreakDTO> = vec![];
    let mut longest_streak = GamesStreakDTO {
        games_ids: vec![],
        start_date: NaiveDate::default(),
        end_date: NaiveDate::default(),
        days: 0,
    };

    for game_with_log in game_with_logs {
        let game_id = game_with_log.id.to_string();

        let existing_game = map.get_mut(&game_id);
        let log = GameLogDTO::from(&game_with_log);
        match existing_game {
            Some(game) => {
                let mut session_time = log.time.clone();

                // Check if this is part of a continuous log (ended on midnight and kept playing)
                if let Some(last_log) = game.logs.last() {
                    if
                    // If the date of the current log is on the previous day of the last log
                    log.start_datetime.date() == (last_log.start_datetime.date() - Duration::days(1))
                    // and the end time of the current log is midnight
                    && log.end_datetime.time() == NaiveTime::MIN
                    // and the start time of the last log is midnight
                    && last_log.start_datetime.time() == NaiveTime::MIN
                    {
                        session_time =
                            DurationDef::microseconds(log.time.micros + last_log.time.micros);
                    }
                }

                // Found longer session
                if session_time.micros > game.longest_session.time.micros {
                    game.longest_session = GameLogDTO {
                        start_datetime: log.start_datetime,
                        end_datetime: log.start_datetime
                            + Duration::microseconds(session_time.clone().micros),
                        time: session_time.clone(),
                    };
                }

                let mut streak_days = 1;
                if let Some(last_streak) = game.streaks.last_mut() {
                    let previous_date = last_streak.start_date - Duration::days(1);
                    if log.start_datetime.date() == previous_date {
                        // Continued the streak
                        last_streak.start_date = log.start_datetime.date();
                        last_streak.days += 1;

                        streak_days = last_streak.days;
                    } else if log.start_datetime.date() < previous_date {
                        // Lost the streak
                        game.streaks.push(GameStreakDTO {
                            start_date: log.start_datetime.date(),
                            end_date: log.end_datetime.date(),
                            days: 1,
                        });
                    }
                }

                // Found longer streak
                if streak_days > game.longest_streak.days {
                    game.longest_streak = GameStreakDTO {
                        start_date: log.start_datetime.date(),
                        end_date: log.start_datetime.date() + Duration::days(streak_days - 1),
                        days: streak_days,
                    }
                }

                game.total_time =
                    DurationDef::microseconds(log.time.micros + game.total_time.micros);

                // Found longer global session
                if session_time.micros > longest_session.time.micros {
                    longest_session = GamesLogDTO {
                        game_id: game_id.clone(),
                        start_datetime: log.start_datetime,
                        end_datetime: log.start_datetime
                            + Duration::microseconds(session_time.clone().micros),
                        time: session_time,
                    };
                }
                streak_days = 1;
                let mut streak_games_ids = vec![game_id.clone()];
                if let Some(last_streak) = streaks.last_mut() {
                    let previous_date = last_streak.start_date - Duration::days(1);
                    if log.start_datetime.date() == previous_date {
                        // Continued the streak
                        if !last_streak.games_ids.contains(&game_id) {
                            last_streak.games_ids.push(game_id.clone());
                        }
                        last_streak.start_date = log.start_datetime.date();
                        last_streak.days += 1;

                        streak_games_ids = last_streak.games_ids.clone();
                        streak_days = last_streak.days;
                    } else if log.start_datetime.date() < previous_date {
                        // Lost the streak, start a new one
                        streaks.push(GamesStreakDTO {
                            games_ids: vec![game_id.clone()],
                            start_date: log.start_datetime.date(),
                            end_date: log.end_datetime.date(),
                            days: 1,
                        });
                    } else {
                        // Already on a streak day, add game if necessary
                        if !last_streak.games_ids.contains(&game_id) {
                            last_streak.games_ids.push(game_id.clone());
                        }
                    }
                } else {
                    // No streaks, add first
                    streaks.push(GamesStreakDTO {
                        games_ids: vec![game_id.clone()],
                        start_date: log.start_datetime.date(),
                        end_date: log.end_datetime.date(),
                        days: 1,
                    });
                }

                // Found longer global streak
                if streak_days > longest_streak.days {
                    longest_streak = GamesStreakDTO {
                        games_ids: streak_games_ids,
                        start_date: log.start_datetime.date(),
                        end_date: log.start_datetime.date() + Duration::days(streak_days - 1),
                        days: streak_days,
                    }
                }
                total_time = DurationDef::microseconds(log.time.micros + total_time.micros);

                game.logs.push(log);
            }
            None => {
                let mut game = GameWithLogsExtendedDTO::from(game_with_log);
                game.streaks.push(GameStreakDTO {
                    start_date: log.start_datetime.date(),
                    end_date: log.end_datetime.date(),
                    days: 1,
                });
                // TODO Implement Clone
                game.longest_streak = GameStreakDTO {
                    start_date: log.start_datetime.date(),
                    end_date: log.end_datetime.date(),
                    days: 1,
                };
                // TODO Implement Clone
                game.longest_session = GameLogDTO {
                    start_datetime: log.start_datetime,
                    end_datetime: log.end_datetime,
                    time: log.time.clone(),
                };
                game.total_time = log.time.clone();
                // TODO Implement add

                let session_time = log.time.clone();

                // Found longer global session
                if session_time.micros > longest_session.time.micros {
                    longest_session = GamesLogDTO {
                        game_id: game_id.clone(),
                        start_datetime: log.start_datetime,
                        end_datetime: log.end_datetime,
                        time: session_time,
                    };
                }

                let mut streak_days = 1;
                let mut streak_games_ids = vec![game_id.clone()];
                if let Some(last_streak) = streaks.last_mut() {
                    let previous_date = last_streak.start_date - Duration::days(1);
                    if log.start_datetime.date() == previous_date {
                        // Continued the streak
                        if !last_streak.games_ids.contains(&game_id) {
                            last_streak.games_ids.push(game_id.clone());
                        }
                        last_streak.start_date = log.start_datetime.date();
                        last_streak.days += 1;

                        streak_games_ids = last_streak.games_ids.clone();
                        streak_days = last_streak.days;
                    } else if log.start_datetime.date() < previous_date {
                        // Lost the streak, start a new one
                        streaks.push(GamesStreakDTO {
                            games_ids: vec![game_id.clone()],
                            start_date: log.start_datetime.date(),
                            end_date: log.end_datetime.date(),
                            days: 1,
                        });
                    } else {
                        // Already on a streak day, add game if necessary
                        if !last_streak.games_ids.contains(&game_id) {
                            last_streak.games_ids.push(game_id.clone());
                        }
                    }
                } else {
                    // No streaks, add first
                    streaks.push(GamesStreakDTO {
                        games_ids: vec![game_id.clone()],
                        start_date: log.start_datetime.date(),
                        end_date: log.end_datetime.date(),
                        days: 1,
                    });
                }

                // Found longer global streak
                if streak_days > longest_streak.days {
                    longest_streak = GamesStreakDTO {
                        games_ids: streak_games_ids,
                        start_date: log.start_datetime.date(),
                        end_date: log.start_datetime.date() + Duration::days(streak_days - 1),
                        days: streak_days,
                    }
                }
                total_time = DurationDef::microseconds(log.time.micros + total_time.micros);
                game.logs.push(log);

                map.insert(game_id, game);
            }
        }
    }

    let num_games = i32::try_from(map.len()).expect("Count was not within valid range");

    GamesWithLogsExtendedDTO {
        count: num_games,
        streaks,
        longest_streak,
        longest_session,
        total_time,
        games_with_logs: map.into_values().collect(),
    }
}
