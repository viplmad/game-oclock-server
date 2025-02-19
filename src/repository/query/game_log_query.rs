use chrono::NaiveDateTime;
use sea_query::{
    Alias, Expr, Func, FunctionCall, Order, Query, QueryStatementWriter, SelectStatement,
    SimpleExpr,
};
use uuid::Uuid;

use crate::entities::{
    GameIden, GameLog, GameLogIden, GameSearch, SearchQuery, LOG_DEVICE_ID_ALIAS,
    LOG_END_DATETIME_ALIAS, LOG_START_DATETIME_ALIAS, LOG_TIME_ALIAS, QUERY_TIME_ALIAS,
};
use crate::errors::SearchErrors;

use super::game_query;
use super::search::apply_search;

pub fn select_sum_time_by_user_id_and_game_id(
    user_id: &Uuid,
    game_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select.expr(coalesce_time_sum());

    select
}

pub fn select_all_by_user_id_and_game_id(
    user_id: &Uuid,
    game_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    add_fields(&mut select);

    select
}

pub fn select_all_first_by_user_id_and_game_id_in(
    user_id: &Uuid,
    game_ids: Vec<Uuid>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select.and_where(
        Expr::col((GameLogIden::Table, GameLogIden::GameId)).is_in(
            game_ids
                .into_iter()
                .map(|id| crate::uuid_utils::to_string(&id)),
        ),
    );
    select
        .column((GameLogIden::Table, GameLogIden::UserId))
        .column((GameLogIden::Table, GameLogIden::GameId))
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).min(),
            GameLogIden::StartDatetime,
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::EndDatetime)).max(),
            GameLogIden::EndDatetime,
        )
        .expr_as(coalesce_time_sum(), Alias::new(QUERY_TIME_ALIAS));
    select.order_by_expr(
        Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).min(),
        Order::Asc,
    );
    select
        .group_by_col((GameLogIden::Table, GameLogIden::UserId))
        .group_by_col((GameLogIden::Table, GameLogIden::GameId));

    select
}

fn select_all_game_with_log_by_start_datetime_gte_and_start_datetime_lte(
    user_id: &Uuid,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
) -> SelectStatement {
    let mut select = game_query::select_all_group_by_id(user_id);

    join_game_log(&mut select);
    where_optional_start_datetime_gte_and_start_datetime_lte(
        &mut select,
        start_datetime,
        end_datetime,
    );

    select
}

pub fn select_all_first_game_with_log_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_game_with_log_by_start_datetime_gte_and_start_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    select
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).min(),
            Alias::new(LOG_START_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::EndDatetime)).max(),
            Alias::new(LOG_END_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::DeviceId)).max(), // TODO
            Alias::new(LOG_DEVICE_ID_ALIAS),
        )
        .expr_as(coalesce_time_sum(), Alias::new(LOG_TIME_ALIAS));
    select.order_by_expr(
        Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).min(),
        Order::Asc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_last_game_with_log_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_game_with_log_by_start_datetime_gte_and_start_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    select
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).min(),
            Alias::new(LOG_START_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::EndDatetime)).max(),
            Alias::new(LOG_END_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::DeviceId)).max(), // TODO
            Alias::new(LOG_DEVICE_ID_ALIAS),
        )
        .expr_as(coalesce_time_sum(), Alias::new(LOG_TIME_ALIAS));
    select.order_by_expr(
        Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).max(),
        Order::Desc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_games_order_by_start_datetime_desc(user_id: &Uuid) -> SelectStatement {
    let mut select = game_query::select_all(user_id);

    join_game_log(&mut select);
    order_by_start_datetime_desc(&mut select);

    select
}

pub fn select_all_games_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> SelectStatement {
    let mut select = select_all_games_order_by_start_datetime_desc(user_id);

    where_start_datetime_gte_and_start_datetime_lte(&mut select, start_datetime, end_datetime);

    select
}

pub fn select_all_games_log_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut select =
        select_all_games_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
            user_id,
            start_datetime,
            end_datetime,
        );

    select
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)),
            Alias::new(LOG_START_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::EndDatetime)),
            Alias::new(LOG_END_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((GameLogIden::Table, GameLogIden::DeviceId)),
            Alias::new(LOG_DEVICE_ID_ALIAS),
        )
        .expr_as(derived_time_expr(), Alias::new(LOG_TIME_ALIAS));

    select
}

pub fn insert(game_log: &GameLog) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameLogIden::Table)
        .columns([
            GameLogIden::UserId,
            GameLogIden::GameId,
            GameLogIden::StartDatetime,
            GameLogIden::EndDatetime,
            GameLogIden::DeviceId,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game_log.user_id).into(),
            crate::uuid_utils::to_string(&game_log.game_id).into(),
            game_log.start_datetime.into(),
            game_log.end_datetime.into(),
            game_log
                .device_id
                .map(|id| crate::uuid_utils::to_string(&id))
                .into(),
        ]);

    insert
}

pub fn delete_by_id(
    user_id: &Uuid,
    game_id: &Uuid,
    start_datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameLogIden::Table)
        .and_where(Expr::col(GameLogIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameLogIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameLogIden::StartDatetime).eq(start_datetime));

    delete
}

pub fn exists_by_id(
    user_id: &Uuid,
    game_id: &Uuid,
    start_datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameLogIden::Table, GameLogIden::GameId))
        .and_where(Expr::col(GameLogIden::StartDatetime).eq(start_datetime));

    select
}

pub fn exists_by_start_datetime_lt_or_end_datetime_gt(
    user_id: &Uuid,
    end_datetime: NaiveDateTime,
    start_datetime: NaiveDateTime,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((GameLogIden::Table, GameLogIden::GameId))
        .and_where(Expr::col(GameLogIden::StartDatetime).lt(end_datetime))
        .and_where(Expr::col(GameLogIden::EndDatetime).gt(start_datetime));

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

fn from_and_where_user_id_and_game_id(
    select: &mut SelectStatement,
    user_id: &Uuid,
    game_id: &Uuid,
) {
    from_and_where_user_id(select, user_id);
    select.and_where(
        Expr::col((GameLogIden::Table, GameLogIden::GameId))
            .eq(crate::uuid_utils::to_string(game_id)),
    );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GameLogIden::Table).and_where(
        Expr::col((GameLogIden::Table, GameLogIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_optional_start_datetime_gte_and_start_datetime_lte(
    select: &mut SelectStatement,
    start_datetime: Option<NaiveDateTime>,
    end_datetime: Option<NaiveDateTime>,
) {
    if let Some(start) = start_datetime {
        select.and_where(Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).gte(start));
    }

    if let Some(end) = end_datetime {
        select.and_where(Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).lte(end));
    }
}

fn where_start_datetime_gte_and_start_datetime_lte(
    select: &mut SelectStatement,
    start_datetime: NaiveDateTime,
    end_datetime: NaiveDateTime,
) {
    select
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).gte(start_datetime))
        .and_where(Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)).lte(end_datetime));
}

fn order_by_start_datetime_desc(select: &mut SelectStatement) {
    select.order_by(
        (GameLogIden::Table, GameLogIden::StartDatetime),
        Order::Desc,
    );
}

fn add_fields(select: &mut SelectStatement) {
    select
        .column((GameLogIden::Table, GameLogIden::UserId))
        .column((GameLogIden::Table, GameLogIden::GameId))
        .column((GameLogIden::Table, GameLogIden::StartDatetime))
        .column((GameLogIden::Table, GameLogIden::EndDatetime))
        .column((GameLogIden::Table, GameLogIden::DeviceId))
        .expr_as(derived_time_expr(), Alias::new(QUERY_TIME_ALIAS));
}

fn coalesce_time_sum() -> FunctionCall {
    Func::coalesce([
        Expr::expr(derived_time_expr()).sum(),
        Expr::val("0 seconds").into(),
    ])
}

fn derived_time_expr() -> SimpleExpr {
    Expr::col((GameLogIden::Table, GameLogIden::EndDatetime))
        .sub(Expr::col((GameLogIden::Table, GameLogIden::StartDatetime)))
}
