use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameIden, GameTagIden, TagIden};

use super::{game_query, tag_query};

pub fn select_all_games_by_tag_id(user_id: i32, tag_id: i32) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_tag_by_tag_id(&mut select, tag_id);

    select
}

pub fn select_all_tags_by_game_id(user_id: i32, game_id: i32) -> impl QueryStatementWriter {
    let mut select = tag_query::select_all(user_id);

    join_game_tag_by_game_id(&mut select, game_id);

    select
}

pub fn insert(user_id: i32, game_id: i32, tag_id: i32) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameTagIden::Table)
        .columns([GameTagIden::UserId, GameTagIden::GameId, GameTagIden::TagId])
        .values_panic([user_id.into(), game_id.into(), tag_id.into()]);

    insert
}

pub fn delete_by_id(user_id: i32, game_id: i32, tag_id: i32) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameTagIden::Table)
        .and_where(Expr::col(GameTagIden::UserId).eq(user_id))
        .and_where(Expr::col(GameTagIden::GameId).eq(game_id))
        .and_where(Expr::col(GameTagIden::TagId).eq(tag_id));

    delete
}

pub fn exists_by_id(user_id: i32, game_id: i32, tag_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameTagIden::Table, GameTagIden::UserId))
        .and_where(Expr::col(GameTagIden::GameId).eq(game_id))
        .and_where(Expr::col(GameTagIden::TagId).eq(tag_id));

    select
}

fn join_game_tag_by_tag_id(select: &mut SelectStatement, tag_id: i32) {
    select
        .left_join(
            GameTagIden::Table,
            Expr::col((GameIden::Table, GameIden::UserId))
                .equals((GameTagIden::Table, GameTagIden::UserId))
                .and(
                    Expr::col((GameIden::Table, GameIden::Id))
                        .equals((GameTagIden::Table, GameTagIden::GameId)),
                ),
        )
        .and_where(Expr::col((GameTagIden::Table, GameTagIden::TagId)).eq(tag_id));
}

fn join_game_tag_by_game_id(select: &mut SelectStatement, game_id: i32) {
    select
        .left_join(
            GameTagIden::Table,
            Expr::col((TagIden::Table, TagIden::UserId))
                .equals((GameTagIden::Table, GameTagIden::UserId))
                .and(
                    Expr::col((TagIden::Table, TagIden::Id))
                        .equals((GameTagIden::Table, GameTagIden::TagId)),
                ),
        )
        .and_where(Expr::col((GameTagIden::Table, GameTagIden::GameId)).eq(game_id));
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(GameTagIden::Table)
        .and_where(Expr::col((GameTagIden::Table, GameTagIden::UserId)).eq(user_id));
}
