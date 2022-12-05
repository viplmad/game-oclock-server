use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};

use crate::entities::{Platform, PlatformIden, PlatformSearch, SearchQuery};
use crate::errors::RepositoryError;

use super::search::apply_search;

pub fn select_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: i32,
    search: PlatformSearch,
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

pub fn insert(user_id: i32, platform: &Platform) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(PlatformIden::Table)
        .columns([
            PlatformIden::UserId,
            PlatformIden::Name,
            PlatformIden::Type,
            PlatformIden::IconFilename,
            PlatformIden::AddedDateTime,
            PlatformIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            platform.name.clone().into(),
            platform.ptype.into(),
            platform.icon_filename.clone().into(),
            crate::utils::now().into(),
            crate::utils::now().into(),
        ])
        .returning(Query::returning().columns([PlatformIden::Id]));

    insert
}

pub fn update_by_id(user_id: i32, id: i32, platform: &Platform) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![
            (PlatformIden::Name, platform.name.clone().into()),
            (PlatformIden::Type, platform.ptype.into()),
            (
                PlatformIden::IconFilename,
                platform.icon_filename.clone().into(),
            ),
        ],
    )
}

pub fn update_icon_filename_by_id(
    user_id: i32,
    id: i32,
    cover_filename: &str,
) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![(PlatformIden::IconFilename, cover_filename.into())],
    )
}

fn update_values_by_id(
    user_id: i32,
    id: i32,
    mut values: Vec<(PlatformIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    values.push((PlatformIden::UpdatedDateTime, crate::utils::now().into()));
    update
        .table(PlatformIden::Table)
        .values(values)
        .and_where(Expr::col(PlatformIden::UserId).eq(user_id))
        .and_where(Expr::col(PlatformIden::Id).eq(id))
        .returning(Query::returning().columns([PlatformIden::Id]));

    update
}

pub fn delete_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(PlatformIden::Table)
        .and_where(Expr::col(PlatformIden::UserId).eq(user_id))
        .and_where(Expr::col(PlatformIden::Id).eq(id));

    delete
}

pub fn exists_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_name(user_id: i32, name: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select.and_where(Expr::col(PlatformIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(user_id: i32, name: &str, id: i32) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(PlatformIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(PlatformIden::Table)
        .and_where(Expr::col((PlatformIden::Table, PlatformIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: i32) {
    select.and_where(Expr::col((PlatformIden::Table, PlatformIden::Id)).eq(id));
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((PlatformIden::Table, PlatformIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((PlatformIden::Table, PlatformIden::UserId))
        .column((PlatformIden::Table, PlatformIden::Name))
        .column((PlatformIden::Table, PlatformIden::Type))
        .column((PlatformIden::Table, PlatformIden::IconFilename))
        .column((PlatformIden::Table, PlatformIden::AddedDateTime))
        .column((PlatformIden::Table, PlatformIden::UpdatedDateTime));
}
