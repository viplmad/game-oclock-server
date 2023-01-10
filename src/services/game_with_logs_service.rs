use std::collections::HashMap;

use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

use crate::entities::{GameSearch, GameWithLog};
use crate::errors::ApiErrors;
use crate::models::{
    GameLogDTO, GameWithLogDTO, GameWithLogPageResult, GameWithLogsDTO, SearchDTO,
};
use crate::repository::game_with_log_repository;

use super::base::{
    check_start_end, handle_get_list_paged_result, handle_query_mapping, handle_result,
};

pub async fn search_first_played_games(
    pool: &PgPool,
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithLogPageResult, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
    let search = handle_query_mapping::<GameWithLogDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_log_repository::search_first_by_datetime_between(
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
    user_id: i32,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    search: SearchDTO,
    quicksearch: Option<String>,
) -> Result<GameWithLogPageResult, ApiErrors> {
    check_start_end(start_date, end_date)?;

    let (start_datetime, end_datetime) = start_end_to_datetime(start_date, end_date);
    let search = handle_query_mapping::<GameWithLogDTO, GameSearch>(search, quicksearch)?;
    let find_result = game_with_log_repository::search_last_by_datetime_between(
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
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameWithLogsDTO>, ApiErrors> {
    if start_date > end_date {
        return Err(ApiErrors::InvalidParameter(String::from(
            "Start date must be previous than end date",
        )));
    }

    let start_datetime = crate::date_utils::date_at_start_of_day(start_date);
    let end_datetime = crate::date_utils::date_at_midnight(end_date);
    let find_result = game_with_log_repository::find_all_by_datetime_between(
        pool,
        user_id,
        start_datetime,
        end_datetime,
    )
    .await;
    let entity_list = handle_result::<Vec<GameWithLog>, GameWithLogDTO>(find_result)?;

    let game_with_logs = create_unique_list(entity_list);
    Ok(game_with_logs)
}

fn start_end_to_datetime(
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> (Option<NaiveDateTime>, Option<NaiveDateTime>) {
    let start_datetime = start_date.map(crate::date_utils::date_at_start_of_day);
    let end_datetime = end_date.map(crate::date_utils::date_at_midnight);
    (start_datetime, end_datetime)
}

fn create_unique_list(game_with_logs: Vec<GameWithLog>) -> Vec<GameWithLogsDTO> {
    let mut map = HashMap::<i32, GameWithLogsDTO>::new();

    for game_with_log in game_with_logs {
        let game_id = game_with_log.id;

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
