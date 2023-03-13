use chrono::NaiveDate;
use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{DLCFinishIden, DLCIden, DLCSearch, SearchQuery, QUERY_DATE_ALIAS};
use crate::errors::SearchErrors;

use super::{dlc_query, search::apply_search};

pub fn select_max_by_user_id_and_dlc_id(user_id: &str, dlc_id: &str) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_dlc_id(&mut select, user_id, dlc_id);
    select.expr(Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).max());

    select
}

pub fn select_all_by_user_id_and_dlc_id(user_id: &str, dlc_id: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id_and_dlc_id(&mut select, user_id, dlc_id);
    add_date_field(&mut select);

    select
}

fn select_all_dlc_with_finish_by_date_gte_and_date_lte(
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> SelectStatement {
    let mut select = dlc_query::select_all_group_by_id(user_id);

    join_dlc_finish(&mut select);

    if let Some(start) = start_date {
        select.and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).gte(start));
    }

    if let Some(end) = end_date {
        select.and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).lte(end));
    }

    select
}

pub fn select_all_first_dlc_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_asc(
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    mut search: DLCSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_dlc_with_finish_by_date_gte_and_date_lte(user_id, start_date, end_date);

    select.expr_as(
        Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).min(),
        Alias::new(QUERY_DATE_ALIAS),
    );
    select.order_by_expr(
        Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).min(),
        Order::Asc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_last_dlc_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    mut search: DLCSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_dlc_with_finish_by_date_gte_and_date_lte(user_id, start_date, end_date);

    select.expr_as(
        Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).max(),
        Alias::new(QUERY_DATE_ALIAS),
    );
    select.order_by_expr(
        Expr::col((DLCFinishIden::Table, DLCFinishIden::Date)).max(),
        Order::Desc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn insert(user_id: &str, dlc_id: &str, date: NaiveDate) -> impl QueryStatementWriter {
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

pub fn delete_by_id(user_id: &str, dlc_id: &str, date: NaiveDate) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(DLCFinishIden::Table)
        .and_where(Expr::col(DLCFinishIden::UserId).eq(user_id))
        .and_where(Expr::col(DLCFinishIden::DLCId).eq(dlc_id))
        .and_where(Expr::col(DLCFinishIden::Date).eq(date));

    delete
}

pub fn exists_by_id(user_id: &str, dlc_id: &str, date: NaiveDate) -> impl QueryStatementWriter {
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
        Expr::col((DLCIden::Table, DLCIden::UserId))
            .equals((DLCFinishIden::Table, DLCFinishIden::UserId))
            .and(
                Expr::col((DLCIden::Table, DLCIden::Id))
                    .equals((DLCFinishIden::Table, DLCFinishIden::DLCId)),
            ),
    );
}

fn from_and_where_user_id_and_dlc_id(select: &mut SelectStatement, user_id: &str, dlc_id: &str) {
    select
        .from(DLCFinishIden::Table)
        .and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::UserId)).eq(user_id))
        .and_where(Expr::col((DLCFinishIden::Table, DLCFinishIden::DLCId)).eq(dlc_id));
}

fn add_date_field(select: &mut SelectStatement) {
    select.column((DLCFinishIden::Table, DLCFinishIden::Date));
}
