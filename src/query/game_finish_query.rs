use chrono::NaiveDate;
use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};

use crate::entities::{GameFinishIden, GameIden, GameSearch, SearchQuery, QUERY_DATE_ALIAS};
use crate::errors::SearchErrors;

use super::game_query;
use super::search::apply_search;

pub fn select_min_date_by_user_id_and_game_id(
    user_id: &str,
    game_id: &str,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select.expr(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min());

    select
}

pub fn select_all_by_user_id_and_game_id(user_id: &str, game_id: &str) -> SelectStatement {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    add_date_field(&mut select);

    select
}

pub fn select_all_first_by_user_id_and_game_id_in(
    user_id: &str,
    game_ids: Vec<String>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select.and_where(Expr::col((GameFinishIden::Table, GameFinishIden::GameId)).is_in(game_ids));
    select.column((GameFinishIden::Table, GameFinishIden::GameId));
    select.expr_as(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min(),
        GameFinishIden::Date,
    );
    select.order_by_expr(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min(),
        Order::Asc,
    );
    select
        .group_by_col((GameFinishIden::Table, GameFinishIden::UserId))
        .group_by_col((GameFinishIden::Table, GameFinishIden::GameId));

    select
}

fn select_all_game_with_finish_by_date_gte_and_date_lte(
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> SelectStatement {
    let mut select = game_query::select_all_group_by_id(user_id);

    join_game_finish(&mut select);
    where_optional_date_gte_and_date_lte(&mut select, start_date, end_date);

    select
}

pub fn select_all_first_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_asc(
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_game_with_finish_by_date_gte_and_date_lte(user_id, start_date, end_date);

    select.expr_as(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min(),
        Alias::new(QUERY_DATE_ALIAS),
    );
    select.order_by_expr(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min(),
        Order::Asc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_last_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &str,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_game_with_finish_by_date_gte_and_date_lte(user_id, start_date, end_date);

    select.expr_as(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)).max(),
        Alias::new(QUERY_DATE_ALIAS),
    );
    select.order_by_expr(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)).max(),
        Order::Desc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_games_order_by_date_desc(user_id: &str) -> SelectStatement {
    let mut select = game_query::select_all(user_id);

    join_game_finish(&mut select);
    order_by_date_desc(&mut select);

    select
}

pub fn select_all_games_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> SelectStatement {
    let mut select = select_all_games_order_by_date_desc(user_id);

    where_date_gte_and_date_lte(&mut select, start_date, end_date);

    select
}

pub fn select_all_games_finish_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut select =
        select_all_games_by_date_gte_and_date_lte_order_by_date_desc(user_id, start_date, end_date);

    select.expr_as(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)),
        Alias::new(QUERY_DATE_ALIAS),
    );

    select
}

pub fn insert(user_id: &str, game_id: &str, date: NaiveDate) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameFinishIden::Table)
        .columns([
            GameFinishIden::UserId,
            GameFinishIden::GameId,
            GameFinishIden::Date,
        ])
        .values_panic([user_id.into(), game_id.into(), date.into()]);

    insert
}

pub fn delete_by_id(user_id: &str, game_id: &str, date: NaiveDate) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameFinishIden::Table)
        .and_where(Expr::col(GameFinishIden::UserId).eq(user_id))
        .and_where(Expr::col(GameFinishIden::GameId).eq(game_id))
        .and_where(Expr::col(GameFinishIden::Date).eq(date));

    delete
}

pub fn exists_by_id(user_id: &str, game_id: &str, date: NaiveDate) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameFinishIden::Table, GameFinishIden::GameId))
        .and_where(Expr::col(GameFinishIden::Date).eq(date));

    select
}

fn join_game_finish(select: &mut SelectStatement) {
    select.left_join(
        GameFinishIden::Table,
        Expr::col((GameIden::Table, GameIden::UserId))
            .equals((GameFinishIden::Table, GameFinishIden::UserId))
            .and(
                Expr::col((GameIden::Table, GameIden::Id))
                    .equals((GameFinishIden::Table, GameFinishIden::GameId)),
            ),
    );
}

fn from_and_where_user_id_and_game_id(select: &mut SelectStatement, user_id: &str, game_id: &str) {
    from_and_where_user_id(select, user_id);
    select.and_where(Expr::col((GameFinishIden::Table, GameFinishIden::GameId)).eq(game_id));
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &str) {
    select
        .from(GameFinishIden::Table)
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::UserId)).eq(user_id));
}

fn where_optional_date_gte_and_date_lte(
    select: &mut SelectStatement,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) {
    if let Some(start) = start_date {
        select.and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).gte(start));
    }

    if let Some(end) = end_date {
        select.and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).lte(end));
    }
}

fn where_date_gte_and_date_lte(
    select: &mut SelectStatement,
    start_date: NaiveDate,
    end_date: NaiveDate,
) {
    select
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).gte(start_date))
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).lte(end_date));
}

fn order_by_date_desc(select: &mut SelectStatement) {
    select.order_by((GameFinishIden::Table, GameFinishIden::Date), Order::Desc);
}

fn add_date_field(select: &mut SelectStatement) {
    select.column((GameFinishIden::Table, GameFinishIden::Date));
}
