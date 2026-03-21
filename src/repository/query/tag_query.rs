use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};
use uuid::Uuid;

use crate::entities::{SearchQuery, Tag, TagIden, TagSearch};
use crate::errors::SearchErrors;

use super::search::{apply_search, apply_search_filter};

pub fn select_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_with_query(
    user_id: &Uuid,
    search: TagSearch,
) -> Result<SearchQuery, SearchErrors> {
    let select = select_all(user_id);

    apply_search(select, search)
}

pub fn count_all_with_query(
    user_id: &Uuid,
    search: TagSearch,
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
    select.expr(Expr::col((TagIden::Table, TagIden::Id)).count());

    select
}

pub fn insert(tag: &Tag) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(TagIden::Table)
        .columns([
            TagIden::Id,
            TagIden::UserId,
            TagIden::Name,
            TagIden::AddedDatetime,
            TagIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&tag.id).into(),
            crate::uuid_utils::to_string(&tag.user_id).into(),
            tag.name.clone().into(),
            tag.added_datetime.into(),
            tag.updated_datetime.into(),
        ]);

    insert
}

pub fn update_by_id(tag: &Tag) -> impl QueryStatementWriter {
    update_values_by_id(
        &tag.user_id,
        &tag.id,
        vec![
            (TagIden::Name, tag.name.clone().into()),
            (TagIden::UpdatedDatetime, tag.updated_datetime.into()),
        ],
    )
}

fn update_values_by_id(
    user_id: &Uuid,
    id: &Uuid,
    values: Vec<(TagIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(TagIden::Table)
        .values(values)
        .and_where(Expr::col(TagIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(TagIden::Id).eq(crate::uuid_utils::to_string(id)));

    update
}

pub fn delete_by_id(user_id: &Uuid, id: &Uuid) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(TagIden::Table)
        .and_where(Expr::col(TagIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(TagIden::Id).eq(crate::uuid_utils::to_string(id)));

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
    select.and_where(Expr::col(TagIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(
    user_id: &Uuid,
    name: &str,
    id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(TagIden::Id).ne(crate::uuid_utils::to_string(id)));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(TagIden::Table).and_where(
        Expr::col((TagIden::Table, TagIden::UserId)).eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_id(select: &mut SelectStatement, id: &Uuid) {
    select.and_where(Expr::col((TagIden::Table, TagIden::Id)).eq(crate::uuid_utils::to_string(id)));
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((TagIden::Table, TagIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((TagIden::Table, TagIden::UserId))
        .column((TagIden::Table, TagIden::Name))
        .column((TagIden::Table, TagIden::AddedDatetime))
        .column((TagIden::Table, TagIden::UpdatedDatetime));
}
