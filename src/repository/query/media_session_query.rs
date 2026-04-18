use chrono::{DateTime, Utc};
use sea_query::{
    Alias, Expr, Func, FunctionCall, Order, Query, QueryStatementWriter, SelectStatement,
    SimpleExpr,
};
use uuid::Uuid;

use crate::entities::{
    AggregateGroupQuery, AggregateQuery, MediaIden, MediaListSearch, MediaSession,
    MediaSessionIden, QUERY_TIME_ALIAS, SESSION_ADDED_DATETIME_ALIAS, SESSION_DEVICE_ID_ALIAS,
    SESSION_END_DATE_ALIAS, SESSION_FINISHED_STATUS_ALIAS, SESSION_GROUP_ID_ALIAS,
    SESSION_START_DATE_ALIAS, SESSION_STARTED_ALIAS, SESSION_UPDATED_DATETIME_ALIAS, SearchQuery,
    SessionAggregateGroupSearch, SessionAggregateSearch, SessionListSearch,
};
use crate::errors::SearchErrors;

use super::media_query;
use super::search::{apply_aggregate_group_search, apply_aggregate_search, apply_search};

#[cfg(test)]
mod tests {
    use sea_query::PostgresQueryBuilder;
    use uuid::Uuid;

    use super::*;
    use crate::entities::{
        AggregateCountMetric, AggregateDateHistogramGroup, AggregateGroup, AggregateMetric,
        AggregateSumMetric, ColIden, ExprIden, FieldIden, FieldType, GroupDateHistogramInterval,
    };

    #[test]
    fn count_all_sessions() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_all_by_user_id(
            &user_id,
            SessionAggregateSearch {
                filter: None,
                aggr: AggregateMetric::Count(AggregateCountMetric::new(
                    FieldIden::Col(ColIden::new(MediaSessionIden::MediaId, FieldType::String)),
                    None,
                    false,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            r#"SELECT COUNT("MediaSession"."media_id") FROM "MediaSession" WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000'"#
        );
    }

    #[test]
    fn sum_time_all_sessions() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_all_by_user_id(
            &user_id,
            SessionAggregateSearch {
                filter: None,
                aggr: AggregateMetric::Sum(AggregateSumMetric::new(
                    FieldIden::Expr(ExprIden::new::<MediaSessionIden>(
                        Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).sub(
                            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)),
                        ),
                        FieldType::Integer,
                    )),
                    None,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            r#"SELECT SUM("MediaSession"."end_date" - "MediaSession"."start_date") FROM "MediaSession" WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000'"#
        );
    }

    #[test]
    fn group_medias_by_month_played() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = aggregate_group_by_user_id(
            &user_id,
            SessionAggregateGroupSearch {
                filter: None,
                aggr: AggregateMetric::Count(AggregateCountMetric::new(
                    FieldIden::Col(ColIden::new(MediaSessionIden::MediaId, FieldType::String)),
                    None,
                    true,
                )),
                group: AggregateGroup::DateHistogram(AggregateDateHistogramGroup::new(
                    FieldIden::Col(ColIden::new(
                        MediaSessionIden::StartDate,
                        FieldType::DateTime,
                    )),
                    None,
                    GroupDateHistogramInterval::Month,
                )),
            },
        );
        assert_eq!(
            query.unwrap().query.to_string(PostgresQueryBuilder),
            r#"SELECT DATE_PART('month', "MediaSession"."start_date"), COUNT(DISTINCT "MediaSession"."media_id") FROM "MediaSession" WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000' GROUP BY DATE_PART('month', "MediaSession"."start_date")"#
        );
    }
}

pub fn select_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    start_datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);
    select.and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).eq(start_datetime),
    );
    add_fields(&mut select);

    select
}

pub fn select_all_by_user_id_and_media_id(
    user_id: &Uuid,
    media_id: &Uuid,
    search: SessionListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);
    add_fields(&mut select);

    apply_search(select, search)
}

pub fn aggregate_all_by_user_id_and_media_id(
    user_id: &Uuid,
    media_id: &Uuid,
    search: SessionAggregateSearch,
) -> Result<AggregateQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);

    apply_aggregate_search(select, search)
}

pub fn aggregate_all_by_user_id(
    user_id: &Uuid,
    search: SessionAggregateSearch,
) -> Result<AggregateQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);

    apply_aggregate_search(select, search)
}

pub fn aggregate_group_by_user_id(
    user_id: &Uuid,
    search: SessionAggregateGroupSearch,
) -> Result<AggregateGroupQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);

    apply_aggregate_group_search(select, search)
}

pub fn select_all_first_by_user_id_and_media_id_in(
    user_id: &Uuid,
    media_ids: Vec<Uuid>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select.and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId)).is_in(
            media_ids
                .into_iter()
                .map(|id| crate::uuid_utils::to_string(&id)),
        ),
    );
    select
        .column((MediaSessionIden::Table, MediaSessionIden::UserId))
        .column((MediaSessionIden::Table, MediaSessionIden::MediaId))
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).min(),
            MediaSessionIden::StartDate,
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).max(),
            MediaSessionIden::EndDate,
        )
        .expr_as(coalesce_time_sum(), Alias::new(QUERY_TIME_ALIAS));
    select.order_by_expr(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).min(),
        Order::Asc,
    );
    select
        .group_by_col((MediaSessionIden::Table, MediaSessionIden::UserId))
        .group_by_col((MediaSessionIden::Table, MediaSessionIden::MediaId));

    select
}

fn select_all_media_with_session_by_start_datetime_gte_and_start_datetime_lte(
    user_id: &Uuid,
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
) -> SelectStatement {
    let mut select = media_query::select_all(user_id);

    join_media_session(&mut select);
    where_optional_start_datetime_gte_and_start_datetime_lte(
        &mut select,
        start_datetime,
        end_datetime,
    );

    select
}

pub fn select_all_first_media_with_session_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
    mut search: MediaListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_media_with_session_by_start_datetime_gte_and_start_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    add_join_fields(&mut select);
    select.order_by_expr(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).min(),
        Order::Asc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

pub fn select_all_last_media_with_session_with_search_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
    mut search: MediaListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = select_all_media_with_session_by_start_datetime_gte_and_start_datetime_lte(
        user_id,
        start_datetime,
        end_datetime,
    );

    add_join_fields(&mut select);
    select.order_by_expr(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).max(),
        Order::Desc,
    );

    // Ignore sort, might conflict with date ordering
    search.sort = None;
    apply_search(select, search)
}

fn add_join_fields(select: &mut SelectStatement) {
    select
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)),
            Alias::new(SESSION_START_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)),
            Alias::new(SESSION_END_DATE_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::DeviceId)),
            Alias::new(SESSION_DEVICE_ID_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::GroupId)),
            Alias::new(SESSION_GROUP_ID_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::Started)),
            Alias::new(SESSION_STARTED_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::FinishedStatus)),
            Alias::new(SESSION_FINISHED_STATUS_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::AddedDatetime)),
            Alias::new(SESSION_ADDED_DATETIME_ALIAS),
        )
        .expr_as(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::UpdatedDatetime)),
            Alias::new(SESSION_UPDATED_DATETIME_ALIAS),
        )
        .expr_as(derived_time_expr(), Alias::new(QUERY_TIME_ALIAS));
}

pub fn select_all_medias_order_by_start_datetime_desc(user_id: &Uuid) -> SelectStatement {
    let mut select = media_query::select_all(user_id);

    join_media_session(&mut select);
    order_by_start_datetime_desc(&mut select);

    select
}

pub fn select_all_medias_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> SelectStatement {
    let mut select = select_all_medias_order_by_start_datetime_desc(user_id);

    where_start_datetime_gte_and_start_datetime_lte(&mut select, start_datetime, end_datetime);

    select
}

pub fn select_all_medias_session_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
    user_id: &Uuid,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut select =
        select_all_medias_by_start_datetime_gte_and_start_datetime_lte_order_by_start_datetime_desc(
            user_id,
            start_datetime,
            end_datetime,
        );

    add_join_fields(&mut select);

    select
}

pub fn insert(media_session: &MediaSession) -> impl QueryStatementWriter {
    let mut insert = Query::insert();

    insert
        .into_table(MediaSessionIden::Table)
        .columns([
            MediaSessionIden::UserId,
            MediaSessionIden::MediaId,
            MediaSessionIden::StartDate,
            MediaSessionIden::EndDate,
            MediaSessionIden::DeviceId,
            MediaSessionIden::GroupId,
            MediaSessionIden::Started,
            MediaSessionIden::FinishedStatus,
            MediaSessionIden::AddedDatetime,
            MediaSessionIden::UpdatedDatetime,
        ])
        .values_panic([
            crate::uuid_utils::to_string(&media_session.user_id).into(),
            crate::uuid_utils::to_string(&media_session.media_id).into(),
            media_session.start_date.into(),
            media_session.end_date.into(),
            media_session
                .device_id
                .map(|id| crate::uuid_utils::to_string(&id))
                .into(),
            crate::uuid_utils::to_string(&media_session.group_id).into(),
            media_session.started.into(),
            media_session.finished_status.into(),
            media_session.added_datetime.into(),
            media_session.updated_datetime.into(),
        ]);

    insert
}

pub fn delete_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    start_datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut delete = Query::delete();

    delete
        .from_table(MediaSessionIden::Table)
        .and_where(Expr::col(MediaSessionIden::UserId).eq(crate::uuid_utils::to_string(user_id)))
        .and_where(Expr::col(MediaSessionIden::MediaId).eq(crate::uuid_utils::to_string(media_id)))
        .and_where(Expr::col(MediaSessionIden::StartDate).eq(start_datetime));

    delete
}

pub fn exists_by_id(
    user_id: &Uuid,
    media_id: &Uuid,
    start_datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id_and_media_id(&mut select, user_id, media_id);
    select
        .column((MediaSessionIden::Table, MediaSessionIden::MediaId))
        .and_where(Expr::col(MediaSessionIden::StartDate).eq(start_datetime));

    select
}

pub fn exists_by_start_datetime_lt_or_end_datetime_gt(
    user_id: &Uuid,
    end_datetime: DateTime<Utc>,
    start_datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
    select
        .column((MediaSessionIden::Table, MediaSessionIden::MediaId))
        .and_where(Expr::col(MediaSessionIden::StartDate).lt(end_datetime))
        .and_where(Expr::col(MediaSessionIden::EndDate).gt(start_datetime));

    select
}

fn join_media_session(select: &mut SelectStatement) {
    select.left_join(
        MediaSessionIden::Table,
        Expr::col((MediaIden::Table, MediaIden::Id))
            .equals((MediaSessionIden::Table, MediaSessionIden::MediaId)),
    );
}

fn from_and_where_user_id_and_media_id(
    select: &mut SelectStatement,
    user_id: &Uuid,
    media_id: &Uuid,
) {
    from_and_where_user_id(select, user_id);
    select.and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId))
            .eq(crate::uuid_utils::to_string(media_id)),
    );
}

fn from_and_where_user_id(select: &mut SelectStatement, user_id: &Uuid) {
    select.from(MediaSessionIden::Table).and_where(
        Expr::col((MediaSessionIden::Table, MediaSessionIden::UserId))
            .eq(crate::uuid_utils::to_string(user_id)),
    );
}

fn where_optional_start_datetime_gte_and_start_datetime_lte(
    select: &mut SelectStatement,
    start_datetime: Option<DateTime<Utc>>,
    end_datetime: Option<DateTime<Utc>>,
) {
    if let Some(start) = start_datetime {
        select.and_where(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).gte(start),
        );
    }

    if let Some(end) = end_datetime {
        select
            .and_where(Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).lte(end));
    }
}

fn where_start_datetime_gte_and_start_datetime_lte(
    select: &mut SelectStatement,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) {
    select
        .and_where(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).gte(start_datetime),
        )
        .and_where(
            Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).lte(end_datetime),
        );
}

fn order_by_start_datetime_desc(select: &mut SelectStatement) {
    select.order_by(
        (MediaSessionIden::Table, MediaSessionIden::StartDate),
        Order::Desc,
    );
}

fn add_fields(select: &mut SelectStatement) {
    select
        .column((MediaSessionIden::Table, MediaSessionIden::UserId))
        .column((MediaSessionIden::Table, MediaSessionIden::MediaId))
        .column((MediaSessionIden::Table, MediaSessionIden::StartDate))
        .column((MediaSessionIden::Table, MediaSessionIden::EndDate))
        .column((MediaSessionIden::Table, MediaSessionIden::DeviceId))
        .column((MediaSessionIden::Table, MediaSessionIden::GroupId))
        .column((MediaSessionIden::Table, MediaSessionIden::Started))
        .column((MediaSessionIden::Table, MediaSessionIden::FinishedStatus))
        .column((MediaSessionIden::Table, MediaSessionIden::AddedDatetime))
        .column((MediaSessionIden::Table, MediaSessionIden::UpdatedDatetime))
        .expr_as(derived_time_expr(), Alias::new(QUERY_TIME_ALIAS));
}

fn coalesce_time_sum() -> FunctionCall {
    Func::coalesce([
        Expr::expr(derived_time_expr()).sum(),
        Expr::val("0 seconds").into(),
    ])
}

fn derived_time_expr() -> SimpleExpr {
    Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).sub(Expr::col((
        MediaSessionIden::Table,
        MediaSessionIden::StartDate,
    )))
}
