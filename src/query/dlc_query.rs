use chrono::Utc;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{DLCIden, DLCSearch, SearchQuery, DLC};
use crate::errors::RepositoryError;

use super::search::apply_search;

pub fn select_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_by_base_game_id(user_id: i32, base_game_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);
    select.and_where(Expr::col(DLCIden::BaseGameId).eq(base_game_id));

    select
}

pub fn select_all_with_search(
    user_id: i32,
    search: DLCSearch,
) -> Result<SearchQuery, RepositoryError> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub(super) fn select_all(user_id: i32) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);

    select
}

pub fn insert(user_id: i32, dlc: &DLC) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(DLCIden::Table)
        .columns([
            DLCIden::UserId,
            DLCIden::Name,
            DLCIden::BaseGameId,
            DLCIden::ReleaseYear,
            DLCIden::CoverFilename,
            DLCIden::AddedDateTime,
            DLCIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            dlc.name.clone().into(),
            dlc.base_game_id.into(),
            dlc.release_year.into(),
            dlc.cover_filename.clone().into(),
            Utc::now().naive_utc().into(),
            Utc::now().naive_utc().into(),
        ])
        .returning(Query::returning().columns([DLCIden::Id]));

    insert
}

pub fn update_by_id(user_id: i32, id: i32, dlc: &DLC) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(DLCIden::Table)
        .values(vec![
            (DLCIden::Name, dlc.name.clone().into()),
            (DLCIden::BaseGameId, dlc.base_game_id.into()),
            (DLCIden::ReleaseYear, dlc.release_year.into()),
            (DLCIden::CoverFilename, dlc.cover_filename.clone().into()),
            (DLCIden::UpdatedDateTime, Utc::now().naive_utc().into()),
        ])
        .and_where(Expr::col(DLCIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCIden::Id).eq(id))
        .returning(Query::returning().columns([DLCIden::Id]));

    update
}

pub fn update_base_game_id_by_id(
    user_id: i32,
    id: i32,
    base_game_id: Option<i32>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(DLCIden::Table)
        .values(vec![(DLCIden::BaseGameId, base_game_id.into())])
        .and_where(Expr::col(DLCIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCIden::Id).eq(id))
        .returning(Query::returning().columns([DLCIden::Id]));

    update
}

pub fn delete_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(DLCIden::Table)
        .and_where(Expr::col(DLCIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCIden::Id).eq(id));

    delete
}

pub fn exists_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_name(user_id: i32, name: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select.and_where(Expr::col(DLCIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(user_id: i32, name: &str, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select
        .and_where(Expr::col(DLCIden::Name).eq(name))
        .and_where(Expr::col(DLCIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(DLCIden::Table)
        .and_where(Expr::col((DLCIden::Table, DLCIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: i32) {
    select.and_where(Expr::col((DLCIden::Table, DLCIden::Id)).eq(id));
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((DLCIden::Table, DLCIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((DLCIden::Table, DLCIden::UserId))
        .column((DLCIden::Table, DLCIden::Name))
        .column((DLCIden::Table, DLCIden::BaseGameId))
        .column((DLCIden::Table, DLCIden::ReleaseYear))
        .column((DLCIden::Table, DLCIden::CoverFilename))
        .column((DLCIden::Table, DLCIden::AddedDateTime))
        .column((DLCIden::Table, DLCIden::UpdatedDateTime));
}
