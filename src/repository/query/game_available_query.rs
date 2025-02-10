use chrono::NaiveDate;
use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameAvailableIden, GameIden, LocationIden, QUERY_DATE_ALIAS};

use super::{game_query, location_query};

pub fn select_all_games_by_location_id_order_by_date(
    user_id: &str,
    location_id: &str,
) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_available_by_location_id(&mut select, location_id);
    add_fields(&mut select);
    add_order_by_date(&mut select);

    select
}

pub fn select_all_locations_by_game_id_order_by_date(
    user_id: &str,
    game_id: &str,
) -> impl QueryStatementWriter {
    let mut select = location_query::select_all(user_id);

    join_game_available_by_game_id(&mut select, game_id);
    add_fields(&mut select);
    add_order_by_date(&mut select);

    select
}

pub fn insert(
    user_id: &str,
    game_id: &str,
    location_id: &str,
    date: NaiveDate,
) -> impl QueryStatementWriter {
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
            user_id.into(),
            game_id.into(),
            location_id.into(),
            date.into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: &str, game_id: &str, location_id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameAvailableIden::Table)
        .and_where(Expr::col(GameAvailableIden::UserId).eq(user_id))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id))
        .and_where(Expr::col(GameAvailableIden::LocationId).eq(location_id));

    delete
}

pub fn exists_by_id(user_id: &str, game_id: &str, location_id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameAvailableIden::Table, GameAvailableIden::UserId))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id))
        .and_where(Expr::col(GameAvailableIden::LocationId).eq(location_id));

    select
}

pub fn exists_locations_by_game_id(user_id: &str, game_id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameAvailableIden::Table, GameAvailableIden::UserId))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id));

    select
}

fn join_game_available_by_location_id(select: &mut SelectStatement, location_id: &str) {
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
            Expr::col((GameAvailableIden::Table, GameAvailableIden::LocationId)).eq(location_id),
        );
}

fn join_game_available_by_game_id(select: &mut SelectStatement, game_id: &str) {
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
        .and_where(Expr::col((GameAvailableIden::Table, GameAvailableIden::GameId)).eq(game_id));
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(GameAvailableIden::Table)
        .and_where(Expr::col((GameAvailableIden::Table, GameAvailableIden::UserId)).eq(user_id));
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
