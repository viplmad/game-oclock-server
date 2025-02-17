use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{GameAvailable, GameAvailableIden, GameIden, LocationIden, QUERY_DATE_ALIAS};

use super::{game_query, location_query};

pub fn select_all_games_by_location_id_order_by_date(
    user_id: &Uuid,
    location_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_available_by_location_id(&mut select, location_id);
    add_fields(&mut select);
    add_order_by_date(&mut select);

    select
}

pub fn select_all_locations_by_game_id_order_by_date(
    user_id: &Uuid,
    game_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = location_query::select_all(user_id);

    join_game_available_by_game_id(&mut select, game_id);
    add_fields(&mut select);
    add_order_by_date(&mut select);

    select
}

pub fn insert(game_available: &GameAvailable) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameAvailableIden::Table)
        .columns([
            GameAvailableIden::UserId,
            GameAvailableIden::GameId,
            GameAvailableIden::LocationId,
            GameAvailableIden::Date,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game_available.user_id).into(),
            crate::uuid_utils::to_string(&game_available.game_id).into(),
            crate::uuid_utils::to_string(&game_available.location_id).into(),
            game_available.date.into(),
        ]);

    insert
}

pub fn delete_by_id(
    user_id: &Uuid,
    game_id: &Uuid,
    location_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameAvailableIden::Table)
        .and_where(Expr::col(GameAvailableIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(
            Expr::col(GameAvailableIden::LocationId).eq(crate::uuid_utils::to_string(location_id)),
        );

    delete
}

pub fn exists_by_id(
    user_id: &Uuid,
    game_id: &Uuid,
    location_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameAvailableIden::Table, GameAvailableIden::UserId))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(
            Expr::col(GameAvailableIden::LocationId).eq(crate::uuid_utils::to_string(location_id)),
        );

    select
}

pub fn exists_locations_by_game_id(user_id: &Uuid, game_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameAvailableIden::Table, GameAvailableIden::UserId))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(crate::uuid_utils::to_string(game_id)));

    select
}

fn join_game_available_by_location_id(select: &mut SelectStatement, location_id: &Uuid) {
    select
        .left_join(
            GameAvailableIden::Table,
            Expr::col((GameIden::Table, GameIden::UserId))
                .equals((GameAvailableIden::Table, GameAvailableIden::UserId))
                .and(
                    Expr::col((GameIden::Table, GameIden::Id))
                        .equals((GameAvailableIden::Table, GameAvailableIden::GameId)),
                ),
        )
        .and_where(
            Expr::col((GameAvailableIden::Table, GameAvailableIden::LocationId))
                .eq(crate::uuid_utils::to_string(location_id)),
        );
}

fn join_game_available_by_game_id(select: &mut SelectStatement, game_id: &Uuid) {
    select
        .left_join(
            GameAvailableIden::Table,
            Expr::col((LocationIden::Table, LocationIden::UserId))
                .equals((GameAvailableIden::Table, GameAvailableIden::UserId))
                .and(
                    Expr::col((LocationIden::Table, LocationIden::Id))
                        .equals((GameAvailableIden::Table, GameAvailableIden::LocationId)),
                ),
        )
        .and_where(
            Expr::col((GameAvailableIden::Table, GameAvailableIden::GameId))
                .eq(crate::uuid_utils::to_string(game_id)),
        );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GameAvailableIden::Table).and_where(
        Expr::col((GameAvailableIden::Table, GameAvailableIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn add_fields(select: &mut SelectStatement) {
    select.expr_as(
        Expr::col((GameAvailableIden::Table, GameAvailableIden::Date)),
        Alias::new(QUERY_DATE_ALIAS),
    );
}

fn add_order_by_date(select: &mut SelectStatement) {
    select.order_by(
        (GameAvailableIden::Table, GameAvailableIden::Date),
        Order::Asc,
    );
}
