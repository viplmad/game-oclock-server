use std::collections::HashMap;

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::{GameSearch, GameWithLog};
use crate::errors::ApiErrors;
use crate::models::{
    GameLogDTO, GameWithLogDTO, GameWithLogPageResult, GameWithLogsDTO, SearchDTO,
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

    let game_with_logs = build_game_with_logs_list(entity_list);
    Ok(game_with_logs)
}

pub(super) async fn find_game_with_logs_between(
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

fn build_game_with_logs_list(game_with_logs: Vec<GameWithLog>) -> Vec<GameWithLogsDTO> {
    let mut map = HashMap::<String, GameWithLogsDTO>::new();

    for game_with_log in game_with_logs {
        let game_id = game_with_log.id.to_string();

        let log = GameLogDTO::from(&game_with_log);
        match map.get_mut(&game_id) {
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
