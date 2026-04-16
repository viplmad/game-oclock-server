use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};
use uuid::Uuid;

use crate::entities::{Device, DeviceIden, DeviceListSearch, SearchQuery};
use crate::errors::SearchErrors;

use super::search::{apply_search, apply_search_filter};

pub fn select_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_with_search(
    user_id: &Uuid,
    search: DeviceListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub fn count_all_with_search(
    user_id: &Uuid,
    search: DeviceListSearch,
) -> Result<SelectStatement, SearchErrors> {
    let select = count_all(user_id);

    apply_search_filter(select, search)
}

pub(super) fn select_all(user_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);

    select
}

pub(super) fn count_all(user_id: &Uuid) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select.expr(Expr::col((DeviceIden::Table, DeviceIden::Id)).count());

    select
}

pub fn insert(device: &Device) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(DeviceIden::Table)
        .columns([
            DeviceIden::Id,
            DeviceIden::UserId,
            DeviceIden::Name,
            DeviceIden::ImageUrl,
            DeviceIden::AddedDatetime,
            DeviceIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&device.id).into(),
            crate::uuid_utils::to_string(&device.user_id).into(),
            device.name.clone().into(),
            device.image_url.clone().into(),
            device.added_datetime.into(),
            device.updated_datetime.into(),
        ]);

    insert
}

pub fn update_by_id(device: &Device) -> impl QueryStatementWriter {
    update_values_by_id(
        &device.user_id,
        &device.id,
        vec![
            (DeviceIden::Name, device.name.clone().into()),
            (DeviceIden::ImageUrl, device.image_url.clone().into()),
            (DeviceIden::UpdatedDatetime, device.updated_datetime.into()),
        ],
    )
}

fn update_values_by_id(
    user_id: &Uuid,
    id: &Uuid,
    values: Vec<(DeviceIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(DeviceIden::Table)
        .values(values)
        .and_where(Expr::col(DeviceIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(DeviceIden::Id).eq(crate::uuid_utils::to_string(id)));

    update
}

pub fn delete_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(DeviceIden::Table)
        .and_where(Expr::col(DeviceIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(DeviceIden::Id).eq(crate::uuid_utils::to_string(id)));

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
    select.and_where(Expr::col(DeviceIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(
    user_id: &Uuid,
    name: &str,
    id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(DeviceIden::Id).ne(crate::uuid_utils::to_string(id)));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(DeviceIden::Table).and_where(
        Expr::col((DeviceIden::Table, DeviceIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_id(select: &mut SelectStatement, id: &Uuid) {
    select.and_where(
        Expr::col((DeviceIden::Table, DeviceIden::Id)).eq(crate::uuid_utils::to_string(id)),
    );
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((DeviceIden::Table, DeviceIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((DeviceIden::Table, DeviceIden::UserId))
        .column((DeviceIden::Table, DeviceIden::Name))
        .column((DeviceIden::Table, DeviceIden::ImageUrl))
        .column((DeviceIden::Table, DeviceIden::AddedDatetime))
        .column((DeviceIden::Table, DeviceIden::UpdatedDatetime));
}
