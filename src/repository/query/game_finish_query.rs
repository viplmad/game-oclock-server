use chrono::{DateTime, Utc};
use sea_query::{Alias, Expr, Order, Query, QueryStatementWriter, SelectStatement};
use uuid::Uuid;

use crate::entities::{
    FINISH_DATETIME_ALIAS, FINISH_DEVICE_ID_ALIAS, FINISH_STATUS_ALIAS, GameFinish, GameFinishIden,
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
    select.expr(Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).min());

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
            Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).min(),
            GameFinishIden::Datetime,
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
        Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).min(),
        Order::Asc,
    );
    select
        .group_by_col((GameFinishIden::Table, GameFinishIden::UserId))
        .group_by_col((GameFinishIden::Table, GameFinishIden::GameId));

    select
}

fn select_all_game_with_finish_by_date_gte_and_date_lte(
    user_id: &Uuid,
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
) -> SelectStatement {
    let mut select = game_query::select_all_group_by_id(user_id);

    join_game_finish(&mut select);
    where_optional_date_gte_and_date_lte(&mut select, start_datetime, end_datetime);

    select
}

pub fn select_all_first_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_asc(
    user_id: &Uuid,
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_game_with_finish_by_date_gte_and_date_lte(user_id, start_datetime, end_datetime);

    select
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).min(),
            Alias::new(FINISH_DATETIME_ALIAS),
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
        Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).min(),
        Order::Asc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_last_game_with_finish_with_search_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &Uuid,
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
    mut search: GameSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select =
        select_all_game_with_finish_by_date_gte_and_date_lte(user_id, start_datetime, end_datetime);

    select
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).max(),
            Alias::new(FINISH_DATETIME_ALIAS),
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
        Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).max(),
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
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> SelectStatement {
    let mut select = select_all_games_order_by_date_desc(user_id);

    where_date_gte_and_date_lte(&mut select, start_datetime, end_datetime);

    select
}

pub fn select_all_games_finish_by_date_gte_and_date_lte_order_by_date_desc(
    user_id: &Uuid,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut select = select_all_games_by_date_gte_and_date_lte_order_by_date_desc(
        user_id,
        start_datetime,
        end_datetime,
    );

    select
        .expr_as(
            Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)),
            Alias::new(FINISH_DATETIME_ALIAS),
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
            GameFinishIden::Datetime,
            GameFinishIden::Status,
            GameFinishIden::DeviceId,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&game_finish.user_id).into(),
            crate::uuid_utils::to_string(&game_finish.game_id).into(),
            game_finish.datetime.into(),
            game_finish.status.into(),
            game_finish
                .device_id
                .map(|id| crate::uuid_utils::to_string(&id))
                .into(),
        ]);

    insert
}

pub fn delete_by_id(
    user_id: &Uuid,
    game_id: &Uuid,
    datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(GameFinishIden::Table)
        .and_where(Expr::col(GameFinishIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(GameFinishIden::GameId).eq(crate::uuid_utils::to_string(game_id)))
        .and_where(Expr::col(GameFinishIden::Datetime).eq(datetime));

    delete
}

pub fn exists_by_id(
    user_id: &Uuid,
    game_id: &Uuid,
    datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_game_id(&mut select, user_id, game_id);
    select
        .column((GameFinishIden::Table, GameFinishIden::GameId))
        .and_where(Expr::col(GameFinishIden::Datetime).eq(datetime));

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
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
) {
    if let Some(start) = start_datetime {
        select.and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).gte(start));
    }

    if let Some(end) = end_datetime {
        select.and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).lte(end));
    }
}

fn where_date_gte_and_date_lte(
    select: &mut SelectStatement,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) {
    select
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).gte(start_datetime))
        .and_where(Expr::col((GameFinishIden::Table, GameFinishIden::Datetime)).lte(end_datetime));
}

fn order_by_date_desc(select: &mut SelectStatement) {
    select.order_by(
        (GameFinishIden::Table, GameFinishIden::Datetime),
        Order::Desc,
    );
}

fn add_fields(select: &mut SelectStatement) {
    select
        .column((GameFinishIden::Table, GameFinishIden::UserId))
        .column((GameFinishIden::Table, GameFinishIden::GameId))
        .column((GameFinishIden::Table, GameFinishIden::Datetime))
        .column((GameFinishIden::Table, GameFinishIden::Status))
        .column((GameFinishIden::Table, GameFinishIden::DeviceId));
}
