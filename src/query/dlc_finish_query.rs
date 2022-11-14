use chrono::NaiveDate;
use sea_query::{Expr, Query, QueryStatementWriter, SelectStatement};

use crate::entities::DLCFinishIden;

pub fn select_all_by_user_id_and_dlc_id(user_id: i32, dlc_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((DLCFinishIden::Table, DLCFinishIden::Date))
        .and_where(Expr::col(DLCFinishIden::DLCId).eq(dlc_id));

    select
}

pub fn insert(user_id: i32, dlc_id: i32, date: NaiveDate) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(DLCFinishIden::Table)
        .columns([
            DLCFinishIden::UserId,
            DLCFinishIden::DLCId,
            DLCFinishIden::Date,
        ])
        .values_panic([user_id.into(), dlc_id.into(), date.into()]);

    insert
}

pub fn delete_by_id(user_id: i32, dlc_id: i32, date: NaiveDate) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(DLCFinishIden::Table)
        .and_where(Expr::col(DLCFinishIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCFinishIden::DLCId).eq(dlc_id))
        .and_where(Expr::col(DLCFinishIden::Date).eq(date));

    delete
}

pub fn exists_by_id(user_id: i32, dlc_id: i32, date: NaiveDate) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((DLCFinishIden::Table, DLCFinishIden::DLCId))
        .and_where(Expr::col(DLCFinishIden::DLCId).eq(dlc_id))
        .and_where(Expr::col(DLCFinishIden::Date).eq(date));

    select
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(DLCFinishIden::Table)
        .and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::UserId)).eq(user_id));
}
