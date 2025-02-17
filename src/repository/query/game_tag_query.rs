use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{GameIden, GameTag, GameTagIden, TagIden};

use super::{game_query, tag_query};

pub fn select_all_games_by_tag_id(user_id: &Uuid, tag_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = game_query::select_all(user_id);

    join_game_tag_by_tag_id(&mut select, tag_id);

    select
}

pub fn select_all_tags_by_game_id(user_id: &Uuid, game_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = tag_query::select_all(user_id);

    join_game_tag_by_game_id(&mut select, game_id);

    select
}

pub fn insert(game_tag: &GameTag) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameTagIden::Table)
        .columns([GameTagIden::UserId, GameTagIden::GameId, GameTagIden::TagId])
        .values_panic([
            crate::uuid_utils::to_string(&game_tag.user_id).into(),
            crate::uuid_utils::to_string(&game_tag.game_id).into(),
            crate::uuid_utils::to_string(&game_tag.tag_id).into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: &Uuid, game_id: &Uuid, tag_id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameTagIden::Table)
        .and_where(Expr::col(GameTagIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameTagIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameTagIden::TagId).eq(crate::uuid_utils::to_string(tag_id)));

    delete
}

pub fn exists_by_id(user_id: &Uuid, game_id: &Uuid, tag_id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameTagIden::Table, GameTagIden::UserId))
        .and_where(Expr::col(GameTagIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameTagIden::TagId).eq(crate::uuid_utils::to_string(tag_id)));

    select
}

fn join_game_tag_by_tag_id(select: &mut SelectStatement, tag_id: &Uuid) {
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
        .and_where(
            Expr::col((GameTagIden::Table, GameTagIden::TagId))
                .eq(crate::uuid_utils::to_string(tag_id)),
        );
}

fn join_game_tag_by_game_id(select: &mut SelectStatement, game_id: &Uuid) {
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
        .and_where(
            Expr::col((GameTagIden::Table, GameTagIden::GameId))
                .eq(crate::uuid_utils::to_string(game_id)),
        );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GameTagIden::Table).and_where(
        Expr::col((GameTagIden::Table, GameTagIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}
