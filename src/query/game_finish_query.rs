use chrono::NaiveDate;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::GameFinishIden;

pub fn select_all_by_user_id_and_game_id(user_id: i32, game_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameFinishIden::Table, GameFinishIden::Date))
        .and_where(Expr::col(GameFinishIden::GameId).eq(game_id));

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

pub fn delete(user_id: i32, game_id: i32, date: NaiveDate) -> impl QueryStatementWriter {
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

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameFinishIden::Table, GameFinishIden::Date))
        .and_where(Expr::col(GameFinishIden::GameId).eq(game_id))
        .and_where(Expr::col(GameFinishIden::Date).eq(date));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(GameFinishIden::Table)
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::UserId)).eq(user_id));
}