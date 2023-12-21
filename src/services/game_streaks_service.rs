use std::collections::HashMap;

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::GameLogWithTime;
use crate::errors::ApiErrors;
use crate::models::{DurationDef, GameLogDTO, GameStreakDTO, GamesStreakDTO};
use crate::repository::game_log_repository;

use super::base::{check_start_end, handle_result, start_end_to_datetime};
use super::{games_service, logs_utils};

pub async fn get_game_streaks(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameStreakDTO>, ApiErrors> {
    check_start_end(start_date, end_date)?;
    games_service::exists_game(pool, user_id, game_id).await?;

    let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
    let find_result = game_log_repository::find_all_by_game_id_between(
        pool,
        user_id,
        game_id,
        start_datetime,
        end_datetime,
    )
    .await;
    let logs = handle_result::<Vec<GameLogWithTime>, GameLogDTO>(find_result)?;
    let streaks = build_game_streaks(logs);

    Ok(streaks)
}

pub async fn get_streaks(
    pool: &PgPool,
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GamesStreakDTO>, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
    let find_result =
        game_log_repository::find_all_between(pool, user_id, start_datetime, end_datetime).await;
    let logs = handle_result::<Vec<GameLogWithTime>, GameLogDTO>(find_result)?;
    let streaks = build_games_streaks(logs);

    Ok(streaks)
}

pub async fn get_sum_game_logs_grouped_by_month(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<HashMap<u32, DurationDef>, ApiErrors> {
    check_start_end(start_date, end_date)?;
    games_service::exists_game(pool, user_id, game_id).await?;

    let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
    let find_result = game_log_repository::find_all_by_game_id_between(
        pool,
        user_id,
        game_id,
        start_datetime,
        end_datetime,
    )
    .await;
    let logs = handle_result::<Vec<GameLogWithTime>, GameLogDTO>(find_result)?;

    let sum_by_month_map = build_sum_game_by_month(logs);

    Ok(sum_by_month_map)
}

fn build_game_streaks(logs: Vec<GameLogWithTime>) -> Vec<GameStreakDTO> {
    let mut streaks: Vec<GameStreakDTO> = vec![];
    for log in logs {
        let start_datetime = log.datetime;
        let end_datetime = log.end_datetime;

        logs_utils::fill_game_streaks(&mut streaks, start_datetime, end_datetime);
    }

    streaks
}

fn build_games_streaks(logs: Vec<GameLogWithTime>) -> Vec<GamesStreakDTO> {
    let mut streaks: Vec<GamesStreakDTO> = vec![];
    for log in logs {
        let game_id = log.game_id.to_string();
        let start_datetime = log.datetime;
        let end_datetime = log.end_datetime;

        logs_utils::fill_streaks(&mut streaks, &game_id, start_datetime, end_datetime);
    }

    streaks
}

fn build_sum_game_by_month(logs: Vec<GameLogWithTime>) -> HashMap<u32, DurationDef> {
    let mut sum_by_month_map = HashMap::<u32, DurationDef>::new();
    for log in logs {
        let start_datetime = log.datetime;
        let time = DurationDef::from(log.query_time);

        logs_utils::fill_sum_game_by_month(&mut sum_by_month_map, start_datetime, time);
    }

    sum_by_month_map
}

fn build_sum_games_by_month(
    logs: Vec<GameLogWithTime>,
) -> HashMap<String, HashMap<u32, DurationDef>> {
    let mut sums_by_month_map = HashMap::<String, HashMap<u32, DurationDef>>::new();
    for log in logs {
        let game_id = log.game_id.to_string();
        let start_datetime = log.datetime;
        let time = DurationDef::from(log.query_time);

        match sums_by_month_map.get_mut(&game_id) {
            Some(sum_by_month_map) => {
                logs_utils::fill_sum_game_by_month(sum_by_month_map, start_datetime, time);
            }
            None => {
                // Start game month sum
                let mut sum_by_month_map = HashMap::<u32, DurationDef>::new();
                logs_utils::fill_sum_game_by_month(&mut sum_by_month_map, start_datetime, time);
                sums_by_month_map.insert(game_id, sum_by_month_map);
            }
        }
    }

    sums_by_month_map
}
