use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement, SimpleExpr};

use crate::entities::{SearchQuery, Tag, TagIden, TagSearch};
use crate::errors::SearchErrors;

use super::search::apply_search;

pub fn select_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_with_query(
    user_id: &str,
    search: TagSearch,
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

pub fn insert(user_id: &str, id: &str, tag: &Tag) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(TagIden::Table)
        .columns([
            TagIden::UserId,
            TagIden::Id,
            TagIden::Name,
            TagIden::AddedDateTime,
            TagIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            id.into(),
            tag.name.clone().into(),
            crate::date_utils::now().into(),
            crate::date_utils::now().into(),
        ]);

    insert
}

pub fn update_by_id(user_id: &str, id: &str, tag: &Tag) -> impl QueryStatementWriter {
    update_values_by_id(user_id, id, vec![(TagIden::Name, tag.name.clone().into())])
}

fn update_values_by_id(
    user_id: &str,
    id: &str,
    mut values: Vec<(TagIden, SimpleExpr)>,
) -> impl QueryStatementWriter {
    let mut update = Query::update();

    values.push((TagIden::UpdatedDateTime, crate::date_utils::now().into()));
    update
        .table(TagIden::Table)
        .values(values)
        .and_where(Expr::col(TagIden::UserId).eq(user_id))
        .and_where(Expr::col(TagIden::Id).eq(id));

    update
}

pub fn delete_by_id(user_id: &str, id: &str) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(TagIden::Table)
        .and_where(Expr::col(TagIden::UserId).eq(user_id))
        .and_where(Expr::col(TagIden::Id).eq(id));

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
    select.and_where(Expr::col(TagIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(user_id: &str, name: &str, id: &str) -> impl QueryStatementWriter {
    let mut select = exists_by_name(user_id, name);

    select.and_where(Expr::col(TagIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(TagIden::Table)
        .and_where(Expr::col((TagIden::Table, TagIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: &str) {
    select.and_where(Expr::col((TagIden::Table, TagIden::Id)).eq(id));
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((TagIden::Table, TagIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((TagIden::Table, TagIden::UserId))
        .column((TagIden::Table, TagIden::Name))
        .column((TagIden::Table, TagIden::AddedDateTime))
        .column((TagIden::Table, TagIden::UpdatedDateTime));
}
