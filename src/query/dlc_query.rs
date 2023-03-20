use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};

use crate::entities::{DLCIden, DLCSearch, SearchQuery, DLC};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_by_base_game_id(user_id: &str, base_game_id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);
    select.and_where(Expr::col(DLCIden::BaseGameId).eq(base_game_id));

    select
}

pub fn select_all_with_search(
    user_id: &str,
    search: DLCSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub(super) fn select_all(user_id: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);

    select
}

pub(super) fn select_all_group_by_id(user_id: &str) -> SelectStatement {
    let mut select = select_all(user_id);

    select.group_by_col((DLCIden::Table, DLCIden::Id));

    select
}

pub fn insert(user_id: &str, id: &str, dlc: &DLC) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(DLCIden::Table)
        .columns([
            DLCIden::UserId,
            DLCIden::Id,
            DLCIden::Name,
            DLCIden::BaseGameId,
            DLCIden::ReleaseYear,
            DLCIden::CoverFilename,
            DLCIden::AddedDateTime,
            DLCIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            id.into(),
            dlc.name.clone().into(),
            dlc.base_game_id.map(|id| id.to_string()).into(),
            dlc.release_year.into(),
            dlc.cover_filename.clone().into(),
            crate::date_utils::now().into(),
            crate::date_utils::now().into(),
        ]);

    insert
}

pub fn update_by_id(user_id: &str, id: &str, dlc: &DLC) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![
            (DLCIden::Name, dlc.name.clone().into()),
            (
                DLCIden::BaseGameId,
                dlc.base_game_id.map(|id| id.to_string()).into(),
            ),
            (DLCIden::ReleaseYear, dlc.release_year.into()),
            (DLCIden::CoverFilename, dlc.cover_filename.clone().into()),
        ],
    )
}

pub fn update_base_game_id_by_id(
    user_id: &str,
    id: &str,
    base_game_id: Option<String>,
) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![(DLCIden::BaseGameId, base_game_id.into())],
    )
}

pub fn update_cover_filename_by_id(
    user_id: &str,
    id: &str,
    cover_filename: Option<String>,
) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![(DLCIden::CoverFilename, cover_filename.into())],
    )
}

fn update_values_by_id(
    user_id: &str,
    id: &str,
    mut values: Vec<(DLCIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    values.push((DLCIden::UpdatedDateTime, crate::date_utils::now().into()));
    update
        .table(DLCIden::Table)
        .values(values)
        .and_where(Expr::col(DLCIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCIden::Id).eq(id));

    update
}

pub fn delete_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(DLCIden::Table)
        .and_where(Expr::col(DLCIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCIden::Id).eq(id));

    delete
}

pub fn exists_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_name(user_id: &str, name: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select.and_where(Expr::col(DLCIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(user_id: &str, name: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(DLCIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(DLCIden::Table)
        .and_where(Expr::col((DLCIden::Table, DLCIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: &str) {
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
