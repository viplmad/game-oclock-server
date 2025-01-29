use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameLinkIden, Link};

pub fn select_all_by_user_id_and_game_id(user_id: &str, game_id: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    add_url_and_description_fields(&mut select);

    select
}

pub fn insert(user_id: &str, game_id: &str, link: &Link) -> impl QueryStatementWriter {
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
            user_id.into(),
            game_id.into(),
            link.url.clone().into(),
            link.description.clone().into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: &str, game_id: &str, url: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameLinkIden::Table)
        .and_where(Expr::col(GameLinkIden::UserId).eq(user_id))
        .and_where(Expr::col(GameLinkIden::GameId).eq(game_id))
        .and_where(Expr::col(GameLinkIden::Url).eq(url));

    delete
}

pub fn exists_by_id(user_id: &str, game_id: &str, url: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameLinkIden::Table, GameLinkIden::GameId))
        .and_where(Expr::col(GameLinkIden::Url).eq(url));

    select
}

fn from_and_where_user_id_and_game_id(select: &mut SelectStatement, user_id: &str, game_id: &str) {
    from_and_where_user_id(select, user_id);
    select.and_where(Expr::col((GameLinkIden::Table, GameLinkIden::GameId)).eq(game_id));
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(GameLinkIden::Table)
        .and_where(Expr::col((GameLinkIden::Table, GameLinkIden::UserId)).eq(user_id));
}

fn add_url_and_description_fields(select: &mut SelectStatement) {
    select
        .column((GameLinkIden::Table, GameLinkIden::Url))
        .column((GameLinkIden::Table, GameLinkIden::Description));
}
