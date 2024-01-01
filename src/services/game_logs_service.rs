use chrono::{Duration, NaiveDateTime, NaiveTime};
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

    if log.start_datetime > log.end_datetime {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Start date time must be previous than end date time",
        )));
    }

    // TODO Split if contains different days
    if log.start_datetime.date() != log.end_datetime.date() {
        // If the end date is next day
        if log.start_datetime.date() == (log.end_datetime.date() - Duration::days(1)) {
            // If end time is not midnight
            if log.end_datetime.time() != NaiveTime::MIN {
                return Err(ApiErrors::InvalidParameter(String::from(
                    "Log must be from the same day or until midnight of next day",
                )));
            }
        } else {
            return Err(ApiErrors::InvalidParameter(String::from(
                "Log must be from the same day or until midnight of next day",
            )));
        }
    }

    let exists_result =
        game_log_repository::exists_gap(pool, user_id, log.start_datetime, log.end_datetime).await;
    handle_already_exists_result::<GameLogDTO>(exists_result)?;

    let merged_log = GameLogDTO::merge_with_default(log);
    let log_to_create = GameLog::from(merged_log);
    let create_result = game_log_repository::create(pool, user_id, game_id, &log_to_create).await;
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
