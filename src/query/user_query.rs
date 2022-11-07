use chrono::Utc;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{User, UserIden};

pub fn select_by_id(id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    where_id(&mut select, id);
    add_fields(&mut select);

    select
}

pub fn select_by_username(username: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    add_fields(&mut select);
    select.and_where(Expr::col(UserIden::Username).eq(username));

    select
}

pub fn insert(user: &User, password: &str) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(UserIden::Table)
        .columns([
            UserIden::Username,
            UserIden::Password,
            UserIden::AddedDateTime,
            UserIden::UpdatedDateTime,
        ])
        .values_panic([
            user.username.clone().into(),
            password.into(),
            Utc::now().naive_utc().into(),
            Utc::now().naive_utc().into(),
        ])
        .returning(Query::returning().columns([UserIden::Id]));

    insert
}

pub fn update_password_by_id(id: i32, password: &str) -> impl QueryStatementWriter {
    let mut update = Query::update();

    update
        .table(UserIden::Table)
        .values(vec![
            (UserIden::Password, password.into()),
            (UserIden::UpdatedDateTime, Utc::now().naive_utc().into()),
        ])
        .and_where(Expr::col(UserIden::Id).eq(id))
        .returning(Query::returning().columns([UserIden::Id]));

    update
}

pub fn exists_by_username(username: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from(&mut select);
    add_id_field(&mut select);
    select.and_where(Expr::col(UserIden::Username).eq(username));

    select
}

fn from(select: &mut SelectStatement) {
    select.from(UserIden::Table);
}

fn where_id(select: &mut SelectStatement, id: i32) {
    select.and_where(Expr::col((UserIden::Table, UserIden::Id)).eq(id));
}

fn add_id_field(select: &mut SelectStatement) {
    select.column((UserIden::Table, UserIden::Id));
}

fn add_fields(select: &mut SelectStatement) {
    add_id_field(select);
    select
        .column((UserIden::Table, UserIden::Username))
        .column((UserIden::Table, UserIden::Password))
        .column((UserIden::Table, UserIden::AddedDateTime))
        .column((UserIden::Table, UserIden::UpdatedDateTime));
}
