use chrono::NaiveDate;
use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{DLCAvailableIden, DLCIden, PlatformIden, QUERY_DATE_ALIAS};

use super::{dlc_query, platform_query};

pub fn select_all_dlcs_by_platform_id_order_by_added_date(
    user_id: i32,
    platform_id: i32,
) -> impl QueryStatementWriter {
    let mut select = dlc_query::select_all(user_id);

    join_dlc_available_by_platform_id(&mut select, platform_id);
    add_fields(&mut select);
    add_order_by_added_date(&mut select);

    select
}

pub fn select_all_platforms_by_dlc_id_order_by_added_date(
    user_id: i32,
    dlc_id: i32,
) -> impl QueryStatementWriter {
    let mut select = platform_query::select_all(user_id);

    join_dlc_available_by_dlc_id(&mut select, dlc_id);
    add_fields(&mut select);
    add_order_by_added_date(&mut select);

    select
}

pub fn insert(
    user_id: i32,
    dlc_id: i32,
    platform_id: i32,
    added_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(DLCAvailableIden::Table)
        .columns([
            DLCAvailableIden::UserId,
            DLCAvailableIden::DLCId,
            DLCAvailableIden::PlatformId,
            DLCAvailableIden::AddedDate,
        ])
        .values_panic([
            user_id.into(),
            dlc_id.into(),
            platform_id.into(),
            added_date.into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: i32, dlc_id: i32, platform_id: i32) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(DLCAvailableIden::Table)
        .and_where(Expr::col(DLCAvailableIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCAvailableIden::DLCId).eq(dlc_id))
        .and_where(Expr::col(DLCAvailableIden::PlatformId).eq(platform_id));

    delete
}

pub fn exists_by_id(user_id: i32, dlc_id: i32, platform_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((DLCAvailableIden::Table, DLCAvailableIden::UserId))
        .and_where(Expr::col(DLCAvailableIden::DLCId).eq(dlc_id))
        .and_where(Expr::col(DLCAvailableIden::PlatformId).eq(platform_id));

    select
}

fn join_dlc_available_by_platform_id(select: &mut SelectStatement, platform_id: i32) {
    select
        .left_join(
            DLCAvailableIden::Table,
            Expr::col((DLCIden::Table, DLCIden::UserId))
                .equals((DLCAvailableIden::Table, DLCAvailableIden::UserId))
                .and(
                    Expr::col((DLCIden::Table, DLCIden::Id))
                        .equals((DLCAvailableIden::Table, DLCAvailableIden::DLCId)),
                ),
        )
        .and_where(
            Expr::col((DLCAvailableIden::Table, DLCAvailableIden::PlatformId)).eq(platform_id),
        );
}

fn join_dlc_available_by_dlc_id(select: &mut SelectStatement, dlc_id: i32) {
    select
        .left_join(
            DLCAvailableIden::Table,
            Expr::col((PlatformIden::Table, PlatformIden::UserId))
                .equals((DLCAvailableIden::Table, DLCAvailableIden::UserId))
                .and(
                    Expr::col((PlatformIden::Table, PlatformIden::Id))
                        .equals((DLCAvailableIden::Table, DLCAvailableIden::PlatformId)),
                ),
        )
        .and_where(Expr::col((DLCAvailableIden::Table, DLCAvailableIden::DLCId)).eq(dlc_id));
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: i32) {
    select
        .from(DLCAvailableIden::Table)
        .and_where(Expr::col((DLCAvailableIden::Table, DLCAvailableIden::UserId)).eq(user_id));
}

fn add_fields(select: &mut SelectStatement) {
    select.expr_as(
        Expr::col((DLCAvailableIden::Table, DLCAvailableIden::AddedDate)),
        Alias::new(QUERY_DATE_ALIAS),
    );
}

fn add_order_by_added_date(select: &mut SelectStatement) {
    select.order_by(
        (DLCAvailableIden::Table, DLCAvailableIden::AddedDate),
        Order::Asc,
    );
}
