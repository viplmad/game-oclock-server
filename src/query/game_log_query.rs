use chrono::NaiveDateTime;
use sea_query::{
    Alias, Expr, Func, FunctionCall, Order, Query, QueryStatementWriter, SelectStatement,
};

use crate::entities::{
    GameIden, GameLog, GameLogIden, GameSearch, SearchQuery, LOG_DATETIME_ALIAS, LOG_TIME_ALIAS,
};
use crate::errors::SearchErrors;

use super::game_query;
use super::search::apply_search;

pub fn select_sum_time_by_user_id_and_game_id(
    user_id: i32,
    game_id: i32,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select.expr(coalesce_time_sum());

    select
}

pub fn select_all_by_user_id_and_game_id(user_id: i32, game_id: i32) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    add_datetime_and_time_fields(&mut select);

    select
}

fn select_all_game_with_log_by_datetime_gte_and_datetime_lte(
    user_id: i32,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
) -> SelectStatement {
    let mut select = game_query::select_all_group_by_id(user_id);

    join_game_log(&mut select);

    if let Some(start) = start_datetime {
        select.and_where(Expr::col((GameLogIden::Table, GameLogIden::DateTime)).gte(start));
    }

    if let Some(end) = end_datetime {
        select.and_where(Expr::col((GameLogIden::Table, GameLogIden::DateTime)).lte(end));
    }

    select
}

pub fn select_all_first_game_with_log_with_search_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
    user_id: i32,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_game_with_log_by_datetime_gte_and_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    select
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::DateTime)).min(),
            Alias::new(LOG_DATETIME_ALIAS),
        )
        .expr_as(coalesce_time_sum(), Alias::new(LOG_TIME_ALIAS));
    select.order_by_expr(
        Expr::col((GameLogIden::Table, GameLogIden::DateTime)).min(),
        Order::Asc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_last_game_with_log_with_search_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
    user_id: i32,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_game_with_log_by_datetime_gte_and_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    select
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::DateTime)).max(),
            Alias::new(LOG_DATETIME_ALIAS),
        )
        .expr_as(coalesce_time_sum(), Alias::new(LOG_TIME_ALIAS));
    select.order_by_expr(
        Expr::col((GameLogIden::Table, GameLogIden::DateTime)).max(),
        Order::Desc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_games_order_by_datetime_desc(user_id: i32) -> SelectStatement {
    let mut select = game_query::select_all(user_id);

    join_game_log(&mut select);
    select.order_by((GameLogIden::Table, GameLogIden::DateTime), Order::Desc);

    select
}

pub fn select_all_games_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
    user_id: i32,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> SelectStatement {
    let mut select = select_all_games_order_by_datetime_desc(user_id);

    select
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::DateTime)).gte(start_datetime))
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::DateTime)).lte(end_datetime));

    select
}

pub fn select_all_games_log_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
    user_id: i32,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut select = select_all_games_by_datetime_gte_and_datetime_lte_order_by_datetime_desc(
        user_id,
        start_datetime,
        end_datetime,
    );

    select
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::DateTime)),
            Alias::new(LOG_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::Time)),
            Alias::new(LOG_TIME_ALIAS),
        );

    select
}

pub fn insert(user_id: i32, game_id: i32, log: &GameLog) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    let secs = log.time.microseconds / 1_000_000; // TODO duplicate variable in DurationDef
    insert
        .into_table(GameLogIden::Table)
        .columns([
            GameLogIden::UserId,
            GameLogIden::GameId,
            GameLogIden::DateTime,
            GameLogIden::Time,
        ])
        .values_panic([
            user_id.into(),
            game_id.into(),
            log.datetime.into(),
            format!("{secs} seconds").into(), // TODO
        ]);

    insert
}

pub fn delete_by_id(
    user_id: i32,
    game_id: i32,
    datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameLogIden::Table)
        .and_where(Expr::col(GameLogIden::UserId).eq(user_id))
        .and_where(Expr::col(GameLogIden::GameId).eq(game_id))
        .and_where(Expr::col(GameLogIden::DateTime).eq(datetime));

    delete
}

pub fn exists_by_id(
    user_id: i32,
    game_id: i32,
    datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameLogIden::Table, GameLogIden::GameId))
        .and_where(Expr::col(GameLogIden::DateTime).eq(datetime));

    select
}

fn join_game_log(select: &mut SelectStatement) {
    select.left_join(
        GameLogIden::Table,
        Expr::col((GameIden::Table, GameIden::UserId))
            .equals((GameLogIden::Table, GameLogIden::UserId))
            .and(
                Expr::col((GameIden::Table, GameIden::Id))
                    .equals((GameLogIden::Table, GameLogIden::GameId)),
            ),
    );
}

fn from_and_where_user_id_and_game_id(select: &mut SelectStatement, user_id: i32, game_id: i32) {
    select
        .from(GameLogIden::Table)
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::UserId)).eq(user_id))
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::GameId)).eq(game_id));
}

fn add_datetime_and_time_fields(select: &mut SelectStatement) {
    select
        .column((GameLogIden::Table, GameLogIden::DateTime))
        .column((GameLogIden::Table, GameLogIden::Time));
}

fn coalesce_time_sum() -> FunctionCall {
    Func::coalesce([
        Expr::col((GameLogIden::Table, GameLogIden::Time)).sum(),
        Expr::val("0 seconds").into(), // TODO
    ])
}
