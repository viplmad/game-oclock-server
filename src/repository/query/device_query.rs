use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};

use crate::entities::{Device, DeviceIden, DeviceSearch, SearchQuery};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: &str,
    search: DeviceSearch,
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

pub fn insert(user_id: &str, id: &str, device: &Device) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(DeviceIden::Table)
        .columns([
            DeviceIden::UserId,
            DeviceIden::Id,
            DeviceIden::Name,
            DeviceIden::IconUrl,
            DeviceIden::AddedDateTime,
            DeviceIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            id.into(),
            device.name.clone().into(),
            device.icon_url.clone().into(),
            crate::date_utils::now().into(),
            crate::date_utils::now().into(),
        ]);

    insert
}

pub fn update_by_id(user_id: &str, id: &str, device: &Device) -> impl QueryStatementWriter {
    update_values_by_id(
        user_id,
        id,
        vec![
            (DeviceIden::Name, device.name.clone().into()),
            (DeviceIden::IconUrl, device.icon_url.clone().into()),
        ],
    )
}

fn update_values_by_id(
    user_id: &str,
    id: &str,
    mut values: Vec<(DeviceIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    values.push((DeviceIden::UpdatedDateTime, crate::date_utils::now().into()));
    update
        .table(DeviceIden::Table)
        .values(values)
        .and_where(Expr::col(DeviceIden::UserId).eq(user_id))
        .and_where(Expr::col(DeviceIden::Id).eq(id));

    update
}

pub fn delete_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(DeviceIden::Table)
        .and_where(Expr::col(DeviceIden::UserId).eq(user_id))
        .and_where(Expr::col(DeviceIden::Id).eq(id));

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
    select.and_where(Expr::col(DeviceIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(user_id: &str, name: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(DeviceIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(DeviceIden::Table)
        .and_where(Expr::col((DeviceIden::Table, DeviceIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: &str) {
    select.and_where(Expr::col((DeviceIden::Table, DeviceIden::Id)).eq(id));
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((DeviceIden::Table, DeviceIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((DeviceIden::Table, DeviceIden::UserId))
        .column((DeviceIden::Table, DeviceIden::Name))
        .column((DeviceIden::Table, DeviceIden::IconUrl))
        .column((DeviceIden::Table, DeviceIden::AddedDateTime))
        .column((DeviceIden::Table, DeviceIden::UpdatedDateTime));
}
