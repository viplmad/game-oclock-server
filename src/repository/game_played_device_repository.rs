use sqlx::PgPool;

use crate::entities::{Device, Game};
use crate::errors::RepositoryError;
use crate::query::game_played_device_query;

use super::base::fetch_all;

pub async fn find_all_games_with_played_device(
    pool: &PgPool,
    user_id: &str,
    device_id: &str,
) -> Result<Vec<Game>, RepositoryError> {
    let query =
        game_played_device_query::select_all_games_by_device_id_order_by_date(user_id, device_id);
    fetch_all(pool, query).await
}

pub async fn find_all_devices_with_played_game(
    pool: &PgPool,
    user_id: &str,
    game_id: &str,
) -> Result<Vec<Device>, RepositoryError> {
    let query =
        game_played_device_query::select_all_devices_by_game_id_order_by_date(user_id, game_id);
    fetch_all(pool, query).await
}
