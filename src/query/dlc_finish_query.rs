use chrono::NaiveDate;
use sea_query::{Expr, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{DLCFinishIden, DLCIden};

use super::dlc_query;

pub fn select_all_by_user_id_and_dlc_id(user_id: i32, dlc_id: i32) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id_and_dlc_id(&mut select, user_id, dlc_id);
    add_date_field(&mut select);

    select
}

pub fn select_all_dlcs_order_by_date(user_id: i32) -> SelectStatement {
    let mut select = dlc_query::select_all(user_id);

    join_dlc_finish(&mut select);
    select.order_by((DLCFinishIden::Table, DLCFinishIden::Date), Order::Desc);

    select
}

pub fn select_all_dlcs_by_date_gte_and_date_lte_order_by_date(
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut select = select_all_dlcs_order_by_date(user_id);

    select
        .and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).gte(start_date))
        .and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).lte(end_date));

    select
}

pub fn select_first_by_user_id_and_dlc_id(user_id: i32, dlc_id: i32) -> impl QueryStatementWriter {
    let mut select = select_all_by_user_id_and_dlc_id(user_id, dlc_id);

    select.order_by(DLCFinishIden::Date, Order::Asc);
    select.limit(1);

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

    from_and_where_user_id_and_dlc_id(&mut select, user_id, dlc_id);
    select
        .column((DLCFinishIden::Table, DLCFinishIden::DLCId))
        .and_where(Expr::col(DLCFinishIden::Date).eq(date));

    select
}

fn join_dlc_finish(select: &mut SelectStatement) {
    select.left_join(
        DLCFinishIden::Table,
        Expr::tbl(DLCIden::Table, DLCIden::UserId)
            .equals(DLCFinishIden::Table, DLCFinishIden::UserId)
            .and(
                Expr::tbl(DLCIden::Table, DLCIden::Id)
                    .equals(DLCFinishIden::Table, DLCFinishIden::DLCId),
            ),
    );
}

fn from_and_where_user_id_and_dlc_id(select: &mut SelectStatement, user_id: i32, dlc_id: i32) {
    select
        .from(DLCFinishIden::Table)
        .and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::UserId)).eq(user_id))
        .and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::DLCId)).eq(dlc_id));
}

fn add_date_field(select: &mut SelectStatement) {
    select.column((DLCFinishIden::Table, DLCFinishIden::Date));
}
