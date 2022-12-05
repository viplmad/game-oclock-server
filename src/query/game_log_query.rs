use chrono::NaiveDateTime;
use sea_query::{Expr, Func, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameIden, GameLog, GameLogIden};

use super::game_query;

pub fn select_sum_time_by_user_id_and_game_id(
    user_id: i32,
    game_id: i32,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select.expr(Func::coalesce([
        Expr::col((GameLogIden::Table, GameLogIden::Time)).sum(),
        Expr::val("0 seconds").into(), // TODO
    ]));

    select
}

pub fn select_all_by_user_id_and_game_id(user_id: i32, game_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    add_datetime_and_time_fields(&mut select);

    select
}

pub fn select_all_games_order_by_datetime_desc(user_id: i32) -> SelectStatement {
    let mut select = game_query::select_all(user_id);

    join_game_log(&mut select);
    select.order_by((GameLogIden::Table, GameLogIden::DateTime), Order::Desc);

    select
}

pub fn select_all_games_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
    user_id: i32,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> SelectStatement {
    let mut select = select_all_games_order_by_datetime_desc(user_id);

    select
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::DateTime)).gte(start_datetime))
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::DateTime)).lte(end_datetime));

    select
}

pub fn select_all_games_log_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
    user_id: i32,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut select = select_all_games_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
        user_id,
        start_datetime,
        end_datetime,
    );

    add_datetime_and_time_fields(&mut select);

    select
}

pub fn insert(user_id: i32, game_id: i32, log: &GameLog) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    let secs = log.time.microseconds / 1_000_000; // TODO duplicate variable in DurationDef
    insert
        .into_table(GameLogIden::Table)
        .columns([
            GameLogIden::UserId,
            GameLogIden::GameId,
            GameLogIden::DateTime,
            GameLogIden::Time,
        ])
        .values_panic([
            user_id.into(),
            game_id.into(),
            log.datetime.into(),
            format!("{secs} seconds").into(), // TODO
        ]);

    insert
}

pub fn delete_by_id(
    user_id: i32,
    game_id: i32,
    datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameLogIden::Table)
        .and_where(Expr::col(GameLogIden::UserId).eq(user_id))
        .and_where(Expr::col(GameLogIden::GameId).eq(game_id))
        .and_where(Expr::col(GameLogIden::DateTime).eq(datetime));

    delete
}

pub fn exists_by_id(
    user_id: i32,
    game_id: i32,
    datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameLogIden::Table, GameLogIden::GameId))
        .and_where(Expr::col(GameLogIden::DateTime).eq(datetime));

    select
}

fn join_game_log(select: &mut SelectStatement) {
    select.left_join(
        GameLogIden::Table,
        Expr::tbl(GameIden::Table, GameIden::UserId)
            .equals(GameLogIden::Table, GameLogIden::UserId)
            .and(
                Expr::tbl(GameIden::Table, GameIden::Id)
                    .equals(GameLogIden::Table, GameLogIden::GameId),
            ),
    );
}

fn from_and_where_user_id_and_game_id(select: &mut SelectStatement, user_id: i32, game_id: i32) {
    select
        .from(GameLogIden::Table)
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::UserId)).eq(user_id))
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::GameId)).eq(game_id));
}

fn add_datetime_and_time_fields(select: &mut SelectStatement) {
    select
        .column((GameLogIden::Table, GameLogIden::DateTime))
        .column((GameLogIden::Table, GameLogIden::Time));
}
