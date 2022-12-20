use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::entities::GameWithLog;
use crate::errors::RepositoryError;
use crate::query::game_log_query;

use super::base::fetch_all;

pub async fn find_all_by_datetime_between(
    pool: &PgPool,
    user_id: i32,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> Result<Vec<GameWithLog>, RepositoryError> {
    let query = game_log_query::select_all_games_log_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(user_id, start_datetime, end_datetime);
    fetch_all(pool, query).await
}
