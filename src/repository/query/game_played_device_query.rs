use sea_query::{Expr, Order, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{DeviceIden, GameIden, GameLogIden};

use super::{device_query, game_query};

pub fn select_all_games_by_device_id_order_by_date(
    user_id: &Uuid,
    device_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_log_by_device_id(&mut select, device_id);
    add_order_by_start_datetime(&mut select);
    // TODO Distinct?
    select
}

pub fn select_all_devices_by_game_id_order_by_date(
    user_id: &Uuid,
    game_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = device_query::select_all(user_id);

    join_game_log_by_game_id(&mut select, game_id);
    add_order_by_start_datetime(&mut select);
    // TODO Distinct?
    select
}

fn join_game_log_by_device_id(select: &mut SelectStatement, device_id: &Uuid) {
    select
        .left_join(
            GameLogIden::Table,
            Expr::col((GameIden::Table, GameIden::UserId))
                .equals((GameLogIden::Table, GameLogIden::UserId))
                .and(
                    Expr::col((GameIden::Table, GameIden::Id))
                        .equals((GameLogIden::Table, GameLogIden::GameId)),
                ),
        )
        .and_where(
            Expr::col((GameLogIden::Table, GameLogIden::DeviceId))
                .eq(crate::uuid_utils::to_string(device_id)),
        );
}

fn join_game_log_by_game_id(select: &mut SelectStatement, game_id: &Uuid) {
    select
        .left_join(
            GameLogIden::Table,
            Expr::col((DeviceIden::Table, DeviceIden::UserId))
                .equals((GameLogIden::Table, GameLogIden::UserId))
                .and(
                    Expr::col((DeviceIden::Table, DeviceIden::Id))
                        .equals((GameLogIden::Table, GameLogIden::DeviceId)),
                ),
        )
        .and_where(
            Expr::col((GameLogIden::Table, GameLogIden::GameId))
                .eq(crate::uuid_utils::to_string(game_id)),
        );
}

fn add_order_by_start_datetime(select: &mut SelectStatement) {
    select.order_by((GameLogIden::Table, GameLogIden::StartDateTime), Order::Asc);
}
