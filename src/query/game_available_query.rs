use chrono::NaiveDate;
use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameAvailableIden, GameIden, PlatformIden, QUERY_DATE_ALIAS};

use super::{game_query, platform_query};

pub fn select_all_games_by_platform_id_order_by_added_date(
    user_id: &str,
    platform_id: &str,
) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_available_by_platform_id(&mut select, platform_id);
    add_fields(&mut select);
    add_order_by_added_date(&mut select);

    select
}

pub fn select_all_platforms_by_game_id_order_by_added_date(
    user_id: &str,
    game_id: &str,
) -> impl QueryStatementWriter {
    let mut select = platform_query::select_all(user_id);

    join_game_available_by_game_id(&mut select, game_id);
    add_fields(&mut select);
    add_order_by_added_date(&mut select);

    select
}

pub fn insert(
    user_id: &str,
    game_id: &str,
    platform_id: &str,
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

pub fn delete_by_id(user_id: &str, game_id: &str, platform_id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameAvailableIden::Table)
        .and_where(Expr::col(GameAvailableIden::UserId).eq(user_id))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id))
        .and_where(Expr::col(GameAvailableIden::PlatformId).eq(platform_id));

    delete
}

pub fn exists_by_id(user_id: &str, game_id: &str, platform_id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameAvailableIden::Table, GameAvailableIden::UserId))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id))
        .and_where(Expr::col(GameAvailableIden::PlatformId).eq(platform_id));

    select
}

pub fn exists_platforms_by_game_id(user_id: &str, game_id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameAvailableIden::Table, GameAvailableIden::UserId))
        .and_where(Expr::col(GameAvailableIden::GameId).eq(game_id));

    select
}

fn join_game_available_by_platform_id(select: &mut SelectStatement, platform_id: &str) {
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
            Expr::col((GameAvailableIden::Table, GameAvailableIden::PlatformId)).eq(platform_id),
        );
}

fn join_game_available_by_game_id(select: &mut SelectStatement, game_id: &str) {
    select
        .left_join(
            GameAvailableIden::Table,
            Expr::col((PlatformIden::Table, PlatformIden::UserId))
                .equals((GameAvailableIden::Table, GameAvailableIden::UserId))
                .and(
                    Expr::col((PlatformIden::Table, PlatformIden::Id))
                        .equals((GameAvailableIden::Table, GameAvailableIden::PlatformId)),
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
        Expr::col((GameAvailableIden::Table, GameAvailableIden::AddedDate)),
        Alias::new(QUERY_DATE_ALIAS),
    );
}

fn add_order_by_added_date(select: &mut SelectStatement) {
    select.order_by(
        (GameAvailableIden::Table, GameAvailableIden::AddedDate),
        Order::Asc,
    );
}
