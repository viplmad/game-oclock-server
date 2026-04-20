use chrono::{DateTime, Utc};
use sea_query::{
    Alias, Expr, ExprTrait, Func, Order, Query, QueryStatementWriter, SelectStatement, SimpleExpr,
};
use uuid::Uuid;

use crate::entities::{
    AggregateGroupQuery, AggregateQuery, MediaIden, MediaListSearch, MediaSession,
    MediaSessionIden, QUERY_TIME_ALIAS, SESSION_ADDED_DATETIME_ALIAS, SESSION_DEVICE_ID_ALIAS,
    SESSION_END_DATE_ALIAS, SESSION_FINISHED_STATUS_ALIAS, SESSION_GROUP_ID_ALIAS,
    SESSION_MEDIA_ID_ALIAS, SESSION_START_DATE_ALIAS, SESSION_STARTED_ALIAS,
    SESSION_UPDATED_DATETIME_ALIAS, STREAK_DAYS_ALIAS, STREAK_END_DATE_ALIAS,
    STREAK_START_DATE_ALIAS, SearchQuery, SessionAggregateGroupSearch, SessionAggregateSearch,
    SessionListSearch,
};
use crate::errors::SearchErrors;

use super::media_query;
use super::search::{apply_aggregate_group_search, apply_aggregate_search, apply_search};

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
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
            r#"SELECT CAST(DATE_PART('month', "MediaSession"."start_date") AS BIGINT), COUNT(DISTINCT "MediaSession"."media_id") FROM "MediaSession" WHERE "MediaSession"."user_id" = '00000000-0000-0000-0000-000000000000' GROUP BY CAST(DATE_PART('month', "MediaSession"."start_date") AS BIGINT)"#
        );
    }

    #[test]
    fn test() {
        let user_id = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        let query = select_streaks(
            &user_id,
            Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
        );
        assert_eq!(query.to_string(PostgresQueryBuilder), r#""#);
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

pub fn select_all_by_user_id(
    user_id: &Uuid,
    search: SessionListSearch,
) -> Result<SearchQuery, SearchErrors> {
    let mut select = Query::select();

    from_and_where_user_id(&mut select, user_id);
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
            Expr::col((MediaSessionIden::Table, MediaSessionIden::MediaId)),
            Alias::new(SESSION_MEDIA_ID_ALIAS),
        )
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

fn derived_time_expr() -> SimpleExpr {
    Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).sub(Expr::col((
        MediaSessionIden::Table,
        MediaSessionIden::StartDate,
    )))
}

/// Streaks
const STREAK_START_DATE_SUB_ALIAS: &str = "start_date";
const STREAK_END_DATE_SUB_ALIAS: &str = "end_date";
const STREAK_STARTS_STREAK_SUB_ALIAS: &str = "starts_streak";
const STREAK_GROUP_SUB_ALIAS: &str = "grp";

pub fn select_streaks(
    user_id: &Uuid,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> impl QueryStatementWriter {
    let mut select = Query::select();

    let min_start_date = Expr::col(STREAK_START_DATE_SUB_ALIAS).min();
    let max_end_date = Expr::col(STREAK_END_DATE_SUB_ALIAS).max();
    let streak_days = max_end_date
        .clone()
        .sub(min_start_date.clone())
        .add(Expr::val(1));
    select
        .expr_as(min_start_date.clone(), STREAK_START_DATE_ALIAS)
        .expr_as(max_end_date.clone(), STREAK_END_DATE_ALIAS)
        .expr_as(streak_days.clone(), STREAK_DAYS_ALIAS);
    select.from_subquery(
        streaks_group(user_id, start_datetime, end_datetime),
        "streaks_group_sub",
    );
    select.add_group_by([Expr::col(STREAK_GROUP_SUB_ALIAS).into()]);
    select.order_by_expr(streak_days.clone().into(), Order::Desc);

    select
}

fn streaks_group(
    user_id: &Uuid,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> SelectStatement {
    let mut select = Query::select();

    select
        .column(STREAK_START_DATE_SUB_ALIAS)
        .column(STREAK_END_DATE_SUB_ALIAS)
        .expr_as(
            filter_over_order_by(
                Expr::col(sea_query::Asterisk).count(),
                Expr::col(STREAK_STARTS_STREAK_SUB_ALIAS).into(),
                Expr::col(STREAK_START_DATE_SUB_ALIAS).into(),
            ),
            STREAK_GROUP_SUB_ALIAS,
        );
    select.from_subquery(
        starts_streak(user_id, start_datetime, end_datetime),
        "starts_streak_sub",
    );
    select.order_by(STREAK_START_DATE_SUB_ALIAS, Order::Asc);

    select
}

fn starts_streak(
    user_id: &Uuid,
    start_datetime: DateTime<Utc>,
    end_datetime: DateTime<Utc>,
) -> SelectStatement {
    let mut select = Query::select();

    let start_date_as_date =
        Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).cast_as("DATE");
    let end_date_as_date =
        Expr::col((MediaSessionIden::Table, MediaSessionIden::EndDate)).cast_as("DATE");
    select
        .expr_as(start_date_as_date.clone(), STREAK_START_DATE_SUB_ALIAS)
        .expr_as(end_date_as_date.clone(), STREAK_END_DATE_SUB_ALIAS)
        .expr_as(
            Func::coalesce([
                start_date_as_date.clone().sub(over_order_by(
                    Func::cust(Lag).arg(end_date_as_date.clone()).into(),
                    Expr::col((MediaSessionIden::Table, MediaSessionIden::StartDate)).into(),
                )),
                // 99 > 1 for first lag so it always starts streak
                Expr::val(99).into(),
            ])
            .gt(Expr::val(1)),
            STREAK_STARTS_STREAK_SUB_ALIAS,
        );
    from_and_where_user_id(&mut select, user_id);
    select
        .and_where(Expr::col(MediaSessionIden::StartDate).gte(start_datetime))
        .and_where(Expr::col(MediaSessionIden::EndDate).lt(end_datetime));
    select.order_by(
        (MediaSessionIden::Table, MediaSessionIden::StartDate),
        Order::Asc,
    );

    select
}

fn over_order_by(from: SimpleExpr, order_by: SimpleExpr) -> SimpleExpr {
    Expr::cust_with_exprs("$1 OVER (ORDER BY $2)", vec![from, order_by])
}

fn filter_over_order_by(from: SimpleExpr, filter: SimpleExpr, order_by: SimpleExpr) -> SimpleExpr {
    over_order_by(
        Expr::cust_with_exprs("$1 FILTER (WHERE $2)", vec![from, filter]),
        order_by,
    )
}

//
struct Lag;

impl sea_query::Iden for Lag {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "LAG").unwrap();
    }
}
