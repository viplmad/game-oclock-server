use chrono::Utc;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{Tag, TagIden};

pub fn select_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_all_by_query(user_id: i32, limit: u64) -> impl QueryStatementWriter {
    let mut select = select_all(user_id);

    select.limit(limit);

    select
}

pub(super) fn select_all(user_id: i32) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_fields(&mut select);

    select
}

pub fn insert(user_id: i32, tag: &Tag) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(TagIden::Table)
        .columns([
            TagIden::UserId,
            TagIden::Name,
            TagIden::AddedDateTime,
            TagIden::UpdatedDateTime,
        ])
        .values_panic([
            user_id.into(),
            tag.name.clone().into(),
            Utc::now().naive_utc().into(),
            Utc::now().naive_utc().into(),
        ])
        .returning(Query::returning().columns([TagIden::Id]));

    insert
}

pub fn update_by_id(user_id: i32, id: i32, tag: &Tag) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(TagIden::Table)
        .values(vec![
            (TagIden::Name, tag.name.clone().into()),
            (TagIden::UpdatedDateTime, Utc::now().naive_utc().into()),
        ])
        .and_where(Expr::col(TagIden::UserId).eq(user_id))
        .and_where(Expr::col(TagIden::Id).eq(id))
        .returning(Query::returning().columns([TagIden::Id]));

    update
}

pub fn delete_by_id(user_id: i32, id: i32) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(TagIden::Table)
        .and_where(Expr::col(TagIden::UserId).eq(user_id))
        .and_where(Expr::col(TagIden::Id).eq(id));

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
    select.and_where(Expr::col(TagIden::Name).eq(name));

    select
}

pub fn exists_by_name_and_id_not(user_id: i32, name: &str, id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    add_id_field(&mut select);
    select
        .and_where(Expr::col(TagIden::Name).eq(name))
        .and_where(Expr::col(TagIden::Id).ne(id));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(TagIden::Table)
        .and_where(Expr::col((TagIden::Table, TagIden::UserId)).eq(user_id));
}

fn where_id(select: &mut SelectStatement, id: i32) {
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
