use std::collections::HashMap;

use chrono::NaiveDate;
use sqlx::PgPool;

use crate::entities::GameWithLog;
use crate::errors::ApiErrors;
use crate::models::{GameDTO, GameLogDTO, GameWithLogsDTO};
use crate::repository::game_with_log_repository;

use super::base::handle_result;

pub async fn get_game_with_logs(
    pool: &PgPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<GameWithLogsDTO>, ApiErrors> {
    let start_datetime = crate::date_utils::date_at_start_of_day(start_date);
    let end_datetime = crate::date_utils::date_at_midnight(end_date);
    let find_result = game_with_log_repository::find_with_log_by_datetime(
        pool,
        user_id,
        start_datetime,
        end_datetime,
    )
    .await;
    let entity_list = handle_result::<Vec<GameWithLog>, GameDTO>(find_result)?;

    let game_with_logs = create_unique_list(entity_list);
    Ok(game_with_logs)
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
