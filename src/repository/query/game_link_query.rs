use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{GameLink, GameLinkIden};

pub fn select_all_by_user_id_and_game_id(user_id: &Uuid, game_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    add_fields(&mut select);

    select
}

pub fn insert(game_link: &GameLink) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameLinkIden::Table)
        .columns([
            GameLinkIden::UserId,
            GameLinkIden::GameId,
            GameLinkIden::Url,
            GameLinkIden::Description,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game_link.user_id).into(),
            crate::uuid_utils::to_string(&game_link.game_id).into(),
            game_link.url.clone().into(),
            game_link.description.clone().into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: &Uuid, game_id: &Uuid, url: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameLinkIden::Table)
        .and_where(Expr::col(GameLinkIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameLinkIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameLinkIden::Url).eq(url));

    delete
}

pub fn exists_by_id(user_id: &Uuid, game_id: &Uuid, url: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameLinkIden::Table, GameLinkIden::GameId))
        .and_where(Expr::col(GameLinkIden::Url).eq(url));

    select
}

fn from_and_where_user_id_and_game_id(
    select: &mut SelectStatement,
    user_id: &Uuid,
    game_id: &Uuid,
) {
    from_and_where_user_id(select, user_id);
    select.and_where(
        Expr::col((GameLinkIden::Table, GameLinkIden::GameId))
            .eq(crate::uuid_utils::to_string(game_id)),
    );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GameLinkIden::Table).and_where(
        Expr::col((GameLinkIden::Table, GameLinkIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn add_fields(select: &mut SelectStatement) {
    select
        .column((GameLinkIden::Table, GameLinkIden::UserId))
        .column((GameLinkIden::Table, GameLinkIden::GameId))
        .column((GameLinkIden::Table, GameLinkIden::Url))
        .column((GameLinkIden::Table, GameLinkIden::Description));
}
