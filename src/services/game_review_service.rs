use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use uuid::Uuid;

use crate::entities::{GameFinish, GameLogWithTime, GameWithFinish, GameWithLog};
use crate::errors::ApiErrors;
use crate::models::{
    DurationDef, FinishDTO, GameFinishDTO, GameFinishedReviewDTO, GameLogDTO, GamePlayedReviewDTO,
    GameStatus, GamesFinishedReviewDTO, GamesPlayedReviewDTO, GamesStreakDTO, LogDTO, StreakDTO,
};

use super::{
    GameFinishService, GameLogService, GameWithFinishService, GameWithLogService, logs_utils,
};

#[derive(Clone)]
pub struct GameReviewService {
    game_log_service: GameLogService,
    game_finish_service: GameFinishService,
    game_with_log_service: GameWithLogService,
    game_with_finish_service: GameWithFinishService,
}

impl GameReviewService {
    pub fn with(
        game_log_service: GameLogService,
        game_finish_service: GameFinishService,
        game_with_log_service: GameWithLogService,
        game_with_finish_service: GameWithFinishService,
    ) -> Self {
        Self {
            game_log_service,
            game_finish_service,
            game_with_log_service,
            game_with_finish_service,
        }
    }
}

impl GameReviewService {
    pub async fn get_played_games_review(
        &self,
        user_id: &Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<GamesPlayedReviewDTO, ApiErrors> {
        let game_with_logs = self
            .game_with_log_service
            .find_game_with_logs_between(user_id, start_date, end_date)
            .await?;

        let game_ids = game_with_logs
            .iter()
            .map(|game| game.id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect();
        let first_logs = self
            .game_log_service
            .find_first_game_logs_by_games(user_id, game_ids)
            .await?;

        let review = build_played_review(game_with_logs, first_logs);
        Ok(review)
    }

    pub async fn get_finished_games_review(
        &self,
        user_id: &Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<GamesFinishedReviewDTO, ApiErrors> {
        let game_with_finishes = self
            .game_with_finish_service
            .find_game_with_finishes_between(user_id, start_date, end_date)
            .await?;

        let game_ids = game_with_finishes
            .iter()
            .map(|game| game.id)
            .collect::<HashSet<Uuid>>()
            .into_iter()
            .collect();
        let first_finishes = self
            .game_finish_service
            .find_first_game_finishes_by_games(user_id, game_ids)
            .await?;

        let review = build_finished_review(game_with_finishes, first_finishes);
        Ok(review)
    }
}

fn build_played_review(
    game_with_logs: Vec<GameWithLog>,
    first_logs: Vec<GameLogWithTime>,
) -> GamesPlayedReviewDTO {
    let mut map = HashMap::<Uuid, GamePlayedReviewDTO>::new();

    let mut total_streaks: Vec<GamesStreakDTO> = vec![];
    let mut longest_streak = GamesStreakDTO {
        games_ids: vec![],
        streak: StreakDTO {
            start_date: NaiveDate::default(),
            end_date: NaiveDate::default(),
            days: 0,
        },
    };

    // Fill logs map and global streaks
    for game_with_log in game_with_logs {
        let game_id = game_with_log.id;

        let log = LogDTO::from(&game_with_log);
        let start_datetime = log.start_datetime;
        let end_datetime = log.end_datetime;
        let device_id = log.device_id;
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
        fill_played_game_review(game, start_datetime, end_datetime, device_id, time);
    }

    // Fill first played
    for first_log in first_logs {
        let game_id = first_log.game_id;

        let log = LogDTO::from(first_log);
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
    let mut total_played_by_release_year = HashMap::<u32, u32>::new();
    let mut total_rated_by_rating = HashMap::<u32, u32>::new();
    let mut longest_session = GameLogDTO::default();
    let mut first_session = GameLogDTO {
        user_id: Uuid::default(),
        game_id: Uuid::default(),
        log: LogDTO {
            start_datetime: DateTime::<Utc>::MAX_UTC,
            end_datetime: DateTime::default(),
            time: DurationDef::default(),
            device_id: None,
        },
    };
    let mut last_session = GameLogDTO {
        user_id: Uuid::default(),
        game_id: Uuid::default(),
        log: LogDTO {
            start_datetime: DateTime::<Utc>::MIN_UTC,
            end_datetime: DateTime::default(),
            time: DurationDef::default(),
            device_id: None,
        },
    };
    for game_played in map.values_mut() {
        let game = &game_played.game;
        let user_id = game.user_id;
        let game_id = game.id;

        total_played += 1;
        total_first_played += if game_played.first_played { 1 } else { 0 };
        total_sessions += game_played.total_sessions;

        // Fill global total time
        total_time = DurationDef::microseconds(total_time.micros + game_played.total_time.micros);
        logs_utils::merge_total_time_grouped(
            &mut total_time_by_month,
            &game_played.total_time_by_month,
        );
        logs_utils::merge_total_time_grouped(
            &mut total_time_by_week,
            &game_played.total_time_by_week,
        );
        logs_utils::merge_total_time_grouped(
            &mut total_time_by_weekday,
            &game_played.total_time_by_weekday,
        );
        logs_utils::merge_total_time_grouped(
            &mut total_time_by_hour,
            &game_played.total_time_by_hour,
        );

        // Fill global total by release year
        logs_utils::fill_total_optional_map(
            &mut total_played_by_release_year,
            &game
                .release_date
                .map(|d| u32::try_from(d.year()).expect("Year is not AC")),
        );

        if game.rating != 0 {
            // Fill global total by rating
            total_rated += 1;
            logs_utils::fill_total_map(&mut total_rated_by_rating, game.rating);
        }

        // Found longer global session
        if let Some(new_longest_session) = get_longest_session(
            &game_played.longest_session,
            &longest_session,
            &user_id,
            &game_id,
        ) {
            longest_session = new_longest_session;
        };

        if let Some(new_first_session) = get_first_session(
            &game_played.first_session,
            &first_session,
            &user_id,
            &game_id,
        ) {
            first_session = new_first_session;
        }

        if let Some(new_last_session) =
            get_last_session(&game_played.last_session, &last_session, &user_id, &game_id)
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
    game_with_finishes: Vec<GameWithFinish>,
    first_finishes: Vec<GameFinish>,
) -> GamesFinishedReviewDTO {
    let mut map = HashMap::<Uuid, GameFinishedReviewDTO>::new();

    // Fill finishes map
    for game_with_finish in game_with_finishes {
        let game_id = game_with_finish.id;

        let finish = FinishDTO::from(&game_with_finish);
        let date = finish.date;
        let status = finish.status;
        let device_id = finish.device_id;

        if !map.contains_key(&game_id) {
            let new_game = GameFinishedReviewDTO::from(game_with_finish);
            map.insert(game_id.clone(), new_game);
        }
        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.
        fill_finished_game_review(game, date, status, device_id);
    }

    // Fill first played
    for first_finish in first_finishes {
        let game_id = first_finish.game_id;

        let finish = FinishDTO::from(first_finish);
        let first_finish_date = finish.date;

        let game = map.get_mut(&game_id).unwrap(); // Safe unwrap: already checked the key is contained.

        if let Some(finish_date) = game.finishes.last() {
            game.first_finished = first_finish_date == finish_date.date.clone();
        }
    }

    // Fill globals and grouped
    let mut total_finished = 0;
    let mut total_first_finished = 0;
    let mut total_finished_by_month = HashMap::<u32, u32>::new();
    let mut total_finished_by_release_year = HashMap::<u32, u32>::new();
    let mut first_finish = GameFinishDTO {
        user_id: Uuid::default(),
        game_id: Uuid::default(),
        finish: FinishDTO {
            date: NaiveDate::MAX,
            status: GameStatus::LowPriority,
            device_id: None,
        },
    };
    let mut last_finish = GameFinishDTO {
        user_id: Uuid::default(),
        game_id: Uuid::default(),
        finish: FinishDTO {
            date: NaiveDate::MIN,
            status: GameStatus::LowPriority,
            device_id: None,
        },
    };
    for game_finished in map.values_mut() {
        let game = &game_finished.game;
        let user_id = game.user_id;
        let game_id = game.id;

        total_finished += 1;
        total_first_finished += if game_finished.first_finished { 1 } else { 0 };

        // Fill global total finished
        logs_utils::merge_total_finished_by_month(
            &mut total_finished_by_month,
            &game_finished.total_finished_grouped,
        );

        // Fill global total by release year
        logs_utils::fill_total_optional_map(
            &mut total_finished_by_release_year,
            &game
                .release_date
                .map(|d| u32::try_from(d.year()).expect("Year is not AC")),
        );

        if let Some(new_first_session) = get_first_finish(
            &game_finished.first_finish,
            &first_finish,
            &user_id,
            &game_id,
        ) {
            first_finish = new_first_session;
        }

        if let Some(new_last_session) =
            get_last_finish(&game_finished.last_finish, &last_finish, &user_id, &game_id)
        {
            last_finish = new_last_session;
        }
    }

    GamesFinishedReviewDTO {
        total_finished,
        total_first_finished,
        first_finish,
        last_finish,
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
        let last_streak_days = last_streak.streak.days;
        if last_streak_days > current_longest_streak.streak.days {
            return Some(GamesStreakDTO {
                games_ids: last_streak.games_ids.clone(),
                streak: StreakDTO {
                    start_date: last_streak.streak.start_date,
                    end_date: last_streak.streak.end_date,
                    days: last_streak_days,
                },
            });
        }
    }
    None
}

fn get_longest_session(
    longest_session: &LogDTO,
    current_longest_session: &GameLogDTO,
    user_id: &Uuid,
    game_id: &Uuid,
) -> Option<GameLogDTO> {
    let longest_session_time = longest_session.time.clone();
    if longest_session_time.micros > current_longest_session.log.time.micros {
        return Some(GameLogDTO {
            user_id: user_id.clone(),
            game_id: game_id.clone(),
            log: LogDTO {
                start_datetime: longest_session.start_datetime,
                end_datetime: longest_session.end_datetime,
                device_id: longest_session.device_id,
                time: longest_session_time,
            },
        });
    }
    None
}

fn get_first_session(
    first_sesion: &LogDTO,
    current_first_session: &GameLogDTO,
    user_id: &Uuid,
    game_id: &Uuid,
) -> Option<GameLogDTO> {
    let first_session_start_datetime = first_sesion.start_datetime;
    if first_session_start_datetime < current_first_session.log.start_datetime {
        return Some(GameLogDTO {
            user_id: user_id.clone(),
            game_id: game_id.clone(),
            log: LogDTO {
                start_datetime: first_session_start_datetime,
                end_datetime: first_sesion.end_datetime,
                device_id: first_sesion.device_id,
                time: first_sesion.time.clone(),
            },
        });
    }
    None
}

fn get_last_session(
    last_sesion: &LogDTO,
    current_last_session: &GameLogDTO,
    user_id: &Uuid,
    game_id: &Uuid,
) -> Option<GameLogDTO> {
    let last_session_start_datetime = last_sesion.start_datetime;
    if last_session_start_datetime > current_last_session.log.start_datetime {
        return Some(GameLogDTO {
            user_id: user_id.clone(),
            game_id: game_id.clone(),
            log: LogDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_sesion.end_datetime,
                device_id: last_sesion.device_id,
                time: last_sesion.time.clone(),
            },
        });
    }
    None
}

fn get_first_finish(
    first_finish: &FinishDTO,
    current_first_finish: &GameFinishDTO,
    user_id: &Uuid,
    game_id: &Uuid,
) -> Option<GameFinishDTO> {
    let first_finish_date = first_finish.date;
    if first_finish_date < current_first_finish.finish.date {
        return Some(GameFinishDTO {
            user_id: user_id.clone(),
            game_id: game_id.clone(),
            finish: FinishDTO {
                date: first_finish_date,
                status: first_finish.status.clone(),
                device_id: first_finish.device_id,
            },
        });
    }
    None
}

fn get_last_finish(
    last_finish: &FinishDTO,
    current_last_finish: &GameFinishDTO,
    user_id: &Uuid,
    game_id: &Uuid,
) -> Option<GameFinishDTO> {
    let last_finish_date = last_finish.date;
    if last_finish_date > current_last_finish.finish.date {
        return Some(GameFinishDTO {
            user_id: user_id.clone(),
            game_id: game_id.clone(),
            finish: FinishDTO {
                date: last_finish_date,
                status: last_finish.status.clone(),
                device_id: last_finish.device_id,
            },
        });
    }
    None
}

fn fill_played_game_review(
    game: &mut GamePlayedReviewDTO,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>, // TODO Pass object
    device_id: Option<Uuid>,
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
        device_id,
        time.clone(),
    );
    game.total_sessions =
        u32::try_from(game.sessions.len()).expect("Count is not within valid range");

    // Found longer session
    fill_longest_first_last_game_session(game);
}

fn fill_finished_game_review(
    game: &mut GameFinishedReviewDTO,
    date: NaiveDate, // TODO Pass object
    status: GameStatus,
    device_id: Option<Uuid>,
) {
    // Fill total finished
    logs_utils::fill_total_finished_by_month(&mut game.total_finished_grouped, date);

    // Fill finishes
    logs_utils::fill_game_finishes(&mut game.finishes, date, status.clone(), device_id);
    game.total_finished =
        u32::try_from(game.finishes.len()).expect("Count is not within valid range");

    if date < game.first_finish.date {
        game.first_finish = FinishDTO {
            date,
            status: status.clone(),
            device_id: device_id.clone(),
        };
    }
    if date > game.last_finish.date {
        game.last_finish = FinishDTO {
            date,
            status: status.clone(),
            device_id: device_id.clone(),
        };
    }
}

fn fill_longest_game_streak(game: &mut GamePlayedReviewDTO) {
    if let Some(last_streak) = game.streaks.last() {
        let last_streak_days = last_streak.days;
        if last_streak_days > game.longest_streak.days {
            game.longest_streak = StreakDTO {
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
            game.longest_session = LogDTO {
                start_datetime: last_session.start_datetime,
                end_datetime: last_session.end_datetime,
                device_id: last_session.device_id.clone(),
                time: last_session_time.clone(),
            };
        }

        let last_session_start_datetime = last_session.start_datetime;
        if last_session_start_datetime <= game.first_session.start_datetime {
            game.first_session = LogDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_session.end_datetime,
                device_id: last_session.device_id.clone(),
                time: last_session_time.clone(),
            }
        }

        if last_session_start_datetime >= game.last_session.start_datetime {
            game.last_session = LogDTO {
                start_datetime: last_session_start_datetime,
                end_datetime: last_session.end_datetime,
                device_id: last_session.device_id.clone(),
                time: last_session_time.clone(),
            }
        }
    }
}
