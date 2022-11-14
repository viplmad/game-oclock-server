use chrono::NaiveDate;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameAvailableIden, GameIden, PlatformIden};

pub(super) fn join_game_available_by_platform_id(select: &mut SelectStatement, platform_id: i32) {
    select
        .column((GameAvailableIden::Table, GameAvailableIden::AddedDate))
        .left_join(
            GameAvailableIden::Table,
            Expr::tbl(GameIden::Table, GameIden::UserId)
                .equals(GameAvailableIden::Table, GameAvailableIden::UserId)
                .and(
                    Expr::tbl(GameIden::Table, GameIden::Id)
                        .equals(GameAvailableIden::Table, GameAvailableIden::GameId),
                ),
        )
        .and_where(
            Expr::col((GameAvailableIden::Table, GameAvailableIden::PlatformId)).eq(platform_id),
        );
}

pub(super) fn join_game_available_by_game_id(select: &mut SelectStatement, game_id: i32) {
    select
        .column((GameAvailableIden::Table, GameAvailableIden::AddedDate))
        .left_join(
            GameAvailableIden::Table,
            Expr::tbl(PlatformIden::Table, PlatformIden::UserId)
                .equals(GameAvailableIden::Table, GameAvailableIden::UserId)
                .and(
                    Expr::tbl(PlatformIden::Table, PlatformIden::Id)
                        .equals(GameAvailableIden::Table, GameAvailableIden::PlatformId),
                ),
        )
        .and_where(Expr::col((GameAvailableIden::Table, GameAvailableIden::GameId)).eq(game_id));
}

pub fn insert(
    user_id: i32,
    game_id: i32,
    platform_id: i32,
    added_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameAvailableIden::Table)
        .columns([
            GameAvailableIden::UserId,
            GameAvailableIden::GameId,
            GameAvailableIden::PlatformId,
            GameAvailableIden::AddedDate,
        ])
        .values_panic([
            user_id.into(),
            game_id.into(),
            platform_id.into(),
            added_date.into(),
        ]);

    insert
}

pub fn delete_by_id(
    user_id: i32,
    game_id: i32,
    platform_id: i32,
    added_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameAvailableIden::Table)
        .and_where(Expr::col(GameAvailableIden::UserId).eq(user_id))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id))
        .and_where(Expr::col(GameAvailableIden::PlatformId).eq(platform_id))
        .and_where(Expr::col(GameAvailableIden::AddedDate).eq(added_date));

    delete
}

pub fn exists_by_id(
    user_id: i32,
    game_id: i32,
    platform_id: i32,
    added_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameAvailableIden::Table, GameAvailableIden::GameId))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id))
        .and_where(Expr::col(GameAvailableIden::PlatformId).eq(platform_id))
        .and_where(Expr::col(GameAvailableIden::AddedDate).eq(added_date));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(GameAvailableIden::Table)
        .and_where(Expr::col((GameAvailableIden::Table, GameAvailableIden::UserId)).eq(user_id));
}
