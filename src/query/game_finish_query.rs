use chrono::NaiveDate;
use sea_query::{Expr, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameFinishIden, GameIden};

use super::game_query;

pub fn select_all_by_user_id_and_game_id(user_id: i32, game_id: i32) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    add_date_field(&mut select);

    select
}

pub fn select_all_games_order_by_date(user_id: i32) -> SelectStatement {
    let mut select = game_query::select_all(user_id);

    join_game_finish(&mut select);
    select.order_by((GameFinishIden::Table, GameFinishIden::Date), Order::Desc);

    select
}

pub fn select_all_games_by_date_gte_and_date_lte_order_by_date(
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut select = select_all_games_order_by_date(user_id);

    select
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).gte(start_date))
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).lte(end_date));

    select
}

pub fn select_first_by_user_id_and_game_id(
    user_id: i32,
    game_id: i32,
) -> impl QueryStatementWriter {
    let mut select = select_all_by_user_id_and_game_id(user_id, game_id);

    select.order_by(GameFinishIden::Date, Order::Asc);
    select.limit(1);

    select
}

pub fn insert(user_id: i32, game_id: i32, date: NaiveDate) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameFinishIden::Table)
        .columns([
            GameFinishIden::UserId,
            GameFinishIden::GameId,
            GameFinishIden::Date,
        ])
        .values_panic([user_id.into(), game_id.into(), date.into()]);

    insert
}

pub fn delete_by_id(user_id: i32, game_id: i32, date: NaiveDate) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameFinishIden::Table)
        .and_where(Expr::col(GameFinishIden::UserId).eq(user_id))
        .and_where(Expr::col(GameFinishIden::GameId).eq(game_id))
        .and_where(Expr::col(GameFinishIden::Date).eq(date));

    delete
}

pub fn exists_by_id(user_id: i32, game_id: i32, date: NaiveDate) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameFinishIden::Table, GameFinishIden::GameId))
        .and_where(Expr::col(GameFinishIden::Date).eq(date));

    select
}

fn join_game_finish(select: &mut SelectStatement) {
    select.left_join(
        GameFinishIden::Table,
        Expr::tbl(GameIden::Table, GameIden::UserId)
            .equals(GameFinishIden::Table, GameFinishIden::UserId)
            .and(
                Expr::tbl(GameIden::Table, GameIden::Id)
                    .equals(GameFinishIden::Table, GameFinishIden::GameId),
            ),
    );
}

fn from_and_where_user_id_and_game_id(select: &mut SelectStatement, user_id: i32, game_id: i32) {
    select
        .from(GameFinishIden::Table)
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::UserId)).eq(user_id))
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::GameId)).eq(game_id));
}

fn add_date_field(select: &mut SelectStatement) {
    select.column((GameFinishIden::Table, GameFinishIden::Date));
}
