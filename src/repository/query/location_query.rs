use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};
use uuid::Uuid;

use crate::entities::{Location, LocationIden, LocationSearch, SearchQuery};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: &Uuid,
    search: LocationSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub(super) fn select_all(user_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);

    select
}

pub fn insert(location: &Location) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(LocationIden::Table)
        .columns([
            LocationIden::Id,
            LocationIden::UserId,
            LocationIden::Name,
            LocationIden::IconUrl,
            LocationIden::AddedDatetime,
            LocationIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&location.id).into(),
            crate::uuid_utils::to_string(&location.user_id).into(),
            location.name.clone().into(),
            location.icon_url.clone().into(),
            location.added_datetime.into(),
            location.updated_datetime.into(),
        ]);

    insert
}

pub fn update_by_id(location: &Location) -> impl QueryStatementWriter {
    update_values_by_id(
        &location.user_id,
        &location.id,
        vec![
            (LocationIden::Name, location.name.clone().into()),
            (LocationIden::IconUrl, location.icon_url.clone().into()),
            (
                LocationIden::UpdatedDatetime,
                location.updated_datetime.into(),
            ),
        ],
    )
}

fn update_values_by_id(
    user_id: &Uuid,
    id: &Uuid,
    values: Vec<(LocationIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(LocationIden::Table)
        .values(values)
        .and_where(Expr::col(LocationIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(LocationIden::Id).eq(crate::uuid_utils::to_string(id)));

    update
}

pub fn delete_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(LocationIden::Table)
        .and_where(Expr::col(LocationIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(LocationIden::Id).eq(crate::uuid_utils::to_string(id)));

    delete
}

pub fn exists_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_id_field(&mut select);

    select
}

pub fn exists_by_name(user_id: &Uuid, name: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select.and_where(Expr::col(LocationIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(
    user_id: &Uuid,
    name: &str,
    id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(LocationIden::Id).ne(crate::uuid_utils::to_string(id)));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(LocationIden::Table).and_where(
        Expr::col((LocationIden::Table, LocationIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_id(select: &mut SelectStatement, id: &Uuid) {
    select.and_where(
        Expr::col((LocationIden::Table, LocationIden::Id)).eq(crate::uuid_utils::to_string(id)),
    );
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((LocationIden::Table, LocationIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((LocationIden::Table, LocationIden::UserId))
        .column((LocationIden::Table, LocationIden::Name))
        .column((LocationIden::Table, LocationIden::IconUrl))
        .column((LocationIden::Table, LocationIden::AddedDatetime))
        .column((LocationIden::Table, LocationIden::UpdatedDatetime));
}
