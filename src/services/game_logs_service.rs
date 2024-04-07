use chrono::{Duration, NaiveDateTime};
use sqlx::postgres::types::PgInterval;
use sqlx::PgPool;

use crate::entities::{GameLog, GameLogWithTime};
use crate::errors::ApiErrors;
use crate::models::{DurationDef, GameLogDTO, Merge, NewGameLogDTO};
use crate::repository::game_log_repository;

use super::base::{
    handle_action_result, handle_already_exists_result, handle_get_list_result,
    handle_not_found_result, handle_result,
};
use super::games_service;

pub async fn get_sum_game_logs(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<DurationDef, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_log_repository::find_sum_time_by_game_id(pool, user_id, game_id).await;
    let duration = handle_result::<PgInterval, GameLogDTO>(find_result)?;
    Ok(DurationDef::from(duration))
}

pub async fn get_game_logs(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<GameLogDTO>, ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let find_result = game_log_repository::find_all_by_game_id(pool, user_id, game_id).await;
    handle_get_list_result::<GameLogWithTime, GameLogDTO>(find_result)
}

pub(super) async fn find_first_game_logs_by_games(
    pool: &PgPool,
    user_id: &str,
    game_ids: Vec<String>,
) -> Result<Vec<GameLogWithTime>, ApiErrors> {
    let find_result =
        game_log_repository::find_all_first_by_user_id_and_game_id_in(pool, user_id, game_ids)
            .await;
    handle_result::<Vec<GameLogWithTime>, GameLogDTO>(find_result)
}

pub async fn create_game_log(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    log: NewGameLogDTO,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;

    let start_datetime = log.start_datetime;
    let end_datetime = log.end_datetime;

    if start_datetime > end_datetime {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Session start datetime must be previous than end datetime",
        )));
    }

    let logs: Vec<NewGameLogDTO> = split_session_into_logs(start_datetime, end_datetime);
    if logs.is_empty() {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Session to add must not have an empty span of time",
        )));
    }

    let exists_result =
        game_log_repository::exists_gap(pool, user_id, start_datetime, end_datetime).await;
    handle_already_exists_result::<GameLogDTO>(exists_result)?;

    let logs_to_create: Vec<GameLog> = logs
        .into_iter()
        .map(GameLogDTO::merge_with_default)
        .map(GameLog::from)
        .collect();
    let create_result =
        game_log_repository::create_multiple(pool, user_id, game_id, logs_to_create).await;
    handle_action_result::<GameLogDTO>(create_result)
}

pub async fn delete_game_log(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    datetime: NaiveDateTime,
) -> Result<(), ApiErrors> {
    games_service::exists_game(pool, user_id, game_id).await?;
    exists_game_log(pool, user_id, game_id, datetime).await?;

    let delete_result = game_log_repository::delete_by_id(pool, user_id, game_id, datetime).await;
    handle_action_result::<GameLogDTO>(delete_result)
}

pub async fn exists_game_log(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
    datetime: NaiveDateTime,
) -> Result<(), ApiErrors> {
    let exists_result = game_log_repository::exists_by_id(pool, user_id, game_id, datetime).await;
    handle_not_found_result::<GameLogDTO>(exists_result)
}

fn split_session_into_logs(
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> Vec<NewGameLogDTO> {
    let mut sessions: Vec<NewGameLogDTO> = vec![];
    if start_datetime.date() == end_datetime.date() {
        // If session happens on the same day
        if start_datetime.time() != end_datetime.time() {
            // Avoid empty log -> return single session if span of time is valid
            sessions.push(NewGameLogDTO {
                start_datetime,
                end_datetime,
            });
        }
    } else {
        // If session spans differents day
        let mut temp_date = start_datetime;
        while temp_date.date() < end_datetime.date() {
            let next_day_at_start_of_day =
                crate::date_utils::date_at_start_of_day(temp_date.date() + Duration::days(1));
            sessions.push(NewGameLogDTO {
                start_datetime: temp_date,
                end_datetime: next_day_at_start_of_day,
            });
            temp_date = next_day_at_start_of_day;
        }
        if end_datetime.time() != temp_date.time() {
            // Avoid empty log -> store last log if span of time until end date is valid
            sessions.push(NewGameLogDTO {
                start_datetime: temp_date,
                end_datetime,
            });
        }
    }
    sessions
}
