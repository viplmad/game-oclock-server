use chrono::NaiveDate;
use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{
    FINISH_DATE_ALIAS, FINISH_DEVICE_ID_ALIAS, FINISH_STATUS_ALIAS, GameFinish, GameFinishIden,
    GameIden, GameSearch, SearchQuery,
};
use crate::errors::SearchErrors;

use super::game_query;
use super::search::apply_search;

pub fn select_min_date_by_user_id_and_game_id(
    user_id: &Uuid,
    game_id: &Uuid,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select.expr(Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min());

    select
}

pub fn select_all_by_user_id_and_game_id(user_id: &Uuid, game_id: &Uuid) -> SelectStatement {
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
        Expr::col((GameFinishIden::Table, GameFinishIden::GameId)).is_in(
            game_ids
                .into_iter()
                .map(|id| crate::uuid_utils::to_string(&id)),
        ),
    );
    select
        .column((GameFinishIden::Table, GameFinishIden::UserId))
        .column((GameFinishIden::Table, GameFinishIden::GameId))
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min(),
            GameFinishIden::Date,
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Status)).min(), // TODO
            GameFinishIden::Status,
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::DeviceId)).min(), // TODO
            GameFinishIden::DeviceId,
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
    user_id: &Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> SelectStatement {
    let mut select = game_query::select_all_group_by_id(user_id);

    join_game_finish(&mut select);
    where_optional_date_gte_and_date_lte(&mut select, start_date, end_date);

    select
}

pub fn select_all_first_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_asc(
    user_id: &Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_game_with_finish_by_date_gte_and_date_lte(user_id, start_date, end_date);

    select
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Date)).min(),
            Alias::new(FINISH_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Status)).min(), // TODO
            Alias::new(FINISH_STATUS_ALIAS),
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::DeviceId)).min(), // TODO
            Alias::new(FINISH_DEVICE_ID_ALIAS),
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
    user_id: &Uuid,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_game_with_finish_by_date_gte_and_date_lte(user_id, start_date, end_date);

    select
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Date)).max(),
            Alias::new(FINISH_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Status)).max(), // TODO
            Alias::new(FINISH_STATUS_ALIAS),
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::DeviceId)).max(), // TODO
            Alias::new(FINISH_DEVICE_ID_ALIAS),
        );
    select.order_by_expr(
        Expr::col((GameFinishIden::Table, GameFinishIden::Date)).max(),
        Order::Desc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_games_order_by_date_desc(user_id: &Uuid) -> SelectStatement {
    let mut select = game_query::select_all(user_id);

    join_game_finish(&mut select);
    order_by_date_desc(&mut select);

    select
}

pub fn select_all_games_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> SelectStatement {
    let mut select = select_all_games_order_by_date_desc(user_id);

    where_date_gte_and_date_lte(&mut select, start_date, end_date);

    select
}

pub fn select_all_games_finish_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> impl QueryStatementWriter {
    let mut select =
        select_all_games_by_date_gte_and_date_lte_order_by_date_desc(user_id, start_date, end_date);

    select
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Date)),
            Alias::new(FINISH_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Status)),
            Alias::new(FINISH_STATUS_ALIAS),
        )
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::DeviceId)),
            Alias::new(FINISH_DEVICE_ID_ALIAS),
        );

    select
}

pub fn insert(game_finish: &GameFinish) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(GameFinishIden::Table)
        .columns([
            GameFinishIden::UserId,
            GameFinishIden::GameId,
            GameFinishIden::Date,
            GameFinishIden::Status,
            GameFinishIden::DeviceId,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game_finish.user_id).into(),
            crate::uuid_utils::to_string(&game_finish.game_id).into(),
            game_finish.date.into(),
            game_finish.status.into(),
            game_finish
                .device_id
                .map(|id| crate::uuid_utils::to_string(&id))
                .into(),
        ]);

    insert
}

pub fn delete_by_id(user_id: &Uuid, game_id: &Uuid, date: NaiveDate) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameFinishIden::Table)
        .and_where(Expr::col(GameFinishIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameFinishIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameFinishIden::Date).eq(date));

    delete
}

pub fn exists_by_id(user_id: &Uuid, game_id: &Uuid, date: NaiveDate) -> impl QueryStatementWriter {
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

fn from_and_where_user_id_and_game_id(
    select: &mut SelectStatement,
    user_id: &Uuid,
    game_id: &Uuid,
) {
    from_and_where_user_id(select, user_id);
    select.and_where(
        Expr::col((GameFinishIden::Table, GameFinishIden::GameId))
            .eq(crate::uuid_utils::to_string(game_id)),
    );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(GameFinishIden::Table).and_where(
        Expr::col((GameFinishIden::Table, GameFinishIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
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

fn add_fields(select: &mut SelectStatement) {
    select
        .column((GameFinishIden::Table, GameFinishIden::UserId))
        .column((GameFinishIden::Table, GameFinishIden::GameId))
        .column((GameFinishIden::Table, GameFinishIden::Date))
        .column((GameFinishIden::Table, GameFinishIden::Status))
        .column((GameFinishIden::Table, GameFinishIden::DeviceId));
}
