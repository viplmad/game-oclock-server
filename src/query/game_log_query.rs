use chrono::NaiveDateTime;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameLog, GameLogIden};

pub fn select_all_by_user_id_and_game_id(user_id: i32, game_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameLogIden::Table, GameLogIden::DateTime))
        .column((GameLogIden::Table, GameLogIden::Time))
        .and_where(Expr::col(GameLogIden::GameId).eq(game_id));

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

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameLogIden::Table, GameLogIden::GameId))
        .and_where(Expr::col(GameLogIden::GameId).eq(game_id))
        .and_where(Expr::col(GameLogIden::DateTime).eq(datetime));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(GameLogIden::Table)
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::UserId)).eq(user_id));
}
